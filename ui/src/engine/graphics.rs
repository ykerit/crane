use egui_glow::glow;
use egui_winit::winit;
use glutin::event_loop::EventLoopWindowTarget;

use super::{application::Application, window::ReRenderEvent};

pub struct Graphices {
    pub ctx: egui::Context,
    pending_full_output: egui::FullOutput,
    egui_winit: egui_winit::State,
    close: bool,
}

impl Graphices {
    pub fn new(
        event_loop: &EventLoopWindowTarget<ReRenderEvent>,
        max_texture_side: usize,
        window: &winit::window::Window,
        gl: Option<std::sync::Arc<glow::Context>>,
    ) -> Self {
        let ctx = egui::Context::default();
        let mut egui_winit = egui_winit::State::new(event_loop);
        egui_winit.set_max_texture_side(max_texture_side);
        let pixels_per_point = window.scale_factor() as f32;
        egui_winit.set_pixels_per_point(pixels_per_point);
        Self {
            ctx,
            pending_full_output: Default::default(),
            close: false,
            egui_winit,
        }
    }

    pub fn warm_up(&mut self, app: &mut dyn Application, window: &winit::window::Window) {
        let saved_memory = self.ctx.memory().clone();
        self.ctx.memory().set_everything_is_visible(true);
        let full_output = self.update(app, window);
        self.pending_full_output.append(full_output);
        *self.ctx.memory() = saved_memory;
        self.ctx.clear_animations();
    }

    pub fn update(
        &mut self,
        app: &mut dyn Application,
        window: &winit::window::Window,
    ) -> egui::FullOutput {
        let frame_start = std::time::Instant::now();
        let raw_input = self.egui_winit.take_egui_input(window);
        let full_output = self.ctx.run(raw_input, |ctx| {
            app.update(ctx);
        });
        self.pending_full_output.append(full_output);
        let full_output = std::mem::take(&mut self.pending_full_output);
        full_output
    }

    pub fn is_close(&self) -> bool {
        self.close
    }

    pub fn on_event(&mut self, app: &mut dyn Application, event: &winit::event::WindowEvent<'_>) -> bool {
        use winit::event::{ElementState, MouseButton, WindowEvent};

        match event {
            WindowEvent::CloseRequested => self.close = app.on_close_event(),
            WindowEvent::Destroyed => self.close = true,
            _ => {}
        }
        self.egui_winit.on_event(&self.ctx, event)
    }
}
