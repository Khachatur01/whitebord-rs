use std::fmt::Display;
use wasm_bindgen::prelude::wasm_bindgen;
use graphics_rs::core::entity::Id;

#[wasm_bindgen]
#[derive(Clone)]
pub struct ElementId {
    owner_id: String,
    index: usize,
}

impl ElementId {
    pub fn with_owner_id(id: &str) -> Self {
        Self {
            owner_id: id.to_string(),
            index: js_sys::Date::new_0().get_milliseconds() as usize,
        }
    }

    pub fn owner_id(&self) -> &str {
        self.owner_id.as_str()
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl Display for ElementId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}_{}", self.owner_id, self.index))
    }
}

impl Id for ElementId {}
