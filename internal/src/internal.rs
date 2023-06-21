use anyhow::Result;
use cgmath::Vector2;
use pollster::FutureExt;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use wgpu::{
    Adapter, Device, Features, InstanceDescriptor, Queue, RenderPassDescriptor, Surface,
    SurfaceConfiguration,
};

/// This struct contains a WGPU device, queue, and surface.
#[derive(Debug)]
pub struct Internal {
    device: Device,
    adapter: Adapter,
    queue: Queue,
    surface: Surface,
    config: SurfaceConfiguration,
}

impl Internal {
    pub fn new<W>(window: &W) -> Result<Internal>
    where
        W: HasRawWindowHandle + HasRawDisplayHandle,
    {
        let backends = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
        let dx12_shader_compiler = wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default();

        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends,
            dx12_shader_compiler,
        });

        log::info!("initializing the surface...");

        let surface = unsafe { instance.create_surface(window) }?;
        let (adapter, device, queue) = async {
            let adapter = wgpu::util::initialize_adapter_from_env_or_default(
                &instance,
                backends,
                Some(&surface),
            )
            .await
            .unwrap();

            let adapter_info = adapter.get_info();
            println!("Using {} ({:?})", adapter_info.name, adapter_info.backend);

            let adapter_features = adapter.features();
            let optional_features = Features::POLYGON_MODE_LINE | Features::POLYGON_MODE_POINT;

            let adapter_limits = adapter.limits();

            let trace_dir = std::env::var("WGPU_TRACE");
            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: Some("kittengpu"),
                        features: adapter_features & optional_features,
                        limits: adapter_limits,
                    },
                    trace_dir.ok().as_ref().map(std::path::Path::new),
                )
                .await
                .expect("Unable to find a suitable GPU adapter!");
            (adapter, device, queue)
        }
        .block_on();

        let mut config = surface
            .get_default_config(&adapter, 1270, 720)
            .expect("surface isn't supported by the adapter.");
        let surface_view_format = config.format.add_srgb_suffix();
        config.view_formats.push(surface_view_format);
        surface.configure(&device, &config);

        Ok(Self {
            device,
            adapter,
            queue,
            surface,
            config,
        })
    }

    pub fn render(&mut self, window_size: Vector2<usize>) -> Result<()> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&Default::default());

        let mut encoder = self.device.create_command_encoder(&Default::default());
        // per pipeline
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            //render_pass.set_pipeline(pipeline);
            //render_pass.draw(vertices, instances);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
