use crate::element::id::Id;
use crate::element::Build;
use entity_model_feature::entity::Entity;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonEntity {
    pub id: Id,
    pub model: serde_json::Value,
}

impl JsonEntity {
    pub fn entity_from_str(str: &str) -> Entity<Id> {
        serde_json::from_str::<JsonEntity>(str).unwrap().into()
    }
}

impl Into<Entity<Id>> for JsonEntity {
    fn into(self) -> Entity<Id> {
        self
            .id
            .clone()
            .element_type()
            .build(Build::FromJson(self))
    }
}
