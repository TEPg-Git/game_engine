use wgpu::util::DeviceExt;

use crate::graphics::Uniforms;

use std::sync::Arc;

use winit::window::Window;

pub struct Renderer {
    //WGPU
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    // UNIFORM
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
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
        // UNIFORM BUFFER
        // ====================================================

        let uniforms = Uniforms {
            position: [0.0, 0.0],
            _padding: [0.0, 0.0],
            color: [1.0, 0.0, 0.0, 1.0],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),

            contents: bytemuck::bytes_of(&uniforms),

            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        println!("Uniform buffer created!");

        // ====================================================
        // UNIFORM BIND GROUP
        // ====================================================

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),

                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,

                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,

                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,

                        has_dynamic_offset: false,

                        min_binding_size: None,
                    },

                    count: None,
                }],
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),

            layout: &uniform_bind_group_layout,

            entries: &[wgpu::BindGroupEntry {
                binding: 0,

                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        println!("Uniform bind group created!");

        // ====================================================
        // CREATE RENDERER
        // ====================================================

        Self {
            surface,
            device,
            queue,
            config,
            uniform_buffer,
            uniform_bind_group,
        }
    }
}
