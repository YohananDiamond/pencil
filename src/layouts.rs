use penrose::client::Client;
use penrose::data_types::{Region, ResizeAction, WinId};
use penrose::layout::client_breakdown;

pub fn really_cool_layout(
    clients: &[&Client],
    focused: Option<WinId>,
    monitor_region: &Region,
    _: u32,
    _: f32,
) -> Vec<ResizeAction> {
    if let Some(fid) = focused {
        let (mx, my, mw, mh) = monitor_region.values();
        clients
            .iter()
            .map(|c| {
                let cid = c.id();
                if cid == fid {
                    (cid, Region::new(mx, my, mw, mh))
                } else {
                    (cid, Region::new(mx, my, 0, 0))
                }
            })
            .collect()
    } else {
        Vec::new()
    }
}
