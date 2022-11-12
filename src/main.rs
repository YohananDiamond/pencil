use penrose::{
    builtin::{
        actions::{exit, modify_with, send_layout_message},
        layout::{
            messages::{ExpandMain, IncMain, ShrinkMain},
            MainAndStack, Monocle,
        },
    },
    core::{
        bindings::{parse_keybindings_with_xmodmap, KeyEventHandler},
        Config, WindowManager,
    },
    map, stack,
    x11rb::RustConn,
    Color, Result,
};

use std::convert::TryFrom;
use std::collections::HashMap;

mod bar;

// TODO: pub mod misc;

// The default number of clients in the main layout area
const N_MAIN: u32 = 1;

// The default percentage of the screen to fill the main area of the layout
const RATIO: f32 = 0.6;

// TODO: comptime
const WORKSPACES: &[&str] = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];

const FLOATING_CLASSES: &[&str] = &["float", "dmenu", "dunst", "polybar", "sxiv"];

fn main() -> Result<()> {
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;
    // TODO: draw windows etc.
    let mouse_bindings = HashMap::new();

    let layouts = stack!(
        MainAndStack::side(N_MAIN, RATIO, 0.2),
        MainAndStack::bottom(N_MAIN, RATIO, 0.2),
        Monocle::boxed()
    );
    // TODO: maybe: .map(|layout| ReserveTop::wrap(Gaps::wrap(layout, OUTER_PX, INNER_PX), BAR_HEIGHT_PX));

    // TODO: let focused_border_color: u32 = xcolor!("pencilwm.highlight", "#883300");

    let config = Config {
        tags: WORKSPACES.iter().map(|s| s.to_string()).collect(),
        floating_classes: FLOATING_CLASSES.iter().map(|s| s.to_string()).collect(),
        focused_border: Color::try_from("#000000").expect("Failed to build color"),
        // TODO: focused_border: Color::from(focused_border_color).as_rgb_hex_string()
        default_layouts: layouts,
        focus_follow_mouse: true,
        ..Config::default()
    };

    let conn = RustConn::new()?;
    let wm = WindowManager::new(config, key_bindings, mouse_bindings, conn)?;

    let bar = bar::status_bar().unwrap();
    let wm = bar.add_to(wm);

    wm.run()

}

fn raw_key_bindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut bindings = map! {
        map_keys: |k: &str| k.to_string();

        // Client
        "M-j" => modify_with(|cs| cs.focus_down()),
        "M-k" => modify_with(|cs| cs.focus_up()),
        "M-S-j" => modify_with(|cs| cs.swap_down()),
        "M-S-k" => modify_with(|cs| cs.swap_up()),
        "M-q" => modify_with(|cs| cs.kill_focused()),

        // Workspace
        "M-m" => modify_with(|cs| cs.toggle_tag()), // TODO: what is this??
        "M-Tab" => modify_with(|cs| cs.next_layout()),
        "M-S-Tab" => modify_with(|cs| cs.previous_layout()),
        "M-C-j" => send_layout_message(|| IncMain(1)),
        "M-C-k" => send_layout_message(|| IncMain(-1)),
        "M-l" => send_layout_message(|| ExpandMain),
        "M-h" => send_layout_message(|| ShrinkMain),

        // Screen
        "M-bracketright" => modify_with(|cs| cs.next_screen()),
        "M-bracketleft" => modify_with(|cs| cs.previous_screen()),

        // WM
        "M-C-S-e" => exit(),

        // TODO: "M-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        // TODO: "M-A-s" => run_internal!(detect_screens);
    };

    for tag in WORKSPACES {
        bindings.extend([
            (
                format!("M-{tag}"),
                modify_with(move |client_set| client_set.focus_tag(tag)),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }

    bindings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_parse_correctly_with_xmodmap() {
        let res = parse_keybindings_with_xmodmap(raw_key_bindings());

        if let Err(e) = res {
            panic!("{:?}", e);
        }
    }
}
