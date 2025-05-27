use crate::element::id::ElementId;
use crate::element::json_entity::JsonEntity;
use crate::element::Build;
use graphics_rs::core::entity::Entity;
use graphics_rs::core::feature_set::FeatureSet;
use graphics_rs::core::Model;
use graphics_rs::geometry::figure::point::Point;
use graphics_rs::geometry::figure::polygon::Polygon;
use graphics_rs::geometry::figure::rectangle::Rectangle;
use graphics_rs::standard_entity_plugin::model::container_model::ContainerModel;
use graphics_rs::standard_entity_plugin::model::geometric::polygon_model::PolygonModel;
use graphics_rs::standard_entity_plugin::model::geometric::rectangle_model::RectangleModel;
use graphics_rs::standard_entity_plugin::model::text_model::TextModel;
use graphics_rs::standard_rendering_plugin::style::shape_style::ShapeStyle;
use graphics_rs::standard_rendering_plugin::style::text_style::TextStyle;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
#[wasm_bindgen]
pub enum ElementType {
    Rectangle,
    Polygon,
    Text,
    Container,
}

impl ElementType {
    pub fn build(&self, build: Build) -> Entity {
        match self {
            ElementType::Rectangle =>
                match build {
                    Build::FromJson(entity_json) => Self::model_from_json::<RectangleModel>(entity_json),
                    Build::Default { owner_id } =>
                        RectangleModel::entity(
                            ElementId::generate(&owner_id, self.clone()),
                            Rectangle::zero_sized(Point::default()),
                            ShapeStyle::default()
                        ),
                },
            ElementType::Polygon =>
                match build {
                    Build::FromJson(entity_json) => Self::model_from_json::<PolygonModel>(entity_json),
                    Build::Default { owner_id } =>
                        PolygonModel::entity(
                            ElementId::generate(&owner_id, self.clone()),
                            Polygon::new(&[]),
                            ShapeStyle::default()
                        )
                },
            ElementType::Text =>
                match build {
                    Build::FromJson(entity_json) => Self::model_from_json::<TextModel>(entity_json),
                    Build::Default { owner_id } =>
                        TextModel::entity(
                            ElementId::generate(&owner_id, self.clone()),
                            "",
                            TextStyle::default()
                        )
                },
            ElementType::Container =>
                match build {
                    Build::FromJson(entity_json) => Self::container_from_json(entity_json),
                    Build::Default { owner_id } =>
                        ContainerModel::entity(
                            ElementId::generate(&owner_id, self.clone()),
                        )
                },
        }
    }

    fn model_from_json<M: Model + DeserializeOwned>(entity_json: JsonEntity) -> Entity {
        Entity::new(
            entity_json.id,
            serde_json::from_value::<M>(entity_json.model).unwrap(),
            FeatureSet::empty()
        )
    }

    fn container_from_json(entity_json: JsonEntity) -> Entity {
        #[derive(Deserialize)]
        struct Children {
            children: Vec<JsonEntity>,
        }

        let model_json: Children = serde_json::from_value::<Children>(entity_json.model).unwrap();

        let children: Vec<Entity> = model_json.children
            .into_iter()
            .map(JsonEntity::into)
            .collect();

        Entity::new(
            entity_json.id,
            ContainerModel { children },
            FeatureSet::empty()
        )
    }
}
