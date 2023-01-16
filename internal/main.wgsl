struct Vertex {
    @location(0) pos: vec2<f32>,
    //@location(1) tex: vec2<f32>,
    @location(1) col: vec4<u32>
}

struct Fragment {
    @builtin(position) pos: vec4<f32>,
    //@location(0) tex: vec2<f32>
    @location(1) col: vec4<f32>
}

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var tex_sampler: sampler;

// super simple vertex shader
@vertex
fn vertex(vert: Vertex) -> Fragment {
    var frag: Fragment;
    //frag.tex = vert.tex;
    frag.pos = vec4<f32>(vert.pos, 1.0, 1.0);
    frag.col = vec4<f32>(vert.col);
    return frag;
}

@fragment
fn fragment(frag: Fragment) -> @location(0) vec4<f32> {
    //return textureSample(texture, tex_sampler, frag.tex);
    return vec4<f32>(1.0, 1.0, 1.0, 1.0) * frag.col;
}