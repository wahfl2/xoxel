use std::sync::Arc;

use pollster::FutureExt as _;
use wgpu::{DeviceDescriptor, RequestAdapterOptions};
use winit::window::Window;

pub struct Backend {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface<'static>,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl Backend {
    pub fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }).block_on().expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter.request_device(
            &DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default()
                    .using_resolution(adapter.limits()),
            },
            None,
        ).block_on().expect("Failed to create device");

        Self { instance, surface, adapter, device, queue }
    }
}