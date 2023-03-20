use anyhow::*;
use std::collections::HashMap;
use wgpu::{
    BindGroupLayoutEntry, BindingType, SamplerBindingType, ShaderStages, TextureViewDimension,
};

/// Generate a wgpu::PipelineLayout for the shader.
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
    // Iterate over the entries collection, create bind group layouts, and collect them into a vector.
    entries
        .values()
        .map(|entries| {
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries,
            })
        })
        .collect::<Vec<_>>()
}

pub fn generate_layout_entries(
    module: &naga::Module,
) -> Result<HashMap<usize, Vec<BindGroupLayoutEntry>>> {
    module
        .global_variables
        .iter()
        .filter_map(|(handle, var)| var.binding.as_ref().map(|binding| (handle, var, binding)))
        .try_fold(
            HashMap::<usize, Vec<BindGroupLayoutEntry>>::new(),
            |mut acc, (global_handle, var, binding)| {
                let stages = module
                    .entry_points
                    .iter()
                    .filter_map(|entry_point| {
                        entry_point
                            .function
                            .expressions
                            .iter()
                            .find_map(|(_, expr)| match expr {
                                naga::Expression::GlobalVariable(handle)
                                    if *handle == global_handle =>
                                {
                                    Some(entry_point.stage)
                                }
                                _ => None,
                            })
                    })
                    .fold(
                        ShaderStages::NONE,
                        |stages, shader_stage| match shader_stage {
                            naga::ShaderStage::Vertex => stages | ShaderStages::VERTEX,
                            naga::ShaderStage::Fragment => stages | ShaderStages::FRAGMENT,
                            naga::ShaderStage::Compute => stages | ShaderStages::COMPUTE,
                        },
                    );

                let ty = map_naga_inner_type_to_wgpu_binding_type(module.types.get_handle(var.ty)?)
                    .map_err(|e| {
                        anyhow!("Failed to map Naga inner type to WGPU binding type: {}", e)
                    })?;

                let group: usize = binding
                    .group
                    .try_into()
                    .map_err(|e| anyhow!("Failed to convert binding group to usize: {}", e))?;

                acc.entry(group).or_default().push(BindGroupLayoutEntry {
                    binding: binding.binding,
                    visibility: stages,
                    ty,
                    count: None,
                });

                Ok(acc)
            },
        )
}

fn vertex_attributes(ty: &naga::Type) -> Result<()> {
    match &ty.inner {
        naga::TypeInner::Struct { members, span } => unimplemented!(""),
        _ => bail!("not a struct!"),
    }
}

fn map_naga_inner_type_to_wgpu_binding_type(ty: &naga::Type) -> Result<BindingType> {
    match ty.inner {
        naga::TypeInner::Image {
            dim,
            arrayed,
            class,
        } => match class {
            naga::ImageClass::Sampled { kind, multi } => match kind {
                naga::ScalarKind::Float => Ok(wgpu::TextureSampleType::Float { filterable: true }),
                naga::ScalarKind::Uint => Ok(wgpu::TextureSampleType::Uint),
                naga::ScalarKind::Sint => Ok(wgpu::TextureSampleType::Sint),
                naga::ScalarKind::Bool => Ok(wgpu::TextureSampleType::Depth),
            }
            .and_then(|sample_type| match dim {
                naga::ImageDimension::D1 if !arrayed => Ok(BindingType::Texture {
                    sample_type,
                    view_dimension: TextureViewDimension::D1,
                    multisampled: multi,
                }),
                naga::ImageDimension::D2 if !arrayed => Ok(BindingType::Texture {
                    sample_type,
                    view_dimension: TextureViewDimension::D2,
                    multisampled: multi,
                }),
                naga::ImageDimension::D3 if !arrayed => Ok(BindingType::Texture {
                    sample_type,
                    view_dimension: TextureViewDimension::D3,
                    multisampled: multi,
                }),
                naga::ImageDimension::Cube if !arrayed => Ok(BindingType::Texture {
                    sample_type,
                    view_dimension: TextureViewDimension::Cube,
                    multisampled: multi,
                }),
                naga::ImageDimension::D2 if arrayed => Ok(BindingType::Texture {
                    sample_type,
                    view_dimension: TextureViewDimension::D2Array,
                    multisampled: multi,
                }),
                naga::ImageDimension::Cube if arrayed => Ok(BindingType::Texture {
                    sample_type,
                    view_dimension: TextureViewDimension::CubeArray,
                    multisampled: multi,
                }),
                _ => bail!("not a supported texture type"),
            }),
            naga::ImageClass::Depth { .. } => bail!("not implemented"),
            naga::ImageClass::Storage { .. } => bail!("not implemented"),
            _ => bail!("scalarkind not supported"),
        },
        naga::TypeInner::Sampler { comparison } if comparison => {
            Ok(BindingType::Sampler(SamplerBindingType::Comparison))
        }
        naga::TypeInner::Sampler { comparison } if !comparison => {
            Ok(BindingType::Sampler(SamplerBindingType::Filtering))
        }
        _ => bail!("not a valid type for a binding resource"),
    }
}
