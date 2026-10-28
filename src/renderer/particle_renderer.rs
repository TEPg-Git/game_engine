#[repr(C)]
#[derive(Clone,Copy,bytemuck::Pod,bytemuck::Zeroable)]
pub struct ParticleInstance{pub(crate) position:[f32;2],pub(crate) color:[f32;4]}
impl ParticleInstance{pub(crate) fn layout()->wgpu::VertexBufferLayout<'static>{const ATTRIBUTES:[wgpu::VertexAttribute;2]=wgpu::vertex_attr_array![2=>Float32x2,3=>Float32x4];wgpu::VertexBufferLayout{array_stride:std::mem::size_of::<ParticleInstance>() as wgpu::BufferAddress,step_mode:wgpu::VertexStepMode::Instance,attributes:&ATTRIBUTES}}pub(crate) fn from_position_color(position:[f32;2],color:[f32;4])->Self{Self{position,color}}}
