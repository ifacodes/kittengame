use super::framebuffer::FrameBuffer;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct PipelineRequirements {
    pub primitive: wgpu::PrimitiveState,
    pub targets: [Option<wgpu::ColorTargetState>; FrameBuffer::MAXCOLORATTACHMENTS], // maxColorAttachments is 8 as per the spec.
}
