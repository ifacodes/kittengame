use anyhow::Result;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
pub trait KittenWindow: HasRawDisplayHandle + HasRawWindowHandle {
    type HandlesWindowLoop;

    fn new(title: &str) -> Result<Self>
    where
        Self: Sized;
}

pub trait HandlesWindowLoop {
    fn run<I, U, R, Q>(self, init_fn: I, update_fn: U, render_fn: R, quit_fn: Q) -> !
    where
        I: 'static + FnMut() -> Result<()>,
        U: 'static + FnMut() -> Result<()>,
        R: 'static + FnMut() -> Result<()>,
        Q: 'static + FnMut() -> Result<()>;
}
