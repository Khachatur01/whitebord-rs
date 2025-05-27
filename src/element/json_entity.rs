use crate::element::id::ElementId;
use crate::element::Build;
use graphics_rs::core::entity::Entity;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonEntity {
    pub id: ElementId,
    pub model: serde_json::Value,
}

impl JsonEntity {
    pub fn entity_from_str(str: &str) -> Entity {
        serde_json::from_str::<JsonEntity>(str).unwrap().into()
    }
}

impl Into<Entity> for JsonEntity {
    fn into(self) -> Entity {
        self
            .id
            .clone()
            .element_type()
            .build(Build::FromJson(self))
    }
}
