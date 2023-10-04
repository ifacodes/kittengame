use crate::{
    types::{
        shader::{load_shader, Shader},
        Color,
    },
    InternalData,
};
use anyhow::Result;
use glam::UVec2;
use pollster::FutureExt;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use wgpu::{
    Adapter, Device, Features, InstanceDescriptor, Queue, RenderPassDescriptor, Surface,
    SurfaceConfiguration,
};
use window::WindowTrait;

/// This struct contains a WGPU device, queue, and surface.
#[derive(Debug)]
pub struct Internal {
    device: Device,
    adapter: Adapter,
    queue: Queue,
    surface: Surface,
    config: SurfaceConfiguration,
    default_shader: Shader,
    default_pipeline: wgpu::RenderPipeline,
}

impl Internal {
    pub fn new<W>(window: &W) -> Result<Internal>
    where
        W: WindowTrait,
    {
        let backends = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
        let dx12_shader_compiler = wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default();
        let gles_minor_version = wgpu::Gles3MinorVersion::Automatic;

        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends,
            dx12_shader_compiler,
            gles_minor_version,
        });

        log::info!("initializing the surface...");

        let surface = unsafe { instance.create_surface(window) }?;
        let (adapter, device, queue) = async {
            let adapter =
                wgpu::util::initialize_adapter_from_env_or_default(&instance, Some(&surface))
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

        let size = window.size()?;

        let mut config = surface
            .get_default_config(&adapter, size.x, size.y)
            .expect("surface isn't supported by the adapter.");
        let surface_view_format = config.format.add_srgb_suffix();
        config.view_formats.push(surface_view_format);
        surface.configure(&device, &config);

        let default_shader = load_shader(
            &device,
            include_str!("../../shaders/fullscreen_triangle.wgsl"),
        )?;

        let default_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&default_shader.pipeline_layout),
            vertex: wgpu::VertexState {
                module: &default_shader.module,
                entry_point: "vert_main",
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &default_shader.module,
                entry_point: "frag_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            multiview: None,
        });

        Ok(Self {
            device,
            adapter,
            queue,
            surface,
            config,
            default_shader,
            default_pipeline,
        })
    }

    pub fn render(&mut self, data: &InternalData, window_size: UVec2) -> Result<()> {
        self.config.height = window_size.y;
        self.config.width = window_size.x;
        self.surface.configure(&self.device, &self.config);
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&Default::default());

        let clear_color: Color = [196, 99, 246, 255].into();

        let mut encoder = self.device.create_command_encoder(&Default::default());
        // per pipeline
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_color.into()),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.default_pipeline);
            render_pass.set_viewport(
                0.0,
                0.0,
                window_size.x as f32,
                window_size.y as f32,
                0.0,
                1.0,
            );

            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

impl Internal {
    fn _render<S: Into<(u32, u32)>>(&mut self, data: (), window_size: S) -> Result<()> {
        /*  for each entry of pass data:

            we set the viewport
            render_pass.set_viewport(0, 0, window_size.x, window_size.y, 0., 1.);

            we get the clear colour
            let clear_colour = wgpu::LoadOp::Clear(
                clear_color.into()
            );

            we get or create a new render_pipeline from the cache using the shader and other variables

            we set all the uniform buffer

            set the scissor rect
            render_pass.set_scissor(scissor_rect);

            set vertex buffer



        */

        Ok(())
    }
}
