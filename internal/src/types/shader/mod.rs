mod parsing;
use anyhow::Result;
use naga::back::wgsl::{write_string, WriterFlags};
use naga::valid::{Capabilities, ValidationFlags, Validator};

/// Internal shader type.
#[derive(Debug)]
pub struct Shader {
    pub module: wgpu::ShaderModule,
    pub bind_group_layouts: [wgpu::BindGroupLayout; 4],
    pub pipeline_layout: wgpu::PipelineLayout,
    pub attachments: usize,
}

/// Loads shader in from file.
pub fn load_shader(device: &wgpu::Device, source: &str) -> Result<Shader> {
    let module = naga::front::wgsl::parse_str(source)?;
    let info = Validator::new(ValidationFlags::all(), Capabilities::all()).validate(&module)?;

    let layout_entries = parsing::generate_layout_entries(&module)?;
    let (bind_group_layouts, pipeline_layout) = {
        if layout_entries.iter().all(|v| v.is_empty()) {
            (
                parsing::generate_bind_group_layouts(device, layout_entries),
                parsing::generate_pipeline_layout(device, &[]),
            )
        } else {
            let bind_group_layouts = parsing::generate_bind_group_layouts(device, layout_entries);
            let bind_group_refs = {
                let [ref a, ref b, ref c, ref d] = bind_group_layouts;
                &[a, b, c, d]
            };
            let pipeline_layout = parsing::generate_pipeline_layout(device, bind_group_refs);
            (bind_group_layouts, pipeline_layout)
        }
    };

    let attachments = parsing::query_attachments(&module)?;

    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(write_string(&module, &info, WriterFlags::all())?.into()),
    });
    Ok(Shader {
        module,
        bind_group_layouts,
        pipeline_layout,
        attachments,
    })
}

mod test {
    use pollster::FutureExt;
    use wgpu::{BindGroupLayoutDescriptor, Features, InstanceDescriptor};

    use super::*;

    #[test]
    fn serde() {
        let shader = include_str!("../../../../shaders/main.wgsl");
        let module = naga::front::wgsl::parse_str(shader).expect("unable to parse shader");
        let vertex_name =
            parsing::get_entrypoint_name(&module, &naga::ShaderStage::Vertex).unwrap();

        let fragment_name =
            parsing::get_entrypoint_name(&module, &naga::ShaderStage::Fragment).unwrap();

        println!("vertex_name: {vertex_name}\nfragment_name: {fragment_name}");
        println!(
            "{}",
            serde_json::to_string_pretty(&module.global_variables)
                .expect("unable to pretty print module")
        )
    }

    #[test]
    fn mm() {
        let backends = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
        let dx12_shader_compiler = wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default();
        let gles_minor_version = wgpu::Gles3MinorVersion::Automatic;

        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends,
            dx12_shader_compiler,
            gles_minor_version,
        });

        let (adapter, device, queue) = async {
            let adapter = wgpu::util::initialize_adapter_from_env_or_default(&instance, None)
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

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            }],
        });
        println!("{:?}", &bind_group_layout)
    }
}
