use wgpu::util::DeviceExt;

use crate::graphics::Vertex;
use crate::renderer::Renderer;
use crate::sprite::Sprite;

impl Renderer {
    pub(super) fn create_sprite_bind_group(&self, sprite: &Sprite) -> wgpu::BindGroup {
        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Sprite Bind Group"),
                    layout: &self.texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&sprite.texture.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&sprite.texture.sampler),
                        },
                    ],
                })
    }

    pub(super) fn create_sprite_vertex_buffer(&self, sprite: &Sprite) -> (wgpu::Buffer, u32) {
        let half_width = sprite.size[0] / 2.0;
                let half_height = sprite.size[1] / 2.0;
        
                let vertices = Self::quad_vertices(half_width, half_height);
        
                let vertex_buffer = self
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Sprite Vertex Buffer"),
                        contents: bytemuck::cast_slice(&vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
        
                (vertex_buffer, vertices.len() as u32)
    }

    pub(super) fn quad_vertices(half_width: f32, half_height: f32) -> [Vertex; 6] {
        [
                    Vertex {
                        position: [-half_width, half_height],
                        tex_coords: [0.0, 0.0],
                    },
                    Vertex {
                        position: [half_width, half_height],
                        tex_coords: [1.0, 0.0],
                    },
                    Vertex {
                        position: [-half_width, -half_height],
                        tex_coords: [0.0, 1.0],
                    },
                    Vertex {
                        position: [half_width, half_height],
                        tex_coords: [1.0, 0.0],
                    },
                    Vertex {
                        position: [half_width, -half_height],
                        tex_coords: [1.0, 1.0],
                    },
                    Vertex {
                        position: [-half_width, -half_height],
                        tex_coords: [0.0, 1.0],
                    },
                ]
    }
}
