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
    // Treat the particle quad as a carrier canvas. Draw a broad streak even
    // when the configured particle size is small, while respecting size controls.
    let speed_factor = clamp(speed / 0.30, 0.15, 1.0);
    let size_factor = clamp(uniforms.scale.x, 0.333, 6.667);
    let trail_length = 0.012 * mix(0.65, 2.6, speed_factor) * size_factor;
    let width = 0.0035 * mix(0.80, 1.7, speed_factor) * size_factor;
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
    // uv.x=1 is the particle head; uv.x=0 is the tail end.
    let transverse = (vertex.tex_coords.y - 0.5) * 2.0;
    let tail_distance = 1.0 - vertex.tex_coords.x;

    // Keep the trail readable with standard alpha blending on SDR swapchains:
    // RGB values above 1.0 do not produce bloom without an HDR post-process.
    let tail_envelope = exp(-tail_distance * mix(2.0, 0.75, vertex.speed));
    let broad_halo = exp(-transverse * transverse * 2.0);
    let streak = exp(-transverse * transverse * 8.0);
    let filament = exp(-transverse * transverse * 24.0);
    let trail = tail_envelope
        * (broad_halo * 0.28 + streak * 0.42 + filament * 0.55)
        * mix(0.45, 0.85, vertex.speed);

    let head_x = (vertex.tex_coords.x - 0.91) * 13.0;
    let head_halo = exp(-head_x * head_x * 0.22)
        * exp(-transverse * transverse * 2.0);
    let hot_core = exp(-head_x * head_x * 1.0)
        * exp(-transverse * transverse * 18.0);
    let rim = exp(-transverse * transverse * 3.5)
        * smoothstep(0.20, 0.95, abs(transverse));

    let spectral_mix = clamp(rim * 0.28 + hot_core * 0.18, 0.0, 0.42);
    let spectral_color = mix(vertex.color.rgb, vec3<f32>(0.45, 0.90, 1.0), spectral_mix);
    let alpha = clamp(trail + head_halo * 0.62 + hot_core, 0.0, 0.94)
        * vertex.color.a;
    let brightness = clamp(0.65 + trail * 0.55 + head_halo * 0.30 + hot_core * 0.60, 0.0, 1.0);
    return vec4<f32>(spectral_color * brightness, alpha);
}
