use crate::BACKEND;

use super::entity::Entity;

pub trait Drawable {
    fn draw(&self);
}

impl Drawable for Vec<Entity> {
    fn draw(&self) {
        for entity in self {
            entity.draw(&BACKEND.lock().unwrap()); // TODO: This should be BACNEKD.draw(entity) instead....
        }
    }
}
