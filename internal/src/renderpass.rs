use crate::types::{shader::Shader, vertex::Vertex, Color, Topology};
use arena::Key;
use glam::{Mat3, Mat4};

#[derive(Debug)]
struct RenderPass {
    data: Vec<RenderPassData>,
}

impl RenderPass {}

#[derive(Debug, Default)]
struct RenderPassBuilder {
    clear_color: Option<Color>,
    target: Option<()>,
    matrix_stack: Vec<Mat3>,
}

impl RenderPassBuilder {
    pub fn clear_color(mut self, color: impl Into<Color>) -> Self {
        self.clear_color = Some(color.into());
        self
    }

    pub fn target(self, target: ()) -> Self {
        todo!()
    }

    pub fn build(self) -> RenderPass {
        RenderPass { data: vec![] }
    }
}

#[derive(Debug)]
struct RenderPassData {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    vertex_count: usize,
    indices_count: usize,
    shader: Option<Key<Shader>>,
    matrix: Mat4,
    topology: Topology,
    scissor: Option<(i32, i32, i32, i32)>,
    clear_color: Color,
}

impl RenderPassData {
    fn new(shader: Option<Key<Shader>>, matrix: Mat4, topology: Topology) -> Self {
        Self {
            vertices: vec![],
            indices: vec![],
            vertex_count: 0,
            indices_count: 0,
            shader,
            matrix,
            topology,
            scissor: None,
            clear_color: Default::default(),
        }
    }
}

impl Default for RenderPassData {
    fn default() -> Self {
        Self::new(None, Mat4::IDENTITY, Topology::Triangles)
    }
}
