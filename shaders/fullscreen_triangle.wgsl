// Create rectangle the size of the screen with a single tri

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
};

@vertex
fn vert_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var output: VertexOutput;
    var uv: vec3<f32> = vec3<f32>(f32((vertex_index << 1u) & 2u), f32(vertex_index & 2u), 1.0);
    output.position = vec4<f32>((mat3x3(-4.0, 0.0, 0.0, -4.0, -4.0, 0.0, 3.0, 1.0, 1.0) * uv).xy, 0.0, 1.0);
    return output;
}

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var pi: f32 = 3.14159;
    // remove the gross part of the color spectrum
    let rem = pi / 3.0;
    let x: f32 = in.position.x / 1280.0 * (2.0 * pi - rem) + pi / 2.0;

    return vec4<f32>(col_val(x), col_val(x - (2.0 * pi / 3.0)), col_val(x - (4.0 * pi / 3.0)), 1.0);
}

fn col_val(x: f32) -> f32 {
    let sin = clamp(sin(x) * 0.75 + 0.5, 0.0, 1.0);
    // convert from rgb to srgb
    return pow(sin, 2.2);
}