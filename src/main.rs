#[macro_use]
extern crate penrose;

use penrose::client::Client;
use penrose::hooks::Hook;
use penrose::layout::{monocle, bottom_stack, side_stack, Layout, LayoutConf};
use penrose::{Backward, Config, Forward, Less, More, WindowManager, XcbConnection};

use penrose::contrib::extensions::Scratchpad;
use penrose::contrib::hooks::{DefaultWorkspace, LayoutSymbolAsRootName};
use penrose::contrib::layouts::paper;

use simplelog::{LevelFilter, SimpleLogger};

mod layouts;
mod misc;

use penrose::draw::*;
use std::{thread, time};

fn main() -> penrose::Result<()> {
    // penrose will log useful information about the current state of the WindowManager during
    // normal operation that can be used to drive scripts and related programs. Additional debug
    // output can be helpful if you are hitting issues.
    SimpleLogger::init(LevelFilter::Debug, simplelog::Config::default()).unwrap();

    // Config structs can be intiialised directly as all fields are public.
    // A default config is provided which sets sensible (but minimal) values for each field.
    let mut config = Config::default();

    // Created at startup. See keybindings below for how to access them
    config.workspaces = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    // Windows with a matching WM_CLASS will always float
    config.floating_classes = &["dmenu", "dunst", "polybar", "sxiv"];

    // When specifying a layout, most of the time you will want LayoutConf::default() as shown
    // below, which will honour gap settings and will not be run on focus changes (only when
    // clients are added/removed). To customise when/how each layout is applied you can create a
    // LayoutConf instance with your desired properties enabled.
    let follow_focus_conf = LayoutConf {
        floating: false,
        gapless: true,
        follow_focus: true,
        allow_wrapping: true,
    };

    // Defauly number of clients in the main layout area
    let n_main = 1;

    // Default percentage of the screen to fill with the main area of the layout
    let ratio = 0.6;

    // Layouts to be used on each workspace. Currently all workspaces have the same set of Layouts
    // available to them, though they track modifications to n_main and ratio independently.
    config.layouts = vec![
        Layout::new("[side]", LayoutConf::default(), side_stack, n_main, ratio),
        Layout::new("[botm]", LayoutConf::default(), bottom_stack, n_main, ratio),
        Layout::new("[mono]", follow_focus_conf, monocle, n_main, ratio),
        // Layout::new("[papr]", follow_focus_conf, paper, n_main, ratio),
        // Layout::floating("[----]"),
    ];

    // Gaps
    config.gap_px = 0;

    // Here we are using a contrib hook that requires configuration to set up a default workspace
    // on workspace "9". This will set the layout and spawn the supplied programs if we make
    // workspace "9" active while it has no clients.
    // config.hooks.push(DefaultWorkspace::new(
    //     "9",
    //     "[botm]",
    //     vec![my_terminal, my_terminal, my_file_manager],
    // ));

    // Scratchpad is an extension: it makes use of the same Hook points as the examples above but
    // addtionally provides a 'toggle' method that can be bound to a key combination in order to
    // trigger the bound scratchpad client.
    let sp = Scratchpad::new("st", 0.8, 0.8);
    sp.register(&mut config);

    /* The gen_keybindings macro parses user friendly key binding definitions into X keycodes and
     * modifier masks. It uses the 'xmodmap' program to determine your current keymap and create
     * the bindings dynamically on startup. If this feels a little too magical then you can
     * alternatively construct a  HashMap<KeyCode, FireAndForget> manually with your chosen
     * keybindings (see helpers.rs and data_types.rs for details).
     * FireAndForget functions do not need to make use of the mutable WindowManager reference they
     * are passed if it is not required: the run_external macro ignores the WindowManager itself
     * and instead spawns a new child process.
     */
    let key_bindings = gen_keybindings! {
        // client management
        "M-j" => run_internal!(cycle_client, Forward),
        "M-k" => run_internal!(cycle_client, Backward),
        "M-S-j" => run_internal!(drag_client, Forward),
        "M-S-k" => run_internal!(drag_client, Backward),
        "M-q" => run_internal!(kill_client),
        "M-s" => sp.toggle(),

        // workspace management
        "M-m" => run_internal!(toggle_workspace),
        "M-bracketright" => run_internal!(cycle_screen, Forward),
        "M-bracketleft" => run_internal!(cycle_screen, Backward),
        "M-S-bracketright" => run_internal!(drag_workspace, Forward),
        "M-S-bracketleft" => run_internal!(drag_workspace, Backward),

        // Layout management
        "M-Tab" => run_internal!(cycle_layout, Forward),
        "M-S-Tab" => run_internal!(cycle_layout, Backward),
        "M-A-Up" => run_internal!(update_max_main, More),
        "M-A-Down" => run_internal!(update_max_main, Less),
        "M-l" => run_internal!(update_main_ratio, More),
        "M-h" => run_internal!(update_main_ratio, Less),

        "M-A-s" => run_internal!(detect_screens),

        "M-C-S-e" => run_internal!(exit);

        // Each keybinding here will be templated in with the workspace index of each workspace,
        // allowing for common workspace actions to be bound at once.
        forall_workspaces: config.workspaces => {
            "M-{}" => focus_workspace,
            "M-S-{}" => client_to_workspace,
        }
    };

    // The underlying connection to the X server is handled as a trait: XConn. XcbConnection is the
    // reference implementation of this trait that uses the XCB library to communicate with the X
    // server. You are free to provide your own implementation if you wish, see xconnection.rs for
    // details of the required methods and expected behaviour.
    let conn = XcbConnection::new().unwrap();

    // Create the WindowManager instance with the config we have built and a connection to the X
    // server. Before calling grab_keys_and_run, it is possible to run additional start-up actions
    // such as configuring initial WindowManager state, running custom code / hooks or spawning
    // external processes such as a start-up script.
    let mut wm = WindowManager::init(config, &conn);

    // Start the bar
    thread::spawn(start_bar);

    // grab_keys_and_run will start listening to events from the X server and drop into the main
    // event loop. From this point on, program control passes to the WindowManager so make sure
    // that any logic you wish to run is done before here!
    wm.grab_keys_and_run(key_bindings);

    Ok(())
}

const HEIGHT: usize = 18;

const BLACK: u32 = 0x282828;
const GREY: u32 = 0x3c3836;
const WHITE: u32 = 0xebdbb2;
const PURPLE: u32 = 0xb16286;
const BLUE: u32 = 0x458588;
const RED: u32 = 0xcc241d;

fn start_bar() -> penrose::Result<()> {
    let workspaces = &["1", "2", "3", "4", "5", "6"];
    let style = TextStyle {
        font: "JetBrains Mono Medium".to_string(),
        point_size: 11,
        fg: WHITE.into(),
        bg: Some(BLACK.into()),
        padding: (2.0, 2.0),
    };
    let highlight = BLUE;
    let empty_ws = GREY;
    let mut bar = dwm_bar(
        Box::new(XCBDraw::new()?),
        0,
        HEIGHT,
        &style,
        highlight,
        empty_ws,
        workspaces,
    )?;

    let config = Config::default();
    let conn = XcbConnection::new().unwrap();
    let mut wm = WindowManager::init(config, &conn);

    thread::sleep(time::Duration::from_millis(1000));
    for focused in 1..6 {
        bar.workspace_change(&mut wm, focused - 1, focused);
        bar.event_handled(&mut wm);
        thread::sleep(time::Duration::from_millis(1000));
    }

    // lmao
    thread::sleep(time::Duration::from_millis(99999999999999999));

    Ok(())
}
