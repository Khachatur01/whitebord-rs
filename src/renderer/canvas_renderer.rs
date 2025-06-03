use geometry::figure::circle::Circle;
use geometry::figure::ellipse::Ellipse;
use geometry::figure::polygon::Polygon;
use geometry::figure::rectangle::Rectangle;
use geometry::figure::segment::Segment;
use js_sys;
use standard_rendering_plugin::renderer::Renderer;
use standard_rendering_plugin::style::shape_style::ShapeStyle;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct CanvasRenderer {
    context: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl CanvasRenderer {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Self { context }
    }
}

impl CanvasRenderer {
    fn apply_style(&mut self, style: &ShapeStyle) {
        self.context.set_fill_style_str(&style.fill_color.to_hex());
        self.context
            .set_stroke_style_str(&style.stroke_color.to_hex());
        self.context.set_line_width(style.stroke_width);

        let array: js_sys::Array = style
            .stroke_dash_array
            .iter()
            .map(|x| JsValue::from_f64(*x as f64))
            .collect::<js_sys::Array>();

        let _ = self.context.set_line_dash(&array);
    }
}

impl Renderer for CanvasRenderer {
    fn clear(&mut self) {
        self.context.reset();
    }

    fn segment(&mut self, segment: &Segment, style: &ShapeStyle) {
        self.apply_style(style);

        self.context.begin_path();

        self.context
            .move_to(segment.start().x(), segment.start().y());
        self.context.line_to(segment.end().x(), segment.end().y());

        self.context.stroke();
    }

    fn polygon(&mut self, polygon: &Polygon, style: &ShapeStyle) {
        self.apply_style(style);

        self.context.begin_path();

        if let Some((first, rest)) = polygon.vertices().split_first() {
            self.context.move_to(first.x(), first.y());

            for point in rest {
                self.context.line_to(point.x(), point.y());
            }
        }

        self.context.close_path();

        self.context.stroke();
    }

    fn rectangle(&mut self, rectangle: &Rectangle, style: &ShapeStyle) {
        self.apply_style(style);

        self.context.begin_path();

        self.context.fill_rect(
            rectangle.top_left().x(),
            rectangle.top_left().y(),
            rectangle.width(),
            rectangle.height(),
        );
        self.context.rect(
            rectangle.top_left().x(),
            rectangle.top_left().y(),
            rectangle.width(),
            rectangle.height(),
        );

        self.context.stroke();
    }

    fn circle(&mut self, circle: &Circle, style: &ShapeStyle) {
        self.apply_style(style);

        todo!()
    }

    fn ellipse(&mut self, ellipse: &Ellipse, style: &ShapeStyle) {
        self.apply_style(style);

        todo!()
    }
}
