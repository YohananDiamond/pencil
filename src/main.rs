use penrose::{
    builtin::{
        actions::{exit, log_current_state, modify_with, send_layout_message},
        layout::{
            messages::{ExpandMain, IncMain, ShrinkMain},
            transformers::{Gaps, ReserveTop},
            MainAndStack, Monocle,
        },
    },
    core::{
        bindings::{parse_keybindings_with_xmodmap, KeyEventHandler},
        Config, WindowManager,
    },
    extensions::hooks::add_ewmh_hooks,
    map, stack,
    x11rb::RustConn,
    Color, Result,
};

use tracing_subscriber::{self, prelude::*};

use std::collections::HashMap;
use std::convert::TryFrom;
use std::env::var;
use std::path::PathBuf;

mod bar;

mod startup;
use startup::SpawnOnStartup;

// TODO: pub mod misc;

// The default number of clients in the main layout area
const N_MAIN: u32 = 1;

// The default percentage of the screen to fill the main area of the layout
const RATIO: f32 = 0.6;

const WORKSPACES: &[&str] = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];

const FLOATING_CLASSES: &[&str] = &["float", "dmenu", "dunst", "polybar", "sxiv"];

fn main() -> Result<()> {
    let home = PathBuf::from(var("HOME").unwrap());
    let config_home = match var("XDG_CONFIG_HOME") {
        Ok(s) => PathBuf::from(s),
        Err(_) => {
            let mut p = home.clone();
            p.push(".config");
            p
        }
    };

    let profile_dir = {
        let mut p = config_home.clone();
        p.push("xorg");
        p.push("xprofile");
        p
    };

    let tracing_builder = tracing_subscriber::fmt()
        .json() // JSON logs
        .flatten_event(true)
        .with_env_filter("info")
        .with_filter_reloading();
    tracing_builder.finish().init();

    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;
    // TODO: draw windows etc.
    let mouse_bindings = HashMap::new();

    let layouts = stack!(
        MainAndStack::side(N_MAIN, RATIO, 0.1),
        MainAndStack::bottom(N_MAIN, RATIO, 0.1),
        Monocle::boxed()
    )
    .map(|l| ReserveTop::wrap(l, bar::BAR_HEIGHT))
    .map(|l| Gaps::wrap(l, 1, 1));

    // TODO: let focused_border_color: u32 = xcolor!("pencilwm.highlight", "#883300");

    let config = add_ewmh_hooks(Config {
        tags: WORKSPACES.iter().map(|s| s.to_string()).collect(),
        floating_classes: FLOATING_CLASSES.iter().map(|s| s.to_string()).collect(),
        focused_border: Color::try_from("#000000").expect("Failed to build color"),
        // TODO: focused_border: Color::from(focused_border_color).as_rgb_hex_string()
        default_layouts: layouts,
        focus_follow_mouse: true,
        startup_hook: Some(SpawnOnStartup::boxed(
            profile_dir.to_string_lossy().into_owned(),
        )),
        ..Config::default()
    });

    let conn = RustConn::new()?;
    let wm = WindowManager::new(config, key_bindings, mouse_bindings, conn)?;

    let bar = bar::status_bar().unwrap();
    let wm = bar.add_to(wm);

    wm.run()
}

pub fn raw_key_bindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
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

        // Debugging
        // "M-A-t" => set_tracing_filter(handle),
        "M-S-s" => log_current_state(),

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
