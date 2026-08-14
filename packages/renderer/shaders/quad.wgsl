struct Uniforms {
    transform: mat4x4<f32>,
    color: vec4<f32>,
    corner_radius: f32,
    _padding: vec3<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_pos: vec2<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_pos = uniforms.transform * vec4<f32>(input.position, 0.0, 1.0);
    output.uv = input.uv;
    output.world_pos = input.position;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let radius = uniforms.corner_radius;
    let size = abs(input.uv - input.uv * 2.0);
    let d = length(max(size - 1.0 + radius, vec2<f32>(0.0))) - radius;
    if (d > 0.0) {
        discard;
    }
    let alpha = smoothstep(1.0, 0.0, d);
    return vec4<f32>(uniforms.color.rgb, uniforms.color.a * alpha);
}