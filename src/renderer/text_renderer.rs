use wgpu::util::DeviceExt;

use crate::graphics::Uniforms;
use crate::renderer::{Renderer, TextObject};
use crate::text::{Text, create_text_bitmap_with_options, load_font};

impl Renderer {
        pub fn create_text_object(&mut self, id: u32, text: Text) {
            let font = load_font();
    
            let (rgba_data, text_width, text_height) = create_text_bitmap_with_options(
                font,
                &text.content,
                text.font_size,
                text.line_spacing,
                text.letter_spacing,
                text.max_width,
                text.alignment,
            );
    
            let text_texture = self.create_text_texture(&rgba_data, text_width, text_height);
            let text_view = text_texture.create_view(&wgpu::TextureViewDescriptor::default());
    
            let text_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Text Bind Group"),
                layout: &self.texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&text_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.text_sampler),
                    },
                ],
            });
    
            let (vertex_buffer, vertex_count) = self.create_text_vertex_buffer(text_width, text_height);
    
            let uniforms = Self::text_uniforms(&text);
    
            let uniform_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Text Uniform Buffer"),
                    contents: bytemuck::bytes_of(&uniforms),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });
    
            let uniform_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Text Uniform Bind Group"),
                layout: &self.uniform_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: self.camera_uniform_buffer.as_entire_binding(),
                    },
                ],
            });
    
            self.text_objects.insert(
                id,
                TextObject {
                    revision: text.revision(),
                    text,
                    uniform_buffer,
                    uniform_bind_group,
                    text_texture,
                    text_bind_group,
                    vertex_buffer,
                    vertex_count,
                },
            );
        }

    fn create_text_texture(
        &self,
        rgba_data: &[u8],
        text_width: u32,
        text_height: u32,
    ) -> wgpu::Texture {
        let unpadded_bytes_per_row = text_width * 4;
        
                let padded_bytes_per_row = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT
                    * unpadded_bytes_per_row.div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);
        
                let mut padded_data = vec![0u8; (padded_bytes_per_row * text_height) as usize];
        
                for y in 0..text_height {
                    let source_start = (y * unpadded_bytes_per_row) as usize;
                    let source_end = source_start + unpadded_bytes_per_row as usize;
                    let destination_start = (y * padded_bytes_per_row) as usize;
                    let destination_end = destination_start + unpadded_bytes_per_row as usize;
        
                    padded_data[destination_start..destination_end]
                        .copy_from_slice(&rgba_data[source_start..source_end]);
                }
        
                let texture = self.device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("Text Texture"),
                    size: wgpu::Extent3d {
                        width: text_width,
                        height: text_height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                });
        
                self.queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    &padded_data,
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(padded_bytes_per_row),
                        rows_per_image: Some(text_height),
                    },
                    wgpu::Extent3d {
                        width: text_width,
                        height: text_height,
                        depth_or_array_layers: 1,
                    },
                );
        
                texture
    }

    fn create_text_vertex_buffer(&self, text_width: u32, text_height: u32) -> (wgpu::Buffer, u32) {
        let screen_width = self.config.width.max(1) as f32;
                let screen_height = self.config.height.max(1) as f32;
        
                let text_width_ndc = (text_width as f32 / screen_width) * 2.0;
                let text_height_ndc = (text_height as f32 / screen_height) * 2.0;
        
                let half_width = text_width_ndc / 2.0;
                let half_height = text_height_ndc / 2.0;
        
                let vertices = Self::quad_vertices(half_width, half_height);
        
                let vertex_buffer = self
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Text Vertex Buffer"),
                        contents: bytemuck::cast_slice(&vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
        
                (vertex_buffer, vertices.len() as u32)
    }

    fn text_uniforms(text: &Text) -> Uniforms {
        Uniforms {
                    position_rotation: [text.position[0], text.position[1], text.rotation, 0.0],
                    scale: [text.scale[0], text.scale[1], 0.0, 0.0],
                    color: [text.color[0], text.color[1], text.color[2], text.opacity],
                }
    }

        pub fn update_text_object(&mut self, id: u32, text: &Text) {
            let needs_bitmap_update = self
                .text_objects
                .get(&id)
                .map(|object| object.revision != text.revision())
                .unwrap_or(true);
    
            if needs_bitmap_update {
                let font = load_font();
    
                let (rgba_data, text_width, text_height) = create_text_bitmap_with_options(
                    font,
                    &text.content,
                    text.font_size,
                    text.line_spacing,
                    text.letter_spacing,
                    text.max_width,
                    text.alignment,
                );
    
                let text_texture = self.create_text_texture(&rgba_data, text_width, text_height);
                let text_view = text_texture.create_view(&wgpu::TextureViewDescriptor::default());
    
                let text_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Text Bind Group"),
                    layout: &self.texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&text_view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&self.text_sampler),
                        },
                    ],
                });
    
                let (vertex_buffer, vertex_count) =
                    self.create_text_vertex_buffer(text_width, text_height);
    
                if let Some(object) = self.text_objects.get_mut(&id) {
                    object.text_texture = text_texture;
                    object.text_bind_group = text_bind_group;
                    object.vertex_buffer = vertex_buffer;
                    object.vertex_count = vertex_count;
                    object.revision = text.revision();
                }
            }
    
            if let Some(object) = self.text_objects.get_mut(&id) {
                let uniforms = Self::text_uniforms(text);
    
                self.queue
                    .write_buffer(&object.uniform_buffer, 0, bytemuck::bytes_of(&uniforms));
    
                object.text = text.clone();
            }
        }
}
