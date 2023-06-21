use anyhow::{anyhow, bail, Result};

pub fn generate_pipeline_layout(
    device: &wgpu::Device,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts,
        push_constant_ranges: &[],
    })
}

pub fn generate_bind_group_layouts(
    device: &wgpu::Device,
    entries: [Vec<wgpu::BindGroupLayoutEntry>; 4],
) -> [wgpu::BindGroupLayout; 4] {
    entries.map(|layout_entries| {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &layout_entries,
        })
    })
}

pub fn get_entrypoint_name<'a>(
    module: &'a naga::Module,
    ty: &'a naga::ShaderStage,
) -> Option<&'a str> {
    module
        .entry_points
        .iter()
        .find(|stage| ty == &stage.stage)
        .map(|entry| entry.name.as_str())
}

pub fn query_attachments(module: &naga::Module) -> Result<usize> {
    let function = &module
        .entry_points
        .iter()
        .find(|entry_point| entry_point.stage == naga::ShaderStage::Fragment)
        .ok_or_else(|| anyhow!("No fragment shader stage!"))?
        .function;

    let result = function
        .result
        .as_ref()
        .ok_or_else(|| anyhow!("No fragment output!"))?;

    // test if output is to a location
    if let Some(binding) = &result.binding {
        match binding {
            naga::Binding::Location { .. } => Ok(1),
            naga::Binding::BuiltIn(..) => bail!("BuiltIn not supported."),
        }
    } else {
        // result is a structure, look up the type
        match &module.types[result.ty].inner {
            naga::TypeInner::Struct { members, .. } => Ok(members.len()),
            _ => bail!("Strange output type encountered."),
        }
    }
}

pub fn generate_layout_entries(
    module: &naga::Module,
) -> Result<[Vec<wgpu::BindGroupLayoutEntry>; 4]> {
    module
        .global_variables
        .iter()
        .filter(|(_, b)| b.binding.is_some())
        .try_fold(
            [vec![], vec![], vec![], vec![]],
            |mut acc, (global_handle, var)| {
                let stages = module.entry_points.iter().fold(
                    wgpu::ShaderStages::NONE,
                    |stages, entry_point| match entry_point.function.expressions.iter().find_map(
                        |(_, x)| match x {
                            naga::Expression::GlobalVariable(handle) => {
                                if *handle == global_handle {
                                    Some(entry_point.stage)
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        },
                    ) {
                        Some(s) => match s {
                            naga::ShaderStage::Vertex => stages | wgpu::ShaderStages::VERTEX,
                            naga::ShaderStage::Fragment => stages | wgpu::ShaderStages::FRAGMENT,
                            naga::ShaderStage::Compute => stages | wgpu::ShaderStages::COMPUTE,
                        },
                        None => stages,
                    },
                );
                let binding = var
                    .binding
                    .as_ref()
                    .ok_or_else(|| anyhow!("unable to get resource binding."))?;
                let ty =
                    map_naga_inner_type_to_wgpu_binding_type(module.types.get_handle(var.ty)?)?;
                if let Some(vec) = acc.get_mut(binding.group as usize) {
                    vec.push(wgpu::BindGroupLayoutEntry {
                        binding: binding.binding,
                        visibility: stages,
                        ty,
                        count: None,
                    });
                }
                Ok(acc)
            },
        )
}

fn map_naga_inner_type_to_wgpu_binding_type(ty: &naga::Type) -> Result<wgpu::BindingType> {
    match ty.inner {
        naga::TypeInner::Image {
            dim,
            arrayed,
            class,
        } => match class {
            naga::ImageClass::Sampled {
                kind: naga::ScalarKind::Float,
                multi,
            } => match dim {
                naga::ImageDimension::D1 if !arrayed => Ok(wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D1,
                    multisampled: multi,
                }),
                naga::ImageDimension::D2 if !arrayed => Ok(wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: multi,
                }),
                naga::ImageDimension::D3 if !arrayed => Ok(wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D3,
                    multisampled: multi,
                }),
                naga::ImageDimension::Cube if !arrayed => Ok(wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::Cube,
                    multisampled: multi,
                }),
                naga::ImageDimension::D2 if arrayed => Ok(wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2Array,
                    multisampled: multi,
                }),
                naga::ImageDimension::Cube if arrayed => Ok(wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::CubeArray,
                    multisampled: multi,
                }),
                _ => bail!("not a supported texture type"),
            },
            naga::ImageClass::Depth { .. } => bail!("not implemented"),
            naga::ImageClass::Storage { .. } => bail!("not implemented"),
            _ => bail!("scalarkind not supported"),
        },
        naga::TypeInner::Sampler { comparison } if comparison => Ok(wgpu::BindingType::Sampler(
            wgpu::SamplerBindingType::Comparison,
        )),
        naga::TypeInner::Sampler { comparison } if !comparison => Ok(wgpu::BindingType::Sampler(
            wgpu::SamplerBindingType::Filtering,
        )),
        _ => bail!("not a valid type for a binding resource"),
    }
}

#[cfg(test)]
mod test {
    use super::query_attachments;

    #[test]
    fn attachment() {
        let module =
            naga::front::wgsl::parse_str(include_str!("../../../../shaders/main.wgsl")).unwrap();
        assert_eq!(1, query_attachments(&module).unwrap())
    }
}
