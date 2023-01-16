use super::render_attachments::RenderAttachments;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct PipelineRequirements {
    pub primitive: wgpu::PrimitiveState,
    pub targets: [Option<wgpu::ColorTargetState>; RenderAttachments::MAXCOLORATTACHMENTS], // maxColorAttachments is 8 as per the spec.
}
