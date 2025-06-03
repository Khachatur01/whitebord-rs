use crate::element::id::Id;
use crate::element::json_entity::JsonEntity;
use crate::element::r#type::ElementType;
use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;

pub mod id;
pub mod r#type;
pub mod json_entity;

pub enum Build {
    FromJson(JsonEntity),
    Default {
        owner_id: String,
    }
}

impl Build {
    pub fn entity_from_json(json: &str) -> Entity<Id> {
        serde_json::from_str::<JsonEntity>(json).unwrap().into()
    }

    pub fn default_entity(element_type: ElementType, owner_id: String) -> Entity<Id> {
        element_type.build(Build::Default { owner_id })
    }
}
