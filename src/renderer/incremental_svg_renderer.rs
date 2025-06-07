mod update_svg;
mod create_svg;

use crate::element::id::Id;
use crate::renderer::incremental_svg_renderer::create_svg::CreateSVG;
use crate::renderer::incremental_svg_renderer::update_svg::UpdateSVG;
use entity_model_feature::entity::Entity;
use standard_rendering_plugin::renderer::renderer_incremental::RendererIncremental;
use standard_svg_plugin::svg_element::{SVGElement, SVG};
use standard_svg_plugin::ToSVG;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::{Element, SvgCircleElement, SvgElement, SvgLineElement, SvgRectElement};

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

impl RendererIncremental<Id> for IncrementalSvgRenderer {
    fn add(&mut self, entity: &Entity<Id>) {
        let Some(to_svg) = entity.query::<ToSVG<Id>>() else {
            return;
        };

        let svg_element: SVGElement = (to_svg.to_svg)(entity);
        let svg_node: &SvgElement = match svg_element.svg() {
            SVG::SVG =>
                todo!(),
            SVG::Group =>
                todo!(),
            SVG::Circle(circle) =>
                &SvgCircleElement::create_svg(&self.document, "circle", circle, svg_element.attributes(), svg_element.css()),
            SVG::Ellipse =>
                todo!(),
            SVG::Line(line) =>
                &SvgLineElement::create_svg(&self.document, "line", line, svg_element.attributes(), svg_element.css()),
            SVG::Polygon =>
                todo!(),
            SVG::Polyline =>
                todo!(),
            SVG::Rectangle(rectangle) =>
                &SvgRectElement::create_svg(&self.document, "rect", rectangle, svg_element.attributes(), svg_element.css()),
            SVG::Text =>
                todo!(),
            SVG::Image =>
                todo!(),
            SVG::Path =>
                todo!(),
            SVG::ForeignObject =>
                todo!(),
        };

        svg_node.set_id(&entity.id().as_html_id());
        self.svg.append_child(&svg_node).expect("Can't append element do svg container.");
    }

    fn modify(&mut self, entity: &Entity<Id>) {
        let Some(to_svg) = entity.query::<ToSVG<Id>>() else {
            return;
        };

        let id: &str = &entity.id().as_html_id();
        let svg_node: Element = self.document.get_element_by_id(id).unwrap();

        let svg_element: SVGElement = (to_svg.to_svg)(entity);
        match svg_element.svg() {
            SVG::SVG =>
                todo!(),
            SVG::Group =>
                todo!(),
            SVG::Circle(circle) =>
                svg_node.dyn_into::<SvgCircleElement>().unwrap().update_svg(circle, svg_element.attributes(), svg_element.css()),
            SVG::Ellipse =>
                todo!(),
            SVG::Line(line) =>
                svg_node.dyn_into::<SvgLineElement>().unwrap().update_svg(line, svg_element.attributes(), svg_element.css()),
            SVG::Polygon =>
                todo!(),
            SVG::Polyline =>
                todo!(),
            SVG::Rectangle(rectangle) =>
                svg_node.dyn_into::<SvgRectElement>().unwrap().update_svg(rectangle, svg_element.attributes(), svg_element.css()),
            SVG::Text =>
                todo!(),
            SVG::Image =>
                todo!(),
            SVG::Path =>
                todo!(),
            SVG::ForeignObject =>
                todo!(),
        };
    }

    fn remove(&mut self, id: &Id) {
        let html_id: &str = &id.as_html_id();

        self.document.get_element_by_id(html_id).expect(&format!("Can't find element by id {html_id} to remove")).remove();
    }
}
