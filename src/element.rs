use crate::element::json_entity::JsonEntity;
use crate::element::r#type::ElementType;
use graphics_rs::core::entity::Entity;

mod id;
pub mod r#type;
pub mod json_entity;

pub enum Build {
    FromJson(JsonEntity),
    Default {
        owner_id: String,
    }
}

pub fn build_from_json(json: &str) -> Entity {
    serde_json::from_str::<JsonEntity>(json).unwrap().into()
}

pub fn build_default(element_type: ElementType, owner_id: String) -> Entity {
    element_type.build(Build::Default { owner_id })
}
