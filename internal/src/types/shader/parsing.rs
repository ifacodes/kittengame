use std::mem;

use anyhow::{anyhow, bail, ensure, Ok, Result};
use itertools::Itertools;
use naga::{Binding, FunctionArgument, TypeInner};
use serde::*;
use thiserror::Error;
use wgpu::BindGroupLayoutEntry;

use crate::types::vertex;

#[derive(Debug, Error)]
enum Error {
    #[error("invalid uniform type {0}")]
    InvalidUniform(String),
    #[error("invalid uniform bindgroup (expected 1, found {0})")]
    InvalidUniformBindGroup(u32),
}

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

pub fn get_stages_in_shader(module: &naga::Module) -> wgpu::ShaderStages {
    module
        .entry_points
        .iter()
        .map(|entry_point| entry_point.stage)
        .fold(wgpu::ShaderStages::empty(), |acc, stage| {
            acc | match stage {
                naga::ShaderStage::Vertex => wgpu::ShaderStages::VERTEX,
                naga::ShaderStage::Fragment => wgpu::ShaderStages::FRAGMENT,
                naga::ShaderStage::Compute => wgpu::ShaderStages::COMPUTE,
            }
        })
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
        .ok_or_else(|| anyhow!("function has no result!"))?;

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

fn validate_vertex_structure(module: &naga::Module) -> Result<()> {
    Ok(())
}

pub fn validate_uniforms(module: &naga::Module) -> Result<Vec<BindGroupLayoutEntry>> {
    let gv = &module.global_variables;
    let uniforms = gv
        .iter()
        .filter(|(_, gv)| gv.space == naga::AddressSpace::Uniform)
        .collect_vec();
    let entries: Result<Vec<BindGroupLayoutEntry>> = uniforms
        .into_iter()
        .map(|(_handle, uniform)| {
            let ty = wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            };

            match module.types[uniform.ty].inner {
                TypeInner::Matrix { .. } => {
                    let resource = uniform
                        .binding
                        .as_ref()
                        .ok_or(anyhow!("unable to get binding info!"))?;

                    ensure!(
                        resource.group == 1,
                        Error::InvalidUniformBindGroup(resource.group)
                    );

                    let entry = BindGroupLayoutEntry {
                        binding: resource.binding,
                        visibility: wgpu::ShaderStages::all(),
                        ty,
                        count: None,
                    };
                    Ok(entry)
                }
                _ => Err(anyhow!("Uniform type not yet supported!")),
            }
        })
        .collect();
    entries
}

#[cfg(test)]
mod test {
    use super::{get_stages_in_shader, query_attachments, validate_uniforms};

    const TEST_SHADER: &str = "
        struct Vertex {
            @location(0) pos: vec2<f32>,
            //@location(1) tex: vec2<f32>,
            @location(1) col: vec4<u32>
        }
        
        struct Fragment {
            @builtin(position) pos: vec4<f32>,
            //@location(0) tex: vec2<f32>
            @location(1) col: vec4<f32>
        }
        
        struct Output {
            @location(0) diffuse: vec4<f32>
        }
        
        @group(0) @binding(0)
        var texture: texture_2d<f32>;
        @group(0) @binding(1)
        var tex_sampler: sampler;
        @group(1) @binding(0)
        var<uniform> matrix: mat4x4<f32>;
        
        
        // super simple vertex shader
        @vertex
        fn vertex(vert: Vertex) -> Fragment {
            var frag: Fragment;
            //frag.tex = vert.tex;
            frag.pos = vec4<f32>(vert.pos, 1.0, 1.0);
            frag.col = vec4<f32>(vert.col);
            return frag;
        }

        @fragment
        fn fragment(frag: Fragment) -> Output {
            //return textureSample(texture, tex_sampler, frag.tex);
            var output: Output;
            output.diffuse = vec4<f32>(1.0, 1.0, 1.0, 1.0) * frag.col;
            return output;
        }
    ";

    #[test]
    fn attachment() {
        let module = naga::front::wgsl::parse_str(TEST_SHADER).unwrap();

        println!(
            "{}",
            serde_json::to_string_pretty(&module.global_variables).unwrap()
        );

        let fragment = module
            .entry_points
            .iter()
            .find(|&entry_point| entry_point.stage == naga::ShaderStage::Fragment)
            .unwrap();

        let fragment = &fragment.function.result.as_ref().unwrap();

        let result = &module.types[fragment.ty];
        let fragment = serde_json::to_string_pretty(fragment).unwrap();
        let result = serde_json::to_string_pretty(&result).unwrap();
        println!("result: {fragment}\ntype: {result}");

        assert_eq!(1, query_attachments(&module).unwrap())
    }

    #[test]
    fn stages() {
        let module = naga::front::wgsl::parse_str(TEST_SHADER).unwrap();
        let stages = get_stages_in_shader(&module);
        assert_eq!(stages, wgpu::ShaderStages::VERTEX_FRAGMENT)
    }

    #[test]
    fn uniforms() {
        let module = naga::front::wgsl::parse_str(TEST_SHADER).unwrap();

        let entries = validate_uniforms(&module).unwrap();
        println!("{entries:#?}");
    }
}
