// ============================================================
// UNIFORMS
// ============================================================

struct Uniforms {
    position_rotation: vec4<f32>,
    scale: vec4<f32>,
    color: vec4<f32>,
};

struct CameraUniforms {
    position: vec2<f32>,
    zoom: f32,
    padding: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(0) @binding(1)
var<uniform> camera: CameraUniforms;
// ============================================================
// VERTEX INPUT
// ============================================================

struct VertexInput {
    @location(0)
    position: vec2<f32>,

    @location(1)
    tex_coords: vec2<f32>,
};


// ============================================================
// VERTEX OUTPUT
// ============================================================

struct VertexOutput {
    @builtin(position)
    position: vec4<f32>,

    @location(0)
    tex_coords: vec2<f32>,

    @location(1)
    color: vec4<f32>,
};


// ============================================================
// VERTEX SHADER
// ============================================================

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    var position = vertex.position;
    position *= uniforms.scale.xy;
    let rotation = uniforms.position_rotation.z;
    let cos_angle = cos(rotation);
    let sin_angle = sin(rotation);
    let rotated_position = vec2<f32>(
        position.x * cos_angle - position.y * sin_angle,
        position.x * sin_angle + position.y * cos_angle
    );
    position = rotated_position + uniforms.position_rotation.xy;
    position -= camera.position;
    position *= camera.zoom;
    output.position = vec4<f32>(position, 0.0, 1.0);
    output.tex_coords = vertex.tex_coords;
    output.color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    return output;
}

// Instanced particles use the same shared quad; each instance supplies its center.
@vertex
fn vs_particle(@location(0) position:vec2<f32>,@location(1) tex_coords:vec2<f32>,@location(2) particle_position:vec2<f32>,@location(3) particle_color:vec4<f32>)->VertexOutput{var output:VertexOutput;var world_position=position*uniforms.scale.xy+particle_position;world_position=(world_position-camera.position)*camera.zoom;output.position=vec4<f32>(world_position,0.0,1.0);output.tex_coords=tex_coords;output.color=particle_color;return output;}

@group(1) @binding(0)
var sprite_texture: texture_2d<f32>;

@group(1) @binding(1)
var sprite_sampler: sampler;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let texture_color = textureSample(sprite_texture, sprite_sampler, vertex.tex_coords);
    return texture_color * uniforms.color * vertex.color;
    }
