struct Fragment {
    @builtin(position) pos: vec4<f32>,
    //@location(0) tex: vec2<f32>
    @location(1) col: vec4<f32>
}

struct Output {
    @location(0) diffuse: vec4<f32>
}

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var tex_sampler: sampler;
@group(1) @binding(0)
var<uniform> matrix: mat4x4<f32>;


// super simple vertex shader
@vertex
fn vertex(@location(0) pos: vec2<f32>, @location(1) tex_pos: vec2<f32>, @location(2) col: vec4<u32>) -> Fragment {
    var frag: Fragment;
    //frag.tex = vert.tex;
    frag.pos = vec4<f32>(pos, 1.0, 1.0);
    frag.col = vec4<f32>(col);
    return frag;
}

@fragment
fn fragment(frag: Fragment) -> Output {
    //return textureSample(texture, tex_sampler, frag.tex);
    var output: Output;
    output.diffuse = vec4<f32>(1.0, 1.0, 1.0, 1.0) * frag.col;
    return output;
}