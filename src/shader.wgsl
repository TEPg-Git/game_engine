// ============================================================
// UNIFORMS
// ============================================================

struct Uniforms {
    position_rotation: vec4<f32>,
    scale: vec4<f32>,
    color: vec4<f32>,

    camera_position: vec2<f32>,
    camera_zoom: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

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
};


// ============================================================
// VERTEX SHADER
// ============================================================

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // ========================================================
    // LOCAL POSITION
    // ========================================================

    var position = vertex.position;

    // ========================================================
    // SCALE
    // ========================================================

    position *= uniforms.scale.xy;

    // ========================================================
    // ROTATION
    // ========================================================

    let rotation = uniforms.position_rotation.z;

    let cos_angle = cos(rotation);
    let sin_angle = sin(rotation);

    let rotated_position = vec2<f32>(
        position.x * cos_angle - position.y * sin_angle,
        position.x * sin_angle + position.y * cos_angle
    );

    // ========================================================
    // WORLD POSITION
    // ========================================================

    position = rotated_position + uniforms.position_rotation.xy;

    // ========================================================
    // CAMERA
    // ========================================================

    position -= uniforms.camera_position;

    position *= uniforms.camera_zoom.x;

    // ========================================================
    // OUTPUT
    // ========================================================

    output.position = vec4<f32>(
        position,
        0.0,
        1.0
    );

    output.tex_coords = vertex.tex_coords;

    return output;
}


// ============================================================
// TEXTURE
// ============================================================

@group(1) @binding(0)
var sprite_texture: texture_2d<f32>;

@group(1) @binding(1)
var sprite_sampler: sampler;


// ============================================================
// FRAGMENT SHADER
// ============================================================

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {

    let texture_color = textureSample(
        sprite_texture,
        sprite_sampler,
        vertex.tex_coords
    );

    return texture_color;
}
