#[derive(Debug, Clone)]
pub struct VertexAttribute {
    pub count: u32,
    pub item_type: u32,
    pub item_size: u32,
}

impl VertexAttribute {
    pub fn new(count: u32, item_type: u32, item_size: u32) -> VertexAttribute {
        VertexAttribute {
            count,
            item_size,
            item_type,
        }
    }
}
