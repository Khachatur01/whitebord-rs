mod element_id;
mod renderer;

use crate::element_id::ElementId;
use renderer::canvas_renderer::CanvasRenderer;
use renderer::svg_renderer::SVGRenderer;
use wasm_bindgen::prelude::wasm_bindgen;
use graphics_rs::geometry::figure::point::Point;
use graphics_rs::geometry::figure::polygon::Polygon;
use graphics_rs::geometry::figure::rectangle::Rectangle;
use graphics_rs::standard_entity_plugin::entity::geometric::polygon_entity::PolygonEntity;
use graphics_rs::standard_entity_plugin::entity::geometric::rectangle_entity::RectangleEntity;
use graphics_rs::standard_rendering_plugin::style::shape_style::ShapeStyle;
use graphics_rs::standard_tool_plugin::tool::draw_tool::click_draw_tool::ClickDrawTool;
use graphics_rs::standard_tool_plugin::tool::draw_tool::move_draw_tool::MoveDrawTool;
use graphics_rs::standard_tool_plugin::tool::Tool;
use graphics_rs::view_port::ViewPort;
use graphics_rs::standard_rendering_plugin::renderer::{Renderable, Renderer};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}



#[wasm_bindgen]
pub struct Whiteboard {
    id: ElementId,
    view_port: ViewPort,
    active_tool: Option<Box<dyn Tool>>,
}

#[wasm_bindgen]
impl Whiteboard {
    pub fn new() -> Self {
        let owner_id = "asdasd";

        Self {
            id: ElementId::with_owner_id(owner_id),
            view_port: ViewPort::new(ElementId::with_owner_id(owner_id)),
            active_tool: None,
        }
    }

    pub fn activate_rectangle_tool(&mut self) {
        let draw_rectangle_tool = MoveDrawTool::new(
            self.view_port.clone(),
            move || {
                let id: ElementId = ElementId::with_owner_id("weuif");
                RectangleEntity::new(id, Rectangle::zero_sized(Point::default()), ShapeStyle::default())
            }
        );

        self.activate_tool(draw_rectangle_tool);
    }

    pub fn activate_polygon_tool(&mut self) {
        let draw_polygon_tool = ClickDrawTool::new(
            self.view_port.clone(),
            move || {
                let id: ElementId = ElementId::with_owner_id("weuif");
                PolygonEntity::new(id, Polygon::new(&[]), ShapeStyle::default())
            }
        );

        self.activate_tool(draw_polygon_tool);
    }

    pub fn activate_select_tool(&mut self) {
        // self.activate_tool(SelectTool::new(self.view_port.clone()));
    }

    pub fn mouse_down(&mut self, x: f64, y: f64) {
        let Some(active_tool) = &mut self.active_tool else {
            return;
        };

        active_tool.mouse_down(&Point::new(x, y));
    }

    pub fn mouse_move(&mut self, x: f64, y: f64) {
        let Some(active_tool) = &mut self.active_tool else {
            return;
        };

        active_tool.mouse_move(&Point::new(x, y));
    }

    pub fn mouse_up(&mut self, x: f64, y: f64) {
        let Some(active_tool) = &mut self.active_tool else {
            return;
        };

        active_tool.mouse_up(&Point::new(x, y));
    }

    pub fn render_canvas(&self, renderer: &mut CanvasRenderer) {
        renderer.clear();

        self.view_port.render(renderer);

        let Some(active_tool) = &self.active_tool else {
            return;
        };
        active_tool.render(renderer);
    }

    pub fn render_svg(&self, renderer: &mut SVGRenderer) {
        renderer.clear();

        self.view_port.render(renderer);

        let Some(active_tool) = &self.active_tool else {
            return;
        };
        active_tool.render(renderer);
    }

    fn activate_tool(&mut self, tool: impl Tool + 'static) {
        self.active_tool = Some(Box::new(tool));
    }
}
