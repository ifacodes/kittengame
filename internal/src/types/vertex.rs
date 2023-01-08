use bytemuck::{Pod, Zeroable};
use cgmath::Vector2;
use image::Rgba;
/// A struct representing vertex information.
#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
struct Vertex {
    pos: Vector2<f32>,
    tex_pos: Vector2<f32>,
    colour: Rgba<u8>,
}

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}

#[cfg(test)]
mod test {
    use cgmath::vec2;

    use super::*;

    #[test]
    fn test() {
        let v = &[Vertex {
            pos: vec2(1.0, 0.0),
            tex_pos: vec2(0.0, 2.0),
            colour: Rgba([0u8, 127u8, 255u8, 255u8]),
        }];
        let b: &[u8] = bytemuck::cast_slice(v);
        assert_eq!(
            b,
            &[0, 0, 128, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 127, 255, 255]
        )
    }
}