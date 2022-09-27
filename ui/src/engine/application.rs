use std::time::{Duration, Instant};

use super::settings::Settings;
use super::window::{ReRenderEvent, RenderEvent};
// use glutin::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};

pub type AppCreator = Box<dyn FnOnce(&ApplicationContext) -> Box<dyn Application>>;

pub trait Application {
    fn update(&mut self, ctx: &egui::Context);

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()
    }

    fn on_close_event(&self) -> bool {
        false
    }
}

pub struct ApplicationContext {}
