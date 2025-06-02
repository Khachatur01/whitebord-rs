use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{SvgElement, SvgRectElement};
use entity_model_feature::entity::Entity;
use standard_svg_plugin::incremental_svg_renderer::SVGRenderer;
use standard_svg_plugin::svg_element::SVGElement;
use standard_svg_plugin::{ToSVG, UpdateSVG};

#[wasm_bindgen]
pub struct IncrementalSvgRenderer {
    svg: SvgElement,
    document: web_sys::Document,
}

#[wasm_bindgen]
impl IncrementalSvgRenderer {
    pub fn new(svg: SvgElement) -> Self {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("global document does not exists");

        Self {
            svg,
            document,
        }
    }
}

impl SVGRenderer for IncrementalSvgRenderer {
    fn remove(&mut self, id: &str) {
        todo!()
    }

    fn add(&mut self, entity: &Entity) {
        let Some(update_svg) = entity.query::<UpdateSVG>() else {
            return;
        };

        let Some(to_svg) = entity.query::<ToSVG>() else {
            return;
        };

        let svg_element: SVGElement = (to_svg.to_svg)(entity);

        let svg_rectangle = self.document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")
            .expect("can't create svg rectangle element")
            .dyn_into::<SvgRectElement>()
            .expect("can't create svg rectangle element");
    }

    fn modify(&mut self, entity: &Entity) {
        todo!()
    }
}
