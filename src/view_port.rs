use crate::element::id::Id;
use entity_model_feature::entity::Entity;
use standard_rendering_plugin::renderable::Renderable;
use standard_rendering_plugin::renderer::renderer::Renderer;
use standard_rendering_plugin::Render;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct LockError<'a>(&'a str);

#[derive(Clone)]
pub struct ViewPort {
    entities: Arc<RwLock<Vec<Entity<Id>>>>,
}

impl ViewPort {
    pub fn new() -> Self {
        Self {
            entities: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn add_entity(&mut self, entity: Entity<Id>) -> Result<(), LockError> {
        self.entities
            .write()
            .map_err(|_| LockError("Failed to acquire lock"))?
            .push(entity);

        Ok(())
    }
}

impl Renderable for ViewPort {
    fn render(&self, renderer: &mut dyn Renderer) {
        /* Use try_read to not lock the current thread for rendering.  */
        let Ok(entities) = self.entities.try_read() else {
            return;
        };

        for entity in entities.iter() {
            if let Some(render) = entity.query::<Render<Id>>() {
                (render.render)(entity, renderer);
            }
        }
    }
}
