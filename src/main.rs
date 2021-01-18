#[macro_use]
extern crate penrose;

use penrose::{
    contrib::extensions::scratchpad::Scratchpad,
    core::{
        config::Config,
        helpers::index_selectors,
        layout::{bottom_stack, monocle, side_stack, Layout, LayoutConf},
        ring::Selector,
        bindings::MouseEvent,
    },
    logging_error_handler,
    xcb::new_xcb_backed_window_manager,
    Backward, Forward, Less, More,
    WindowManager,
};

pub mod misc;

const FOLLOW_FOCUS_CONF: LayoutConf = LayoutConf {
    floating: false,
    gapless: true,
    follow_focus: true,
    allow_wrapping: true,
};

// The default number of clients in the main layout area
const N_MAIN: u32 = 1;

// The default percentage of the screen to fill the main area of the layout
const RATIO: f32 = 0.6;

fn main() -> penrose::Result<()> {
    // Environment variables
    let terminal = std::env::var("TERMINAL").unwrap_or_else(|_| "st".into());

    let terminal_sp = Scratchpad::new(&terminal, 0.7, 0.7);

    const WS_RANGE: std::ops::RangeInclusive<u8> = 1..=9;
    let workspaces: Vec<_> = WS_RANGE.map(|n| format!("{}", n)).collect();

    let key_bindings = gen_keybindings! {
        // Client management
        "M-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-q" => run_internal!(kill_client);
        "M-s" => terminal_sp.toggle();

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

        refmap [ WS_RANGE ] in {
            "M-{}" => focus_workspace [ index_selectors(10) ];
            "M-S-{}" => client_to_workspace [ index_selectors(10) ];
        };
    };

    let mouse_bindings = gen_mousebindings! {
        Press Right + [Meta] => |wm: &mut WindowManager<_>, _: &MouseEvent| wm.cycle_workspace(Forward),
        Press Left + [Meta] => |wm: &mut WindowManager<_>, _: &MouseEvent| wm.cycle_workspace(Backward)
    };

    let focused_border_color = xcolor!("pencilwm.highlight", "#883300");

    let config = Config::default()
        .builder()
        .floating_classes(["dmenu", "dunst", "polybar", "sxiv"].iter().cloned())
        .workspaces(workspaces)
        .focused_border(focused_border_color)
        .layouts(vec![
            Layout::new("[side]", LayoutConf::default(), side_stack, N_MAIN, RATIO),
            Layout::new("[botm]", LayoutConf::default(), bottom_stack, N_MAIN, RATIO),
            Layout::new("[mono]", FOLLOW_FOCUS_CONF, monocle, N_MAIN, RATIO),
            // Layout::new("[papr]", follow_focus_conf, paper, n_main, ratio),
            // Layout::floating("[----]"),
        ])
        .gap_px(0)
        .build()
        .expect("Failed to build config");

    let mut wm = new_xcb_backed_window_manager(
        config,
        vec![terminal_sp.get_hook()],
        logging_error_handler(),
    )?;

    wm.grab_keys_and_run(key_bindings, mouse_bindings)
}
