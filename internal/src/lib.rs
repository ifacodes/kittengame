pub mod data;
pub mod internal;
mod renderpass;
pub mod types;
use anyhow::Result;
pub use data::InternalData;
use glam::{UVec2, Vec2};
pub use internal::Internal;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use types::Color;

impl Internal {
    fn render_size(&self) -> UVec2 {
        todo!()
    }
    fn draw_rect<P: Into<Vec2>, S: Into<Vec2>, C: Into<Color>>(
        &self,
        position: P,
        size: S,
        color: C,
    ) {
        let position = position.into();
        let size = size.into();
        let color = color.into();
        // create vertices?
    }
}

pub trait Game {}
