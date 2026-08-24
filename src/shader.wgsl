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

struct Uniforms {
    position: vec2<f32>,
};

@group(0)
@binding(0)
var<uniform> uniforms: Uniforms;

@group(1)
@binding(0)
var texture_data: texture_2d<f32>;

@group(1)
@binding(1)
var texture_sampler: sampler;

@vertex
fn vs_main(
    input: VertexInput
) -> VertexOutput {

    var output: VertexOutput;

    output.position = vec4<f32>(
        input.position.x + uniforms.position.x,
        input.position.y + uniforms.position.y,
        0.0,
        1.0
    );

    output.tex_coords = input.tex_coords;

    return output;
}

@fragment
fn fs_main(
    input: VertexOutput
) -> @location(0) vec4<f32> {

    return textureSample(
        texture_data,
        texture_sampler,
        input.tex_coords
    );
}
