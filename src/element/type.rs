use crate::element::id::ElementId;
use crate::element::json_entity::JsonEntity;
use crate::element::Build;
use geometry::figure::point::Point;
use geometry::figure::polygon::Polygon;
use geometry::figure::rectangle::Rectangle;
use standard_entity_plugin::model::container_model::ContainerModel;
use standard_entity_plugin::model::geometric::polygon_model::PolygonModel;
use standard_entity_plugin::model::geometric::rectangle_model::RectangleModel;
use standard_entity_plugin::model::text_model::TextModel;
use standard_rendering_plugin::style::shape_style::ShapeStyle;
use standard_rendering_plugin::style::text_style::TextStyle;
use entity_model_feature::entity::Entity;
use entity_model_feature::feature_set::FeatureSet;
use entity_model_feature::Model;
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
