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
    let speed_factor = clamp(speed / 0.3, 0.0, 1.0);
    // Longer trails at higher speeds, while keeping slow particles compact.
    let trail_length = mix(0.010, 0.078, speed_factor);
    let width = mix(0.0035, 0.0060, speed_factor);
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
    let tail_fade = exp(-tail_distance * mix(1.9, 3.4, vertex.speed));
    let soft_halo = exp(-transverse * transverse * 1.35);
    let narrow_streak = exp(-transverse * transverse * 8.0);
    let filament = exp(-transverse * transverse * 22.0);
    let trail = tail_fade
        * (soft_halo * 0.18 + narrow_streak * 0.48 + filament * 0.20)
        * mix(0.25, 0.56, vertex.speed);

    let head_x = (uv.x - 0.92) * 18.0;
    let head_halo = exp(-head_x * head_x * 0.18)
        * exp(-transverse * transverse * 1.8);
    let hot_core = exp(-head_x * head_x * 1.8)
        * exp(-transverse * transverse * 20.0);
    let rim = exp(-transverse * transverse * 2.8)
        * smoothstep(0.35, 0.95, abs(transverse));

    // A restrained cyan/ice highlight gives the stream a prismatic edge.
    let spectral_mix = clamp(rim * 0.20 + hot_core * 0.12, 0.0, 0.28);
    let spectral_color = mix(vertex.color.rgb, vec3<f32>(0.48, 0.88, 1.0), spectral_mix);
    let glow_alpha = clamp(trail + head_halo * 0.28 + hot_core * 0.82, 0.0, 1.0)
        * vertex.color.a;
    let emission = 0.48 + trail * 1.35 + head_halo * 0.65 + hot_core * 2.25;
    return vec4<f32>(spectral_color * emission, glow_alpha);
}
