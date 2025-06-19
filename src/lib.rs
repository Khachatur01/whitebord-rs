#![feature(extern_types)]

mod view_port;
mod renderer;
mod element;
mod from_js_key;

use crate::element::id::Id;
use crate::element::{Build, ElementType};
use crate::from_js_key::from_js_key;
use crate::renderer::renderer::canvas_renderer::CanvasRenderer;
use crate::renderer::renderer::svg_renderer::SVGRenderer;
use crate::view_port::ViewPort;
use entity_model_feature::entity::Entity;
use event_handler::Receiver;
use geometry::point::point_2d::Point2D;
use standard_rendering_plugin::renderable::Renderable;
use standard_rendering_plugin::renderer::renderer::Renderer;
use standard_tool_plugin::tool::draw_tool::click_draw_tool::ClickDrawTool;
use standard_tool_plugin::tool::draw_tool::move_draw_tool::MoveDrawTool;
use standard_tool_plugin::tool::select_tool::SelectTool;
use standard_tool_plugin::tool::Interaction;
use standard_tool_plugin::tool::{PointingDevice, Tool};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
unsafe extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[inline]
fn listen_async<E: 'static>(receiver: Receiver<E>, mut callback: impl FnMut(E) + 'static) {
    spawn_local(async move {
        while let Ok(event) = receiver.recv().await {
            callback(event);
        }
    });
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
    pub fn activate_move_draw(&mut self, element_type: ElementType) {
        let owner_id: String = self.owner_id.clone();

        let move_draw_tool: MoveDrawTool<Id> = MoveDrawTool::new(move || Build::default(&owner_id, element_type));

        let mut view_port: ViewPort = self.view_port.clone();
        listen_async(move_draw_tool.event.finish_drawing(), move |entity: Entity<Id>| {
            view_port.add_entity(entity).expect("Can't lock view port to add entity");
        });

        self.active_tool = Some(Box::new(move_draw_tool));
    }

    pub fn activate_click_draw(&mut self, element_type: ElementType) {
        let owner_id: String = self.owner_id.clone();

        let click_draw_tool: ClickDrawTool<Id> = ClickDrawTool::new(move || Build::default(&owner_id, element_type));

        let mut view_port: ViewPort = self.view_port.clone();
        listen_async(click_draw_tool.event.finish_drawing(), move |entity: Entity<Id>| {
            view_port.add_entity(entity).expect("Can't lock view port to add entity");
        });

        self.active_tool = Some(Box::new(click_draw_tool));
    }

    pub fn activate_select_tool(&mut self) {
        self.active_tool = Some(Box::new(SelectTool::<Id>::new()));
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
                Point2D::new(x, y),
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
