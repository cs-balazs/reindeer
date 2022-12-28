pub type Vec3 = [f32; 3];

pub trait Vec3Constructable {
    fn new(x: f32, y: f32, z: f32) -> Self;
}

impl Vec3Constructable for Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        [x, y, z]
    }
}
