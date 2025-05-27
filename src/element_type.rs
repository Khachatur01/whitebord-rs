use crate::element_id::ElementId;
use graphics_rs::core::entity::Entity;
use graphics_rs::geometry::figure::point::Point;
use graphics_rs::geometry::figure::polygon::Polygon;
use graphics_rs::geometry::figure::rectangle::Rectangle;
use graphics_rs::standard_entity_plugin::entity::container_entity::ContainerEntity;
use graphics_rs::standard_entity_plugin::entity::geometric::polygon_entity::PolygonEntity;
use graphics_rs::standard_entity_plugin::entity::geometric::rectangle_entity::RectangleEntity;
use graphics_rs::standard_rendering_plugin::style::shape_style::ShapeStyle;
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Serialize, Eq, PartialEq)]
#[wasm_bindgen]
pub enum ElementType {
    Rectangle,
    Polygon,
    Container,
}

impl ElementType {
    pub fn default(&self, owner_id: &str) -> Entity {
        let id: ElementId = ElementId::generate(owner_id, self.clone());

        match self {
            ElementType::Rectangle => 
                RectangleEntity::with_standard_feature_set(id, Rectangle::zero_sized(Point::default()), ShapeStyle::default()),
            ElementType::Polygon => 
                PolygonEntity::with_standard_feature_set(id, Polygon::new(&[]), ShapeStyle::default()),
            ElementType::Container => 
                ContainerEntity::with_standard_feature_set(id),
        }
    }
}
