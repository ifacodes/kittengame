pub mod data;
pub mod internal;
pub mod types;

use anyhow::Result;
use cgmath::mint::Vector2;
pub use data::InternalData;
pub use internal::Internal;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

pub trait WindowTrait: HasRawDisplayHandle + HasRawWindowHandle {
    fn size(&self) -> Result<Vector2<usize>>;
    fn set_size(&mut self, size: (u32, u32));
    fn resizable(&self) -> bool;
    fn set_resizable(&mut self, resizable: bool);
}

impl Internal {
    fn render_size(&self) -> Vector2<u32> {
        todo!()
    }

    fn set_clear_colour(&mut self) {
        todo!()
    }

    fn begin_draw(&mut self) {
        todo!()
    }
    fn end_draw(&mut self) {
        todo!()
    }
}
