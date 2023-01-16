use wgpu::TextureFormat;
use wgpu::TextureView;

/// A collection of Color Attachments.
#[derive(Debug)]
pub struct RenderAttachments {
    attachments: [Option<Attachment>; Self::MAXCOLORATTACHMENTS],
}

/// Colour Attachment
#[derive(Debug)]
pub struct Attachment {
    view: TextureView,
    pub format: TextureFormat,
}

impl RenderAttachments {
    pub const MAXCOLORATTACHMENTS: usize = 8;

    pub fn get(&self, index: usize) -> Option<&Option<Attachment>> {
        self.attachments.get(index)
    }
}
