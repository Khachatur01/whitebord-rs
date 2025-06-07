use geometry::figure::circle::Circle;
use geometry::figure::ellipse::Ellipse;
use geometry::figure::polygon::Polygon;
use geometry::figure::rectangle::Rectangle;
use geometry::figure::segment::Segment;
use geometry::figure::triangle::Triangle;
use geometry::point::point_2d::Point2D;
use geometry::point::point_3d::Point3D;
use standard_rendering_plugin::renderer::renderer::camera::Camera;
use standard_rendering_plugin::renderer::renderer::light::Light;
use standard_rendering_plugin::renderer::renderer::Renderer;
use standard_rendering_plugin::style::shape_style::ShapeStyle;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::{Node, SvgElement, SvgLineElement, SvgPathElement, SvgPolygonElement, SvgRectElement};
use geometry::figure::path::Path;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct SVGRenderer {
    svg: SvgElement,
    document: web_sys::Document,
}

#[wasm_bindgen]
impl SVGRenderer {
    pub fn new(svg: SvgElement) -> Self {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("global document does not exists");

        Self {
            svg,
            document
        }
    }
}

impl Renderer for SVGRenderer {
    fn clear(&mut self) {
        self.svg.set_inner_html("");
    }

    fn path(&mut self, path: &Path, style: &ShapeStyle) {
        let svg_path = self.document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
            .expect("can't create svg path element")
            .dyn_into::<SvgPathElement>()
            .expect("can't create svg path element");

        svg_path
            .set_attribute("d", &path.to_svg_path())
            .expect("TODO: panic message");

        svg_path
            .set_attribute("fill", "none")
            .expect("TODO: panic message");
        svg_path
            .set_attribute("stroke", "black")
            .expect("TODO: panic message");

        self.svg
            .append_child(&svg_path.dyn_into::<Node>().expect(""))
            .expect("");
    }

    fn segment_2d(&mut self, segment: &Segment<Point2D>, style: &ShapeStyle) {
        let svg_line = self.document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "line")
            .expect("can't create svg line element")
            .dyn_into::<SvgLineElement>()
            .expect("can't create svg line element");

        svg_line
            .set_attribute("stroke", "black")
            .expect("TODO: panic message");
        svg_line
            .set_attribute("stroke-dasharray", "4 1")
            .expect("TODO: panic message");

        svg_line
            .set_attribute("x1", &format!("{}", segment.start().x()))
            .expect("TODO: panic message");
        svg_line
            .set_attribute("y1", &format!("{}", segment.start().y()))
            .expect("TODO: panic message");

        svg_line
            .set_attribute("x2", &format!("{}", segment.end().x()))
            .expect("TODO: panic message");
        svg_line
            .set_attribute("y2", &format!("{}", segment.end().y()))
            .expect("TODO: panic message");

        self.svg
            .append_child(&svg_line.dyn_into::<Node>().expect(""))
            .expect("");
    }

    fn polygon_2d(&mut self, polygon: &Polygon<Point2D>, shape_style: &ShapeStyle) {
        let svg_polygon = self.document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "polygon")
            .expect("can't create svg polygon element")
            .dyn_into::<SvgPolygonElement>()
            .expect("can't create svg polygon element");

        svg_polygon
            .set_attribute("stroke", "black")
            .expect("TODO: panic message");
        svg_polygon
            .set_attribute("fill", "none")
            .expect("TODO: panic message");

        let points = polygon
            .vertices()
            .iter()
            .map(|vertex| format!("{},{}", vertex.x, vertex.y))
            .collect::<Vec<String>>()
            .join(" ");

        svg_polygon.set_attribute("points", &points).expect("");

        self.svg
            .append_child(&svg_polygon.dyn_into::<Node>().expect(""))
            .expect("");
    }

    fn triangle_2d(&mut self, polygon: &Triangle<Point2D>, style: &ShapeStyle) {
        todo!()
    }

    fn rectangle(&mut self, rectangle: &Rectangle, style: &ShapeStyle) {
        let svg_rectangle = self.document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")
            .expect("can't create svg rectangle element")
            .dyn_into::<SvgRectElement>()
            .expect("can't create svg rectangle element");

        let absolute_sized_rectangle: Rectangle = rectangle.absolute_sized();
        svg_rectangle
            .set_attribute("x", &format!("{}", absolute_sized_rectangle.top_left().x()))
            .expect("TODO: panic message");
        svg_rectangle
            .set_attribute("y", &format!("{}", absolute_sized_rectangle.top_left().y()))
            .expect("TODO: panic message");

        svg_rectangle
            .set_attribute("width", &format!("{}", absolute_sized_rectangle.width()))
            .expect("TODO: panic message");
        svg_rectangle
            .set_attribute("height", &format!("{}", absolute_sized_rectangle.height()))
            .expect("TODO: panic message");

        svg_rectangle
            .set_attribute("fill", "none")
            .expect("TODO: panic message");
        svg_rectangle
            .set_attribute("stroke", "black")
            .expect("TODO: panic message");

        self.svg
            .append_child(&svg_rectangle.dyn_into::<Node>().expect(""))
            .expect("");
    }

    fn circle(&mut self, circle: &Circle, style: &ShapeStyle) {
        todo!()
    }

    fn ellipse(&mut self, ellipse: &Ellipse, style: &ShapeStyle) {
        todo!()
    }

    fn segment_3d(&mut self, segment: &Segment<Point3D>, style: &ShapeStyle) {
        todo!()
    }

    fn polygon_3d(&mut self, polygon: &Polygon<Point3D>, style: &ShapeStyle) {
        todo!()
    }

    fn triangles_3d(&mut self, triangles: &[(&Triangle<Point3D>, &ShapeStyle)], camera: &Camera, light: &Light) {
        todo!()
    }
}
