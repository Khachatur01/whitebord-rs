use crate::element::ElementType;
use entity_model_feature::entity_id::EntityId;
use entity_model_feature::AsSerialize;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Id {
    owner_id: String,
    index: usize,
    element_type: ElementType,
}

impl Id {
    pub fn generate(owner_id: &str, element_type: ElementType) -> Self {
        Self {
            owner_id: owner_id.to_string(),
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

    pub fn as_html_id(&self) -> String {
        format!("{}-{}", self.owner_id, self.index)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}_{}", self.owner_id, self.index))
    }
}

impl AsSerialize for Id {
    fn as_serialize(&self) -> &dyn dyn_serde::ser::Serialize {
        self
    }
}

impl EntityId for Id {}
