use std::sync::{Arc, RwLock};
use graphics_rs::entity_model_feature::entity::Entity;
use graphics_rs::standard_rendering_plugin::Render;
use graphics_rs::standard_rendering_plugin::renderer::{Renderable, Renderer};

#[derive(Debug)]
pub struct LockError<'a>(&'a str);

#[derive(Clone)]
pub struct ViewPort {
    entities: Arc<RwLock<Vec<Entity>>>,
}

impl ViewPort {
    pub fn new() -> Self {
        Self {
            entities: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) -> Result<(), LockError> {
        self.entities
            .write()
            .map_err(|_| LockError("Failed to acquire lock"))?
            .push(entity);

        Ok(())
    }
}

impl Renderable for ViewPort {
    fn render(&self, renderer: &mut dyn Renderer) {
        let Ok(entities) = self.entities.read() else {
            return;
        };

        for entity in entities.iter() {
            if let Some(render) = entity.query::<Render>() {
                (render.render)(entity, renderer);
            }
        }
    }
}
