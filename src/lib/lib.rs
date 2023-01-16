use anyhow::Result;
use internal::{Internal, InternalData};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

#[derive(Debug)]
struct KittenGpu {
    internal: Internal,
    data: InternalData,
}

impl KittenGpu {
    pub fn new<W>(window: &W) -> Result<Self>
    where
        W: HasRawWindowHandle + HasRawDisplayHandle,
    {
        let internal = Internal::new(window)?;
        let data = InternalData::default();
        Ok(Self { internal, data })
    }
}
