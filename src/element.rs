use crate::element::id::Id;
use crate::element::json_entity::JsonEntity;
use entity_model_feature::entity::Entity;
use entity_model_feature::feature_set::FeatureSet;
use entity_model_feature::Model;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use standard_entity_plugin::entity_model::{DefaultEntity, StandardFeatureSet};
use standard_entity_plugin::model::container_model::ContainerModel;
use standard_entity_plugin::model::polygon_model::PolygonModel;
use standard_entity_plugin::model::rectangle_model::RectangleModel;
use standard_entity_plugin::model::path_model::PathModel;
use standard_entity_plugin::model::text_model::TextModel;
use std::result;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod id;
pub mod json_entity;

pub type Result<Ok> = result::Result<Ok, serde_json::Error>;

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[wasm_bindgen]
pub enum ElementType {
    Rectangle,
    Polygon,
    FreeHand,
    Text,
    Container,
}


pub enum Build {
    FromJson(JsonEntity),
    Default {
        owner_id: String,
        element_type: ElementType
    }
}

impl Build {
    pub fn default(owner_id: &str, element_type: ElementType) -> Entity<Id> {
        /* Building default entity should not cause an error, so we unwrap the result safely */
        Build::Default {owner_id: owner_id.to_string(), element_type}.build().unwrap()
    }

    pub fn from_json(json_entity: JsonEntity) -> Result<Entity<Id>> {
        Build::FromJson(json_entity).build()
    }

    pub fn build(self) -> Result<Entity<Id>> {
        let element_type = match &self {
            Build::FromJson(JsonEntity { id, .. }) => id.element_type(),
            Build::Default { element_type, .. } => element_type,
        };

        match element_type {
            ElementType::Rectangle => self.build_entity::<RectangleModel>(),
            ElementType::Polygon => self.build_entity::<PolygonModel>(),
            ElementType::FreeHand => self.build_entity::<PathModel>(),
            ElementType::Text => self.build_entity::<TextModel>(),
            ElementType::Container => self.build_container::<ContainerModel<Id>>(),
        }
    }

    fn build_entity<M: Model + DeserializeOwned + DefaultEntity<Id> + StandardFeatureSet<Id>>(self) -> Result<Entity<Id>> {
        let mut entity: Entity<Id> = match self {
            Build::FromJson(entity_json) =>
                Self::entity_model_from_json::<M>(entity_json)?,
            Build::Default { owner_id, element_type } =>
                M::default_entity(Id::generate(&owner_id, element_type)),
        };

        entity.add_feature_set(M::standard_feature_set());

        Ok(entity)
    }
    fn build_container<M: Model + DefaultEntity<Id> + StandardFeatureSet<Id>>(self) -> Result<Entity<Id>> {
        let mut container_entity: Entity<Id> = match self {
            Build::FromJson(entity_json) =>
                Self::container_model_from_json(entity_json)?,
            Build::Default { owner_id, element_type } =>
                M::default_entity(Id::generate(&owner_id, element_type)),
        };

        container_entity.add_feature_set(M::standard_feature_set());

        Ok(container_entity)
    }

    fn entity_model_from_json<M: Model + DeserializeOwned>(entity_json: JsonEntity) -> Result<Entity<Id>> {
        Ok(
            Entity::new(
                entity_json.id,
                serde_json::from_value::<M>(entity_json.model)?,
                FeatureSet::empty()
            )
        )
    }
    fn container_model_from_json(entity_json: JsonEntity) -> Result<Entity<Id>> {
        #[derive(Deserialize)]
        struct Children {
            children: Vec<JsonEntity>,
        }

        let model_json: Children = serde_json::from_value::<Children>(entity_json.model)?;

        let children: Result<Vec<Entity<Id>>> = model_json.children
            .into_iter()
            .map(JsonEntity::try_into)
            .collect();

        let children: Vec<Entity<Id>> = children?;

        Ok(
            Entity::new(
                entity_json.id,
                ContainerModel { children },
                FeatureSet::empty()
            )
        )
    }
}
