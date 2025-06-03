use crate::renderer::incremental_svg_renderer::update_svg::UpdateSVG;
use standard_svg_plugin::property_map::PropertyMap;
use wasm_bindgen::JsCast;
use web_sys::Document;

pub trait CreateSVG<SVGElement> {
    fn create_svg(document: &Document, tag: &str, svg_element: &SVGElement, attributes: &PropertyMap, css: &PropertyMap) -> Self;
}

/* Implement CreateSVG for all dom svg elements that implement UpdateSVG trait. */
impl<SVGElement, DOMElement> CreateSVG<SVGElement> for DOMElement
where DOMElement: UpdateSVG<SVGElement> + JsCast {
    fn create_svg(document: &Document, tag: &str, svg_element: &SVGElement, attributes: &PropertyMap, css: &PropertyMap) -> Self {
        let mut dom_svg_element: DOMElement = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), tag)
            .expect(format!("Can't create svg {tag} element.").as_str())
            .dyn_into::<DOMElement>()
            .expect(format!("Can't cast JSValue into {tag} node.").as_str());

        dom_svg_element.update_svg(svg_element, attributes, css);

        dom_svg_element
    }
}
