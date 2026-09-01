use std::sync::Arc;

use winit::window::Window;

pub struct Renderer {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Self {
        // ====================================================
        // WGPU INSTANCE
        // ====================================================

        let instance = wgpu::Instance::default();

        // ====================================================
        // SURFACE
        // ====================================================

        let surface = instance.create_surface(window.clone()).unwrap();

        // ====================================================
        // ADAPTER
        // ====================================================

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),

            force_fallback_adapter: false,

            compatible_surface: Some(&surface),

            apply_limit_buckets: false,
        }))
        .expect("Failed to find suitable GPU adapter");

        println!("GPU: {:?}", adapter.get_info());

        // ====================================================
        // DEVICE + QUEUE
        // ====================================================

        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("East Engine Device"),

            required_features: wgpu::Features::empty(),

            required_limits: wgpu::Limits::default(),

            experimental_features: wgpu::ExperimentalFeatures::disabled(),

            memory_hints: wgpu::MemoryHints::default(),

            trace: wgpu::Trace::Off,
        }))
        .expect("Failed to create GPU device");

        println!("Device created!");

        // ====================================================
        // SURFACE CONFIG
        // ====================================================

        let size = window.inner_size();

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .expect("Surface is not supported");

        surface.configure(&device, &config);

        println!("Surface configured!");

        // ====================================================
        // CREATE RENDERER
        // ====================================================

        Self {
            surface,
            device,
            queue,
            config,
        }
    }
}
