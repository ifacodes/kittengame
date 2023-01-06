use cgmath::Vector2;
use wgpu::{Device, Queue};

// Internal texture type.
pub struct Texture {
    texture: wgpu::Texture,
    format: wgpu::TextureFormat,
    size: Vector2<usize>,
    render_target: bool,
}

// Internal mutable texture type.
pub struct TextureMut<'a> {
    texture: &'a Texture,
    device: &'a Device,
    queue: &'a Queue,
}

impl Texture {
    fn new_texture(
        device: &Device,
        size: Vector2<usize>,
        format: wgpu::TextureFormat,
        render_target: bool,
    ) -> Texture {
        let usage = render_target
            .then(|| wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
            .unwrap_or(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING);
        Texture {
            texture: device.create_texture(&wgpu::TextureDescriptor {
                size: wgpu::Extent3d {
                    width: size.x as u32,
                    height: size.y as u32,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage,
                label: None,
            }),
            format,
            size,
            render_target,
        }
    }
}

impl<'a> TextureMut<'a> {
    fn write_to_texture(&mut self) {
        todo!()
    }
}
