#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: Option<[f32; 3]>,
    pub normal: Option<[f32; 3]>,
}
