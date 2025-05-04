use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::{Node, SvgElement, SvgLineElement, SvgPolygonElement, SvgRectElement};
use graphics_rs::geometry::figure::circle::Circle;
use graphics_rs::geometry::figure::ellipse::Ellipse;
use graphics_rs::geometry::figure::polygon::Polygon;
use graphics_rs::geometry::figure::rectangle::Rectangle;
use graphics_rs::geometry::figure::segment::Segment;
use graphics_rs::standard_rendering_plugin::renderer::Renderer;
use graphics_rs::standard_rendering_plugin::style::shape_style::ShapeStyle;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen]
pub struct SVGRenderer {
    svg: SvgElement,
}

#[wasm_bindgen]
impl SVGRenderer {
    pub fn new(svg: SvgElement) -> Self {
        Self { svg }
    }
}

impl Renderer for SVGRenderer {
    fn clear(&mut self) {
        self.svg.set_inner_html("");
    }

    fn segment(&mut self, segment: &Segment, style: &ShapeStyle) {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("global document does not exists");

        let svg_line = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "line")
            .expect("can't create svg line element")
            .dyn_into::<SvgLineElement>()
            .expect("can't create svg line element");

        svg_line.set_attribute("stroke", "black").expect("TODO: panic message");
        svg_line.set_attribute("stroke-dasharray", "4 1").expect("TODO: panic message");

        svg_line.set_attribute("x1", &format!("{}", segment.start().x())).expect("TODO: panic message");
        svg_line.set_attribute("y1", &format!("{}", segment.start().y())).expect("TODO: panic message");

        svg_line.set_attribute("x2", &format!("{}", segment.end().x())).expect("TODO: panic message");
        svg_line.set_attribute("y2", &format!("{}", segment.end().y())).expect("TODO: panic message");

        self.svg.append_child(&svg_line.dyn_into::<Node>().expect("")).expect("");
    }

    fn polygon(&mut self, polygon: &Polygon, shape_style: &ShapeStyle) {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("global document does not exists");

        let svg_polygon = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "polygon")
            .expect("can't create svg polygon element")
            .dyn_into::<SvgPolygonElement>()
            .expect("can't create svg polygon element");

        svg_polygon.set_attribute("stroke", "black").expect("TODO: panic message");
        svg_polygon.set_attribute("fill", "none").expect("TODO: panic message");

        let points = polygon
            .vertices()
            .iter()
            .map(|vertex| format!("{},{}", vertex.x, vertex.y))
            .collect::<Vec<String>>()
            .join(" ");

        log(points.as_str());

        svg_polygon.set_attribute("points", &points).expect("");

        self.svg.append_child(&svg_polygon.dyn_into::<Node>().expect("")).expect("");
    }

    fn rectangle(&mut self, rectangle: &Rectangle, style: &ShapeStyle) {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("global document does not exists");

        let svg_rectangle = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")
            .expect("can't create svg rectangle element")
            .dyn_into::<SvgRectElement>()
            .expect("can't create svg rectangle element");

        let absolute_sized_rectangle: Rectangle = rectangle.absolute_sized();
        svg_rectangle.set_attribute("x", &format!("{}", absolute_sized_rectangle.top_left().x())).expect("TODO: panic message");
        svg_rectangle.set_attribute("y", &format!("{}", absolute_sized_rectangle.top_left().y())).expect("TODO: panic message");

        svg_rectangle.set_attribute("width", &format!("{}", absolute_sized_rectangle.width())).expect("TODO: panic message");
        svg_rectangle.set_attribute("height", &format!("{}", absolute_sized_rectangle.height())).expect("TODO: panic message");

        svg_rectangle.set_attribute("fill", "none").expect("TODO: panic message");
        svg_rectangle.set_attribute("stroke", "black").expect("TODO: panic message");

        self.svg.append_child(&svg_rectangle.dyn_into::<Node>().expect("")).expect("");
    }

    fn circle(&mut self, circle: &Circle, style: &ShapeStyle) {
        todo!()
    }

    fn ellipse(&mut self, ellipse: &Ellipse, style: &ShapeStyle) {
        todo!()
    }
}
