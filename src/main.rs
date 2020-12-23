#![allow(unused_imports)]

#[macro_use]
extern crate penrose;

use std::collections::HashMap;

use simplelog::SimpleLogger;

use penrose::client::Client;
use penrose::contrib::{
    extensions::Scratchpad,
    hooks::{DefaultWorkspace, LayoutSymbolAsRootName},
    layouts::paper,
};
use penrose::hooks::Hook;
use penrose::layout::{bottom_stack, monocle, side_stack, Layout, LayoutConf};
use penrose::{Backward, Config, Forward, Less, More, Selector, WindowManager, XcbConnection};
use penrose::helpers::index_selectors;

mod misc;

fn main() -> penrose::Result<()> {
    SimpleLogger::init(simplelog::LevelFilter::Debug, simplelog::Config::default()).unwrap();

    let mut config = Config::default();
    config.workspaces = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    config.focused_border = xcolor!("penrose.highlight", "#883300");

    // Windows with a matching WM_CLASS will always float
    config.floating_classes = &["dmenu", "dunst", "polybar", "sxiv"];

    let follow_focus_conf = LayoutConf {
        floating: false,
        gapless: true,
        follow_focus: true,
        allow_wrapping: true,
    };

    let n_main = 1; // Default number of clients in the main layout area
    let ratio = 0.6; // Default percentage of the screen to fill with the main area of the layout

    // Layouts to be used on each workspace. Currently all workspaces have the same set of Layouts
    // available to them, though they track modifications to n_main and ratio independently.
    config.layouts = vec![
        Layout::new("[side]", LayoutConf::default(), side_stack, n_main, ratio),
        Layout::new("[botm]", LayoutConf::default(), bottom_stack, n_main, ratio),
        Layout::new("[mono]", follow_focus_conf, monocle, n_main, ratio),
        // Layout::new("[papr]", follow_focus_conf, paper, n_main, ratio),
        // Layout::floating("[----]"),
    ];

    config.gap_px = 0;

    let sp = Scratchpad::new("st", 0.8, 0.8);
    sp.register(&mut config);

    let keybindings = gen_keybindings! {
        // Client management
        "M-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-q" => run_internal!(kill_client);
        "M-s" => sp.toggle();

        // Workspace management
        "M-m" => run_internal!(toggle_workspace);
        "M-bracketright" => run_internal!(cycle_screen, Forward);
        "M-bracketleft" => run_internal!(cycle_screen, Backward);
        "M-S-bracketright" => run_internal!(drag_workspace, Forward);
        "M-S-bracketleft" => run_internal!(drag_workspace, Backward);

        // Layout management
        "M-Tab" => run_internal!(cycle_layout, Forward);
        "M-S-Tab" => run_internal!(cycle_layout, Backward);
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-l" => run_internal!(update_main_ratio, More);
        "M-h" => run_internal!(update_main_ratio, Less);

        "M-A-s" => run_internal!(detect_screens);

        "M-C-S-e" => run_internal!(exit);

        refmap [ config.ws_range() ] in {
            "M-{}" => focus_workspace [ index_selectors(config.workspaces.len()) ];
            "M-S-{}" => client_to_workspace [ index_selectors(config.workspaces.len()) ];
        };
    };

    // Create the WindowManager instance with the config we have built and a connection to the X
    // server. Before calling grab_keys_and_run, it is possible to run additional start-up actions
    // such as configuring initial WindowManager state, running custom code / hooks or spawning
    // external processes such as a start-up script.
    let conn = XcbConnection::new().unwrap();
    let mut wm = WindowManager::init(config, &conn);

    // grab_keys_and_run will start listening to events from the X server and drop into the main
    // event loop. From this point on, program control passes to the WindowManager so make sure
    // that any logic you wish to run is done before here!
    wm.grab_keys_and_run(keybindings, HashMap::new());

    Ok(())
}
