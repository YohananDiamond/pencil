//! Startup hooks for direct adding to your penrose config.
use penrose::{
    core::{hooks::StateHook, State},
    util::spawn,
    x::XConn,
    Result,
};

use std::rc::Rc;

/// Spawn a client program on window manager startup
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpawnOnStartup {
    prog: Rc<String>,
}

impl SpawnOnStartup {
    /// Create a new startup hook ready for adding to your Config
    pub fn boxed<X>(prog: String) -> Box<dyn StateHook<X>>
    where
        X: XConn,
    {
        Box::new(Self { prog: Rc::new(prog) })
    }
}

impl<X> StateHook<X> for SpawnOnStartup
where
    X: XConn,
{
    fn call(&mut self, _state: &mut State<X>, _x: &X) -> Result<()> {
        spawn(self.prog.as_str().to_owned())
    }
}
