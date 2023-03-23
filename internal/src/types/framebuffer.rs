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
            if let Some(color_attachment) = color_attachment {
                states[index] = Some(ColorTargetState {
                    format: color_attachment.format,
                    blend: blends.get(index).and_then(|b| *b), // Possibly replace with a way to pass in the blend modes?
                    write_mask: ColorWrites::all(),
                })
            }
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
    format: TextureFormat,
    depth_stencil: bool,
}
