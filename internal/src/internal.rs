use anyhow::Result;
use cgmath::Vector2;
use pollster::FutureExt;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use wgpu::{
    Backends, Device, Features, Limits, PowerPreference, Queue, RenderPassDescriptor,
    RequestAdapterOptions, Surface,
};

#[derive(Debug)]
pub struct Internal {
    device: Device,
    queue: Queue,
    surface: Surface,
}

impl Internal {
    pub fn new<W>(window: &W) -> Result<Internal>
    where
        W: HasRawWindowHandle + HasRawDisplayHandle,
    {
        let instance = wgpu::Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(window) }?;
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .block_on()
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("kittengpu"),
                    features: Features::POLYGON_MODE_LINE | Features::POLYGON_MODE_POINT,
                    limits: Limits::downlevel_defaults(),
                },
                None,
            )
            .block_on()
            .unwrap();
        Ok(Self {
            device,
            queue,
            surface,
        })
    }

    pub fn render(&mut self, window_size: Vector2<usize>) -> Result<()> {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        // per pipeline
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[],
                depth_stencil_attachment: None,
            });

            //render_pass.set_pipeline(pipeline);
            //render_pass.draw(vertices, instances);
        }

        Ok(())
    }
}
