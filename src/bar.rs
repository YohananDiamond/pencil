const FONT: &'static str = "Unifont:pixelsize=16:antialias=true:autohint=true;";

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
    let BLACK = Color::from((1.0, 1.0, 1.0));
    let WHITE = Color::from((0.0, 0.0, 0.0));
    let GREY = Color::from((0.8, 0.8, 0.8));
    let BLUE = Color::from((0.2, 0.8, 0.4));

    let highlight: Color = BLUE;
    let empty_ws: Color = GREY;

    let style = TextStyle {
        font: FONT.to_string(),
        point_size: 8,
        fg: WHITE,
        bg: Some(BLACK),
        padding: (2.0, 2.0),
    };

    let padded_style = TextStyle {
        padding: (4.0, 2.0),
        ..style.clone()
    };

    StatusBar::try_new(
        Position::Top,
        30,
        style.bg.unwrap_or_else(|| 0x000000.into()),
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
            Box::new(battery_summary("BAT1", &padded_style)),
            Box::new(battery_summary("BAT0", &padded_style)),
            Box::new(amixer_volume("Master", &padded_style)),
            Box::new(current_date_and_time(&padded_style)),
        ],
    )
}
