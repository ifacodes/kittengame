use cgmath::Vector2;
use wgpu::{Device, Queue};

// Internal texture type.
#[derive(Debug)]
pub struct Texture {
    texture: wgpu::Texture,
    pub format: wgpu::TextureFormat,
    pub size: Vector2<usize>,
}

impl Texture {
    pub fn get_view(&self) -> wgpu::TextureView {
        self.texture.create_view(&Default::default())
    }
}

// Internal mutable reference to a texture.
#[derive(Debug)]
pub struct TextureMut<'a> {
    texture: &'a mut Texture,
    device: &'a Device,
    queue: &'a Queue,
}

impl<'a> TextureMut<'a> {
    fn write_to_texture(&mut self) {
        todo!()
    }
}

/// create a new wgpu texture
pub fn new_wgpu_texture(
    device: &Device,
    size: Vector2<usize>,
    format: wgpu::TextureFormat,
    render_target: bool,
) -> wgpu::Texture {
    let usage = render_target
        .then(|| wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
        .unwrap_or(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING);

    device.create_texture(&wgpu::TextureDescriptor {
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
        view_formats: &[],
    })
}

pub fn new_depth_texture(device: &Device, size: Vector2<usize>) -> wgpu::Texture {
    new_wgpu_texture(device, size, wgpu::TextureFormat::Depth32Float, true)
}
