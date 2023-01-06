use anyhow::Result;
use pollster::FutureExt;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use wgpu::{
    Backends, Device, Features, Limits, PowerPreference, Queue, RequestAdapterOptions, Surface,
};
pub struct Internal {
    device: Device,
    queue: Queue,
    surface: Surface,
}

impl Internal {
    pub fn new<W>(window: &W) -> Result<Internal>
    where
        W: HasRawWindowHandle + HasRawDisplayHandle,
    {
        let instance = wgpu::Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(window) }?;
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .block_on()
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("kittengpu"),
                    features: Features::POLYGON_MODE_LINE | Features::POLYGON_MODE_POINT,
                    limits: Limits::downlevel_defaults(),
                },
                None,
            )
            .block_on()
            .unwrap();
        Ok(Self {
            device,
            queue,
            surface,
        })
    }
}
