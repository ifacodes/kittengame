use anyhow::{anyhow, bail, Result};

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

pub fn query_types(module: &naga::Module) {
    module.types.iter().for_each(|(_handle, ty)| {
        if let Some(name) = &ty.name {
            println!("Struct {name}");
            if let naga::TypeInner::Struct { members, .. } = &ty.inner {
                members.iter().for_each(|member| {
                    if let Some(name) = &member.name {
                        println!(
                            "\t{name}: {}",
                            type_string(&module.types[member.ty].inner)
                                .unwrap_or("unable to parse type")
                        );
                        if let Some(binding) = &member.binding {
                            match binding {
                                naga::Binding::Location { location, .. } => {
                                    println!("\tbinding: {location}")
                                }
                                naga::Binding::BuiltIn(builtin) => {
                                    println!("\tbuiltin: {builtin:?}")
                                }
                            }
                        }
                    }
                })
            }
        } else {
            println!(
                "{}",
                type_string(&ty.inner).unwrap_or("unable to parse type")
            );
        }
    })
}

fn type_string(inner: &naga::TypeInner) -> Option<&str> {
    match inner {
        naga::TypeInner::Vector { size, kind, .. } => match (size, kind) {
            (naga::VectorSize::Bi, naga::ScalarKind::Float) => Some("vec2<f32>"),
            (naga::VectorSize::Tri, naga::ScalarKind::Float) => Some("vec3<f32>"),
            (naga::VectorSize::Quad, naga::ScalarKind::Float) => Some("vec4<f32>"),
            (naga::VectorSize::Bi, naga::ScalarKind::Uint) => Some("vec2<u32>"),
            (naga::VectorSize::Tri, naga::ScalarKind::Uint) => Some("vec3<u32>"),
            (naga::VectorSize::Quad, naga::ScalarKind::Uint) => Some("vec4<u32>"),
            _ => unimplemented!(),
        },
        naga::TypeInner::Image {
            dim,
            arrayed,
            class,
        } => match class {
            naga::ImageClass::Sampled {
                kind: naga::ScalarKind::Float,
                ..
            } => match dim {
                naga::ImageDimension::D2 => {
                    if !arrayed {
                        Some("2D Texture, Sampled Float")
                    } else {
                        None
                    }
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        },
        naga::TypeInner::Sampler { comparison } => {
            if !comparison {
                Some("Sampler, comparison: false")
            } else {
                Some("Sampler, comparison: true")
            }
        }
        _ => None,
    }
}
