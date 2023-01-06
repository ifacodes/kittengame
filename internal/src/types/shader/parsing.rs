use std::collections::HashMap;

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
    entries: HashMap<usize, Vec<wgpu::BindGroupLayoutEntry>>,
) -> Vec<wgpu::BindGroupLayout> {
    entries.iter().fold(vec![], |mut acc, (_, entries)| {
        acc.push(
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries,
            }),
        );
        acc
    })
}

pub fn generate_layout_entries(
    module: &naga::Module,
) -> Result<HashMap<usize, Vec<wgpu::BindGroupLayoutEntry>>> {
    module
        .global_variables
        .iter()
        .filter(|(_, b)| b.binding.is_some())
        .try_fold(
            HashMap::<usize, Vec<wgpu::BindGroupLayoutEntry>>::new(),
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
                acc.entry(binding.group.try_into()?).or_default().push(
                    wgpu::BindGroupLayoutEntry {
                        binding: binding.binding,
                        visibility: stages,
                        ty,
                        count: None,
                    },
                );
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
