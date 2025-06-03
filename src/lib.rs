mod atomic_view_port;
mod renderer;
mod element;
mod from_js_key;

use crate::atomic_view_port::ViewPort;
use crate::element::Build;
use element::r#type::ElementType;
use geometry::figure::point::Point;
use standard_rendering_plugin::renderer::{Renderable, Renderer};
use standard_tool_plugin::tool::draw_tool::click_draw_tool::ClickDrawTool;
use standard_tool_plugin::tool::draw_tool::move_draw_tool::MoveDrawTool;
use standard_tool_plugin::tool::select_tool::SelectTool;
use standard_tool_plugin::tool::{PointingDevice, Tool};
use standard_tool_plugin::tool::Interaction;
use renderer::canvas_renderer::CanvasRenderer;
use renderer::svg_renderer::SVGRenderer;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::spawn_local;
use crate::from_js_key::from_js_key;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Whiteboard {
    owner_id: String,
    view_port: ViewPort,
    active_tool: Option<Box<dyn Tool>>,
}


#[wasm_bindgen]
impl Whiteboard {
    pub fn new(owner_id: &str) -> Self {
        /* enable panic message printing */
        console_error_panic_hook::set_once();

        Self {
            owner_id: owner_id.to_string(),
            view_port: ViewPort::new(),
            active_tool: None,
        }
    }
}

#[wasm_bindgen]
impl Whiteboard {
    pub fn activate_rectangle_tool(&mut self) {
        let owner_id: String = self.owner_id.clone();

        let rectangle_tool: MoveDrawTool = MoveDrawTool::new(move || Build::default_entity(ElementType::Rectangle, owner_id.clone()));

        let mut view_port: ViewPort = self.view_port.clone();
        let receiver = rectangle_tool.event.end_drawing();

        spawn_local(async move {
            while let Ok(entity) = receiver.recv().await {
                view_port.add_entity(entity).expect("Can't lock view port to add rectangle entity");
            }
        });

        self.active_tool = Some(Box::new(rectangle_tool));
    }

    pub fn activate_polygon_tool(&mut self) {
        let owner_id: String = self.owner_id.clone();

        let polygon_tool: ClickDrawTool = ClickDrawTool::new(move || Build::default_entity(ElementType::Polygon, owner_id.clone()));

        let mut view_port: ViewPort = self.view_port.clone();
        let receiver = polygon_tool.event.end_drawing();

        spawn_local(async move {
            while let Ok(entity) = receiver.recv().await {
                view_port.add_entity(entity).expect("Can't lock view port to add polygon entity");
            }
        });

        self.active_tool = Some(Box::new(polygon_tool));
    }

    pub fn activate_select_tool(&mut self) {
        self.active_tool = Some(Box::new(SelectTool::new()));
    }
}

#[wasm_bindgen]
impl Whiteboard {
    pub fn mouse_down(&mut self, x: f64, y: f64) {
        let Some(active_tool) = &mut self.active_tool else {
            return;
        };

        active_tool.interact(
            Interaction::PointerDown(
                (x, y).into(),
                PointingDevice::Mouse,
            )
        );
    }

    pub fn mouse_move(&mut self, x: f64, y: f64) {
        let Some(active_tool) = &mut self.active_tool else {
            return;
        };

        active_tool.interact(
            Interaction::PointerMove(
                (x, y).into(),
                PointingDevice::Mouse,
            )
        );
    }

    pub fn mouse_up(&mut self, x: f64, y: f64) {
        let Some(active_tool) = &mut self.active_tool else {
            return;
        };

        active_tool.interact(
            Interaction::PointerUp(
                Point::new(x, y),
                PointingDevice::Mouse,
            )
        );
    }

    pub fn key_down(&mut self, key: &str) {
        let Some(key) = from_js_key(key) else {
            return;
        };

        let Some(active_tool) = &mut self.active_tool else {
            return;
        };

        active_tool.interact(Interaction::KeyDown(key));
    }

    pub fn key_up(&mut self, key: &str) {
        let Some(key) = from_js_key(key) else {
            return;
        };

        let Some(active_tool) = &mut self.active_tool else {
            return;
        };

        active_tool.interact(Interaction::KeyUp(key));
    }
}

#[wasm_bindgen]
impl Whiteboard {
    pub fn render_canvas(&self, renderer: &mut CanvasRenderer) {
        self.render(renderer);
    }

    pub fn render_svg(&self, renderer: &mut SVGRenderer) {
        self.render(renderer);
    }

    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.clear();
        self.view_port.render(renderer);

        if let Some(active_tool) = &self.active_tool {
            active_tool.render(renderer);
        }
    }
}
