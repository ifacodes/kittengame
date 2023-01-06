mod parsing;
use anyhow::Result;
use naga::back::wgsl::{write_string, WriterFlags};
use naga::valid::{Capabilities, ValidationFlags, Validator};

/// Internal shader type.
struct Shader {
    module: wgpu::ShaderModule,
    bind_group_layouts: Vec<wgpu::BindGroupLayout>,
    pipeline_layout: Option<wgpu::PipelineLayout>,
}

/// Loads shader in from file.
fn load_shader(device: &wgpu::Device, shader_path: &str) -> Result<Shader> {
    let module = naga::front::wgsl::parse_str(shader_path)?;
    let info = Validator::new(ValidationFlags::all(), Capabilities::all()).validate(&module)?;

    let bind_group_layouts =
        parsing::generate_bind_group_layouts(device, parsing::generate_layout_entries(&module)?);

    let pipeline_layout = Some(parsing::generate_pipeline_layout(
        device,
        &bind_group_layouts.iter().collect::<Vec<_>>(),
    ));

    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(write_string(&module, &info, WriterFlags::all())?.into()),
    });
    Ok(Shader {
        module,
        bind_group_layouts,
        pipeline_layout,
    })
}
