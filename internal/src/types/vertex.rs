use bytemuck::{Pod, Zeroable};
use cgmath::Vector2;
use image::Rgba;

/// A struct representing vertex information.
#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vertex {
    pos: Vector2<f32>,
    //tex_pos: Vector2<f32>,
    color: Rgba<u8>,
}

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}

impl Vertex {
    /// How the vertex buffer should be represented in memory.
    pub const VERTEXBUFFERLAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        //attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Uint8x4],
        attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Uint8x4],
    };
}

#[cfg(test)]
mod test {
    use cgmath::vec2;

    use super::*;

    #[test]
    fn test() {
        let v = &[Vertex {
            pos: vec2(1.0, 0.0),
            //tex_pos: vec2(0.0, 2.0),
            color: Rgba([0u8, 127u8, 255u8, 255u8]),
        }];
        let b: &[u8] = bytemuck::cast_slice(v);
        //assert_eq!(b, &[0, 0, 128, 63, 0, 0, 0, 0, 0, 0, 0, 64, 0, 127, 255, 255])
        assert_eq!(b, &[0, 0, 128, 63, 0, 0, 0, 0, 0, 127, 255, 255])
    }
}
