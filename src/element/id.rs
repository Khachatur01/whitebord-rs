use crate::element::r#type::ElementType;
use graphics_rs::core::{AsSerialize, EntityId};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ElementId {
    owner_id: String,
    index: usize,
    element_type: ElementType,
}

impl ElementId {
    pub fn generate(id: &str, element_type: ElementType) -> Self {
        Self {
            owner_id: id.to_string(),
            index: js_sys::Date::new_0().get_milliseconds() as usize,
            element_type
        }
    }

    pub fn owner_id(&self) -> &str {
        self.owner_id.as_str()
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn element_type(&self) -> &ElementType {
        &self.element_type
    }
}

impl Display for ElementId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}_{}", self.owner_id, self.index))
    }
}

impl AsSerialize for ElementId {
    fn as_serialize(&self) -> &dyn dyn_serde::ser::Serialize {
        self
    }
}

impl EntityId for ElementId {}
