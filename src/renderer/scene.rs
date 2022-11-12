use super::entity::Entity;
use super::lib;

pub struct Scene {
    pub entities: Vec<Entity>,
}

impl Scene {
    pub fn new(entities: Vec<Entity>) -> Scene {
        Scene { entities }
    }

    pub fn draw(&self, ctx: &lib::Context) {
        for entity in &self.entities {
            entity.draw(ctx);
        }
    }
}
