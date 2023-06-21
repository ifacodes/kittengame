// Create rectangle the size of the screen with a single tri

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(0) vertex_position: vec2<f32>
};

@vertex
fn vert_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var output: VertexOutput;
    if vertex_index == 2u {
        output.position.x = 3.0;
    } else {
        output.position.x = -1.0;
    }
    if vertex_index == 1u {
        output.position.y = -3.0;
    } else {
        output.position.y = 1.0;
    }
    output.vertex_position = output.position.xy;
    output.position = vec4<f32>(output.position.xy, 0.0, 1.0);
    return output;
}

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.vertex_position, 0.5, 1.0)
}
