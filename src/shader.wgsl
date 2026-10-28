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

    @location(2)
    speed: f32,
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
    output.speed = 0.0;
    return output;
}

// Instanced particles use the same shared quad; each instance supplies its center.
@vertex
fn vs_particle(
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) particle_position: vec2<f32>,
    @location(3) particle_color: vec4<f32>,
    @location(4) particle_velocity: vec2<f32>,
) -> VertexOutput {
    var output: VertexOutput;
    let speed = length(particle_velocity);
    let direction = select(
        vec2<f32>(1.0, 0.0),
        particle_velocity / max(speed, 0.00001),
        speed > 0.00001
    );
    let side = vec2<f32>(-direction.y, direction.x);
    // Keep trails visible even while particles are accelerating from rest.
    let speed_factor = clamp(speed / 0.18, 0.25, 1.0);
    let size_factor = clamp(uniforms.scale.x, 0.333, 6.667);
    let trail_length = mix(0.035, 0.130, speed_factor) * size_factor;
    let width = mix(0.0060, 0.0120, speed_factor) * size_factor;
    let along = (tex_coords.x - 1.0) * trail_length;
    let across = (tex_coords.y - 0.5) * width;
    var world_position = particle_position + direction * along + side * across;
    world_position -= camera.position;
    world_position *= camera.zoom;
    output.position = vec4<f32>(world_position, 0.0, 1.0);
    output.tex_coords = tex_coords;
    output.color = particle_color;
    output.speed = speed_factor;
    return output;
}

@group(1) @binding(0)
var sprite_texture: texture_2d<f32>;

@group(1) @binding(1)
var sprite_sampler: sampler;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let texture_color = textureSample(sprite_texture, sprite_sampler, vertex.tex_coords);
    return texture_color * uniforms.color * vertex.color;
    }


// Analytic particle glow: soft, speed-sensitive streak plus a compact hot core.
@fragment
fn fs_particle(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let uv = vertex.tex_coords;
    let transverse = (uv.y - 0.5) * 2.0;
    let tail_distance = max(0.0, 0.92 - uv.x);
    // Layered aurora trail: broad haze, defined filament, and a bright leading core.
    let tail_fade = exp(-tail_distance * mix(1.2, 2.6, vertex.speed));
    let soft_halo = exp(-transverse * transverse * 0.85);
    let narrow_streak = exp(-transverse * transverse * 5.5);
    let filament = exp(-transverse * transverse * 18.0);
    let trail = tail_fade
        * (soft_halo * 0.42 + narrow_streak * 0.72 + filament * 0.36)
        * mix(0.45, 0.90, vertex.speed);

    let head_x = (uv.x - 0.92) * 18.0;
    let head_halo = exp(-head_x * head_x * 0.12)
        * exp(-transverse * transverse * 1.2);
    let hot_core = exp(-head_x * head_x * 1.3)
        * exp(-transverse * transverse * 14.0);
    let rim = exp(-transverse * transverse * 2.8)
        * smoothstep(0.35, 0.95, abs(transverse));

    // A restrained cyan/ice highlight gives the stream a prismatic edge.
    let spectral_mix = clamp(rim * 0.20 + hot_core * 0.12, 0.0, 0.28);
    let spectral_color = mix(vertex.color.rgb, vec3<f32>(0.48, 0.88, 1.0), spectral_mix);
    let glow_alpha = clamp(trail + head_halo * 0.42 + hot_core * 0.95, 0.0, 1.0)
        * vertex.color.a;
    let emission = 0.72 + trail * 1.8 + head_halo * 1.0 + hot_core * 2.8;
    return vec4<f32>(spectral_color * emission, glow_alpha);
}
