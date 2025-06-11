use algebra::linear::matrix::Matrix;
use geometry::figure::circle::Circle;
use geometry::figure::ellipse::Ellipse;
use geometry::figure::path::command::Command;
use geometry::figure::path::Path;
use geometry::figure::polygon::Polygon;
use geometry::figure::rectangle::Rectangle;
use geometry::figure::segment::Segment;
use geometry::figure::triangle::Triangle;
use geometry::point::point_2d::Point2D;
use geometry::point::point_3d::Point3D;
use js_sys;
use standard_rendering_plugin::renderer::renderer::camera::Camera;
use standard_rendering_plugin::renderer::renderer::light::Light;
use standard_rendering_plugin::renderer::renderer::Renderer;
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
            .set_stroke_style_str(&style.stroke.color.to_hex());
        self.context.set_line_width(style.stroke.width);

        let array: js_sys::Array = style
            .stroke.dash_array
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

    fn path(&mut self, path: &Path, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>) {
        self.apply_style(style);

        self.context.begin_path();

        for command in path.commands() {
            match command {
                Command::MoveTo(move_to) =>
                    self.context.move_to(move_to.to_point.x, move_to.to_point.y),
                Command::LineTo(line_to) =>
                    self.context.line_to(line_to.to_point.x, line_to.to_point.y),
                Command::HorizontalLineTo(horizontal_line_to) => {}
                Command::VerticalLineTo(vertical_line_to) => {}
                Command::BezierTo(bezier_to) => {}
                Command::ArcTo(arc_to) => {}
                Command::Close => {}
            }
        }

        self.context.stroke();
    }

    fn segment_2d(&mut self, segment: &Segment<Point2D>, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>) {
        self.apply_style(style);

        self.context.begin_path();

        self.context
            .move_to(segment.start().x(), segment.start().y());
        self.context.line_to(segment.end().x(), segment.end().y());

        self.context.stroke();
    }

    fn polygon_2d(&mut self, polygon: &Polygon<Point2D>, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>) {
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

    fn triangle_2d(&mut self, polygon: &Triangle<Point2D>, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>) {
        todo!()
    }

    fn rectangle(&mut self, rectangle: &Rectangle, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>) {
        self.apply_style(style);

        self.context.begin_path();

        self.context.fill_rect(
            rectangle.top_left.x,
            rectangle.top_left.y,
            rectangle.width,
            rectangle.height,
        );
        self.context.rect(
            rectangle.top_left.x,
            rectangle.top_left.y,
            rectangle.width,
            rectangle.height,
        );

        self.context.stroke();
    }

    fn circle(&mut self, circle: &Circle, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>) {
        self.apply_style(style);

        todo!()
    }

    fn ellipse(&mut self, ellipse: &Ellipse, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>) {
        self.apply_style(style);

        todo!()
    }

    fn segment_3d(&mut self, segment: &Segment<Point3D>, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>) {
        todo!()
    }

    fn polygon_3d(&mut self, polygon: &Polygon<Point3D>, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>) {
        todo!()
    }

    fn triangles_3d(&mut self, triangles: &[(&Triangle<Point3D>, &ShapeStyle)], camera: &Camera, light: &Light, transform_matrix: Option<Matrix<3>>) {
        todo!()
    }
}
