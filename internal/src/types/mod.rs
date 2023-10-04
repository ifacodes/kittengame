//! internal types used in kittengpu.

use bytemuck::{Pod, Zeroable};
pub mod framebuffer;
pub mod pipeline;
pub mod shader;
pub mod texture;
pub mod uniform;
pub mod vertex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Topology {
    Points,
    Lines,
    Triangles,
}

impl From<Topology> for wgpu::PrimitiveTopology {
    fn from(val: Topology) -> Self {
        match val {
            Topology::Points => wgpu::PrimitiveTopology::PointList,
            Topology::Lines => wgpu::PrimitiveTopology::LineList,
            Topology::Triangles => wgpu::PrimitiveTopology::TriangleList,
        }
    }
}
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Pod, Zeroable)]
pub struct Color([u8; 4]);

impl Color {
    pub const BLACK: Color = Color([0, 0, 0, 255]);
    pub const WHITE: Color = Color([255, 255, 255, 255]);
    pub const RED: Color = Color([255, 0, 0, 255]);
    pub const GREEN: Color = Color([0, 255, 0, 255]);
    pub const BLUE: Color = Color([0, 0, 255, 255]);
    pub const YELLOW: Color = Color([255, 255, 0, 255]);
    pub const CYAN: Color = Color([0, 255, 255, 255]);
    pub const MAGENTA: Color = Color([255, 0, 255, 255]);
}

impl<T> From<[T; 4]> for Color
where
    [u8; 4]: std::convert::From<[T; 4]>,
{
    fn from(value: [T; 4]) -> Self {
        let value: [u8; 4] = value.into();
        Self(value)
    }
}

impl From<wgpu::Color> for Color {
    fn from(value: wgpu::Color) -> Self {
        let value = [value.r, value.g, value.b, value.a];
        let value = value.map(|x| (x * 255.0) as u8);
        Self(value)
    }
}

impl From<Color> for wgpu::Color {
    fn from(value: Color) -> Self {
        let [r, g, b, a] = value.0.map(|x| (x as f64) / 255.);
        wgpu::Color { r, g, b, a }
    }
}

impl<T> From<image::Rgba<T>> for Color
where
    [u8; 4]: std::convert::From<[T; 4]>,
{
    fn from(value: image::Rgba<T>) -> Self {
        value.0.into()
    }
}

impl<T> From<Color> for image::Rgba<T>
where
    image::Rgba<T>: std::convert::From<[u8; 4]>,
{
    fn from(value: Color) -> Self {
        value.0.into()
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}
