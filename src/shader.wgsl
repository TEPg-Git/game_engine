struct Uniforms {
    position: vec2<f32>,
    color: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0)
    position: vec2<f32>,

    @location(1)
    tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position)
    position: vec4<f32>,

    @location(0)
    tex_coords: vec2<f32>,
};

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    output.position = vec4<f32>(
        vertex.position + uniforms.position,
        0.0,
        1.0
    );

    output.tex_coords = vertex.tex_coords;

    return output;
}

@group(1) @binding(0)
var text_texture: texture_2d<f32>;

@group(1) @binding(1)
var text_sampler: sampler;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let texture_color = textureSample(
        text_texture,
        text_sampler,
        vertex.tex_coords
    );

    return vec4<f32>(
        uniforms.color.rgb,
        texture_color.a
    );
}
