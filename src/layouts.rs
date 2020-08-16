use penrose::client::Client;
use penrose::layout::client_breakdown;
use penrose::data_types::{WinId, Region, ResizeAction};

/**
 * A simple layout that places the main region on the left and tiles remaining
 * windows in a single column to the right.
 */
pub fn side_stack(
    clients: &[&Client],
    _: Option<WinId>,
    monitor_region: &Region,
    max_main: u32,
    ratio: f32,
) -> Vec<ResizeAction> {
    let (mx, my, mw, mh) = monitor_region.values();
    let (n_main, n_stack) = client_breakdown(&clients, max_main);
    let h_stack = if n_stack > 0 { mh / n_stack } else { 0 };
    let h_main = if n_main > 0 { mh / n_main } else { 0 };
    let split = if max_main > 0 {
        (mw as f32 * ratio) as u32
    } else {
        0
    };

    clients
        .iter()
        .enumerate()
        .map(|(n, c)| {
            let n = n as u32;
            if n < max_main {
                let w = if n_stack == 0 { mw } else { split };
                (c.id(), Region::new(mx, my + n * h_main, w, h_main))
            } else {
                let sn = n - max_main; // nth stacked client
                let region = Region::new(mx + split, my + sn * h_stack, mw - split, h_stack);
                (c.id(), region)
            }
        })
        .collect()
}
