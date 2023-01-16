use anyhow::{anyhow, bail, Result};
use arena::{Arena, Key};
use std::{collections::HashMap, hash::Hash, rc::Rc};
use wgpu::{
    BlendState, Device, FragmentState, MultisampleState, PrimitiveState, RenderPipeline,
    RenderPipelineDescriptor, Sampler, VertexState,
};

use crate::types::{
    pipeline::{self, PipelineRequirements},
    render_attachments::RenderAttachments,
    shader::Shader,
    texture::Texture,
    vertex::Vertex,
};

#[derive(Debug, Default)]
pub struct InternalData {
    pipelines: HashMap<Key<Shader>, HashMap<PipelineRequirements, Rc<RenderPipeline>>>,
    shaders: Arena<Shader>,
    textures: Arena<Texture>,
    samplers: Arena<Sampler>,
}

impl InternalData {
    pub fn get_pipeline(
        &mut self,
        device: &Device,
        attachments: RenderAttachments,
        key: Key<Shader>,
        primitive: PrimitiveState,
        blend: &[Option<BlendState>; RenderAttachments::MAXCOLORATTACHMENTS],
    ) -> Result<Rc<RenderPipeline>> {
        let shader = self.shaders.get(key).ok_or_else(|| anyhow!(""))?;
        let mut targets: [Option<wgpu::ColorTargetState>; RenderAttachments::MAXCOLORATTACHMENTS] =
            Default::default();

        (0..RenderAttachments::MAXCOLORATTACHMENTS).for_each(|n| {
            targets[n] = attachments.get(n).and_then(|opt| {
                opt.as_ref().map(|attachment| wgpu::ColorTargetState {
                    format: attachment.format,
                    blend: blend[n],
                    write_mask: wgpu::ColorWrites::all(),
                })
            })
        });

        Ok(self
            .pipelines
            .entry(key)
            .or_default()
            .entry(PipelineRequirements {
                primitive,
                targets: [None, None, None, None, None, None, None, None],
            })
            .or_insert_with(|| {
                Rc::new(device.create_render_pipeline(&RenderPipelineDescriptor {
                    label: None,
                    layout: shader.pipeline_layout.as_ref(),
                    vertex: VertexState {
                        module: &shader.module,
                        entry_point: "vertex",
                        buffers: &[Vertex::VERTEXBUFFERLAYOUT],
                    },
                    primitive,
                    depth_stencil: None,
                    multisample: MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    fragment: Some(FragmentState {
                        module: &shader.module,
                        entry_point: "fragment",
                        targets: &targets,
                    }),
                    multiview: None,
                }))
            })
            .clone())
    }
}
