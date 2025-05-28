mod atomic_view_port;
mod renderer;
mod element;

use crate::atomic_view_port::AtomicViewPort;
use crate::element::Build;
use element::r#type::ElementType;
use graphics_rs::geometry::figure::point::Point;
use graphics_rs::standard_rendering_plugin::renderer::{Renderable, Renderer};
use graphics_rs::standard_tool_plugin::tool::draw_tool::click_draw_tool::ClickDrawTool;
use graphics_rs::standard_tool_plugin::tool::draw_tool::move_draw_tool::MoveDrawTool;
use graphics_rs::standard_tool_plugin::tool::select_tool::SelectTool;
use graphics_rs::standard_tool_plugin::tool::{Interaction, PointingDevice, Tool};
use renderer::canvas_renderer::CanvasRenderer;
use renderer::svg_renderer::SVGRenderer;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Whiteboard {
    owner_id: String,
    view_port: AtomicViewPort,
}


#[wasm_bindgen]
impl Whiteboard {
    pub fn new(owner_id: &str) -> Self {
        /* enable panic message printing */
        console_error_panic_hook::set_once();

        Self {
            owner_id: owner_id.to_string(),
            view_port: AtomicViewPort::new(),
        }
    }
}

#[wasm_bindgen]
impl Whiteboard {
    pub fn activate_rectangle_tool(&mut self) {
        let owner_id: String = self.owner_id.clone();

        self.activate_tool(
            MoveDrawTool::new(move || Build::default_entity(ElementType::Rectangle, owner_id.clone())),
            // MoveDrawTool::new(move || Build::entity_from_json("{}")),
        );
    }

    pub fn activate_polygon_tool(&mut self) {
        let owner_id: String = self.owner_id.clone();

        self.activate_tool(
            ClickDrawTool::new(move || Build::default_entity(ElementType::Polygon, owner_id.clone())),
            // ClickDrawTool::new(move || Build::entity_from_json("{}")),
        );
    }

    pub fn activate_select_tool(&mut self) {
        self.activate_tool(SelectTool::new());
    }

    fn activate_tool(&mut self, tool: impl Tool + 'static) {
        let Ok(mut view_port) = self.view_port.write() else {
            return;
        };

        view_port.activate_tool(tool);
    }
}

#[wasm_bindgen]
impl Whiteboard {
    pub fn mouse_down(&self, x: f64, y: f64) {
        let Ok(mut view_port) = self.view_port.write() else {
            return;
        };

        view_port.interaction_event(Interaction::PointerDown(
            Point::new(x, y),
            PointingDevice::Mouse,
        ));
    }
    pub fn mouse_move(&self, x: f64, y: f64) {
        let Ok(mut view_port) = self.view_port.write() else {
            return;
        };

        view_port.interaction_event(Interaction::PointerMove(
            Point::new(x, y),
            PointingDevice::Mouse,
        ));
    }
    pub fn mouse_up(&self, x: f64, y: f64) {
        let Ok(mut view_port) = self.view_port.write() else {
            return;
        };

        view_port.interaction_event(Interaction::PointerUp(
            Point::new(x, y),
            PointingDevice::Mouse,
        ));
    }
}

#[wasm_bindgen]
impl Whiteboard {
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
}
