mod element_id;
mod renderer;
mod atomic_view_port;

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
use graphics_rs::standard_tool_plugin::tool::{Interaction, PointingDevice, Tool};
use graphics_rs::standard_rendering_plugin::renderer::{Renderable, Renderer};
use graphics_rs::standard_tool_plugin::tool::select_tool::SelectTool;
use crate::atomic_view_port::AtomicViewPort;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen]
pub struct Whiteboard {
    id: ElementId,
    view_port: AtomicViewPort,
}

#[wasm_bindgen]
impl Whiteboard {
    pub fn new() -> Self {
        let owner_id = "asdasd";

        Self {
            id: ElementId::with_owner_id(owner_id),
            view_port: AtomicViewPort::new(ElementId::with_owner_id(owner_id)),
        }
    }

    pub fn activate_rectangle_tool(&mut self) {
        let draw_rectangle_tool = MoveDrawTool::new(
            move || {
                let id: ElementId = ElementId::with_owner_id("weuif");
                RectangleEntity::with_standard_feature_set(id, Rectangle::zero_sized(Point::default()), ShapeStyle::default())
            }
        );

        self.activate_tool(draw_rectangle_tool);
    }

    pub fn activate_polygon_tool(&mut self) {
        let draw_polygon_tool = ClickDrawTool::new(
            move || {
                let id: ElementId = ElementId::with_owner_id("weuif");
                PolygonEntity::with_standard_feature_set(id, Polygon::new(&[]), ShapeStyle::default())
            }
        );

        self.activate_tool(draw_polygon_tool);
    }

    pub fn activate_select_tool(&mut self) {
        self.activate_tool(SelectTool::new());
    }

    pub fn mouse_down(&self, x: f64, y: f64) {
        let Ok(mut view_port) = self.view_port.write() else {
            return;
        };

        view_port.interaction_event(
            Interaction::PointerDown(Point::new(x, y), PointingDevice::Mouse),
        );
    }
    pub fn mouse_move(&self, x: f64, y: f64) {
        let Ok(mut view_port) = self.view_port.write() else {
            return;
        };

        view_port.interaction_event(
            Interaction::PointerMove(Point::new(x, y), PointingDevice::Mouse),
        );
    }
    pub fn mouse_up(&self, x: f64, y: f64) {
        let Ok(mut view_port) = self.view_port.write() else {
            return;
        };

        view_port.interaction_event(
            Interaction::PointerUp(Point::new(x, y), PointingDevice::Mouse),
        );
    }

    pub fn render_canvas(&self, renderer: &mut CanvasRenderer) {
        let Ok(view_port) = self.view_port.read() else {
            return;
        };

        renderer.clear();
        view_port.render(renderer);
    }

    pub fn render_svg(&self, renderer: &mut SVGRenderer) {
        let Ok(view_port) = self.view_port.read() else {
            return;
        };

        renderer.clear();
        view_port.render(renderer);
    }

    fn activate_tool(&mut self, tool: impl Tool + 'static) {
        let Ok(mut view_port) = self.view_port.write() else {
            return;
        };

        view_port.activate_tool(tool);
    }
}
