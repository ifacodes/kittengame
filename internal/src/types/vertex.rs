use anyhow::Result;
use bytemuck::{Pod, Zeroable};
use glam::Vec2;

use super::Color;

/// A struct representing vertex information.
#[repr(C)]
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Vertex {
    pos: Vec2,
    tex: Vec2,
    col: Color,
}

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}

impl Vertex {
    /// How the vertex buffer should be represented in memory.
    pub const BUFFER_LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Uint8x4],
    };

    pub fn new(pos: Vec2, tex: Vec2, col: impl Into<Color>) -> Self {
        let col = col.into();
        Self { pos, tex, col }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let v = &[Vertex {
            pos: Vec2::from((1., 0.)),
            tex: Vec2::from((0., 2.)),
            col: [0u8, 127u8, 255u8, 255u8].into(),
        }];
        let b: &[u8] = bytemuck::cast_slice(v);
        assert_eq!(
            b,
            &[0, 0, 128, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 127, 255, 255]
        )
        //assert_eq!(b, &[0, 0, 128, 63, 0, 0, 0, 0, 0, 127, 255, 255])
    }
}
