use wgpu::{
    BlendState, Color, ColorTargetState, ColorWrites, Operations, RenderPassColorAttachment,
    TextureFormat, TextureView,
};

#[derive(Debug, Default)]
pub struct FrameBuffer {
    pub color_attachments: [Option<RenderAttachment>; 8],
    pub depth_stencil_attachment: Option<RenderAttachment>,
}

impl FrameBuffer {
    pub fn new(
        color_attachments: [Option<RenderAttachment>; 8],
        depth_stencil_attachment: Option<RenderAttachment>,
    ) -> Self {
        FrameBuffer {
            color_attachments,
            depth_stencil_attachment,
        }
    }

    pub fn color_target_states(
        &mut self,
        blends: &[Option<BlendState>],
    ) -> [Option<ColorTargetState>; 8] {
        let mut states: [Option<ColorTargetState>; 8] = Default::default();

        for (index, color_attachment) in self.color_attachments.iter().enumerate() {
            states[index] = color_attachment
                .as_ref()
                .map(|a| a.color_target_state(blends.get(index).and_then(|b| *b)));
        }

        states
    }

    pub fn color_attachments(
        &mut self,
        ops: [Operations<Color>; 8],
    ) -> [Option<RenderPassColorAttachment>; 8] {
        let mut color_attachments: [Option<RenderPassColorAttachment>; 8] = Default::default();

        for (index, color_attachment) in self.color_attachments.iter().enumerate() {
            if let Some(color_attachment) = color_attachment {
                color_attachments[index] = Some(RenderPassColorAttachment {
                    view: &color_attachment.view,
                    resolve_target: None,
                    ops: ops[index],
                })
            }
        }

        color_attachments
    }
}

#[derive(Debug)]
pub struct RenderAttachment {
    view: TextureView,
    pub format: TextureFormat,
    depth_stencil: bool,
}

impl RenderAttachment {
    pub fn color_target_state(&self, blend: Option<BlendState>) -> wgpu::ColorTargetState {
        ColorTargetState {
            format: self.format,
            blend,
            write_mask: ColorWrites::all(),
        }
    }
}
