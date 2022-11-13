const FONT: &str = "Unifont";

pub const BAR_HEIGHT: u32 = 20;

use penrose::{x::XConn, Color};
use penrose_ui::{
    bar::{
        widgets::{
            amixer_volume, battery_summary, current_date_and_time, wifi_network, ActiveWindowName,
            CurrentLayout, Workspaces,
        },
        Position, StatusBar,
    },
    core::TextStyle,
};

// Mostly the example dwm bar from the main repo but recreated here so it's easier to tinker
// with and add in debug widgets when needed.
pub fn status_bar<X: XConn>() -> penrose_ui::Result<StatusBar<X>> {
    let black = Color::from(0x000000);
    let white = Color::from(0xFFFFFF);
    let grey = Color::from(0xAAAAAA);
    let blue = Color::from(0x22CC77);

    let highlight: Color = blue;
    let empty_ws: Color = grey;

    let style = TextStyle {
        font: FONT.to_string(),
        point_size: 12,
        fg: white,
        bg: Some(black),
        padding: (2.0, 2.0),
    };

    let padded_style = TextStyle {
        padding: (4.0, 2.0),
        ..style.clone()
    };

    StatusBar::try_new(
        Position::Top,
        BAR_HEIGHT,
        style.bg.unwrap_or_else(|| black),
        &[&style.font],
        vec![
            Box::new(Workspaces::new(&style, highlight, empty_ws)),
            Box::new(CurrentLayout::new(&style)),
            // Box::new(penrose_bar::widgets::debug::StateSummary::new(style)),
            Box::new(ActiveWindowName::new(
                20,
                &TextStyle {
                    bg: Some(highlight),
                    padding: (6.0, 4.0),
                    ..style.clone()
                },
                true,
                false,
            )),
            Box::new(wifi_network(&padded_style)),
            Box::new(battery_summary("BAT0", &padded_style)),
            Box::new(amixer_volume("Master", &padded_style)),
            Box::new(current_date_and_time(&padded_style)),
        ],
    )
}
