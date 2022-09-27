use std::time::Instant;

use super::application;
use super::settings;
use crate::engine::graphics::Graphices;
use crate::engine::{
    application::{AppCreator, Application, ApplicationContext},
    settings::Settings,
};
use winit::event_loop::{EventLoop, EventLoopProxy};
use winit::monitor;
use winit::window::WindowBuilder;
use winit::{self, event_loop::EventLoopWindowTarget};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// pub enum RenderEvent {
//     Wait,
//     ReRender,
//     ReRenderAfter(Instant),
//     Exit,
// }

// pub struct ReRenderEvent;

// pub trait WindowApp {
//     fn is_focused(&self) -> bool;
//     fn window(&self) -> Option<&winit::window::Window>;
//     fn close(&mut self);
//     fn render(&mut self) -> RenderEvent;
//     fn on_event(
//         &mut self,
//         event_loop: &EventLoopWindowTarget<ReRenderEvent>,
//         event: &winit::event::Event<'_, ReRenderEvent>,
//     ) -> RenderEvent;
// }

fn center_window_postion(
    monitor: Option<monitor::MonitorHandle>,
    settings: &mut settings::Settings,
) {
    if let Some(monitor) = monitor {
        let monitor_size = monitor.size();
        let inner_size = settings
            .initial_window_size
            .unwrap_or(egui::Vec2 { x: 800.0, y: 600.0 });
        if monitor_size.width > 0 && monitor_size.height > 0 {
            let x = (monitor_size.width - inner_size.x as u32) / 2;
            let y = (monitor_size.height - inner_size.y as u32) / 2;
            settings.initial_window_postion = Some(egui::Pos2 {
                x: x as _,
                y: y as _,
            });
        }
    }
}

// fn window_builder(settings: &settings::Settings) -> winit::window::WindowBuilder {
//     let mut window_builder = winit::window::WindowBuilder::new()
//         .with_always_on_top(true)
//         .with_decorations(true)
//         .with_fullscreen(
//             settings
//                 .fullscreen
//                 .then(|| winit::window::Fullscreen::Borderless(None)),
//         )
//         .with_maximized(settings.maximized)
//         .with_resizable(settings.resizable);

//     #[cfg(target_os = "macos")]
//     if *fullsize_content {
//         window_builder = window_builder
//             .with_title_hidden(true)
//             .with_titlebar_transparent(true)
//             .with_fullsize_content_view(true);
//     }
//     if let Some(pos) = settings.initial_window_postion {
//         window_builder = window_builder.with_position(winit::dpi::PhysicalPosition {
//             x: pos.x as f64,
//             y: pos.y as f64,
//         });
//     }
//     if let Some(initial_window_size) = settings.initial_window_size {
//         window_builder = window_builder.with_inner_size(points_to_size(initial_window_size));
//     }
//     window_builder
// }

// pub fn points_to_size(points: egui::Vec2) -> winit::dpi::LogicalSize<f64> {
//     winit::dpi::LogicalSize {
//         width: points.x as f64,
//         height: points.y as f64,
//     }
// }

// mod render {

//     use super::*;
//     use std::sync::Arc;

//     struct GlowRenderRuntime {
//         gl: Arc<egui_glow::glow::Context>,
//         painter: egui_glow::Painter,
//         app: Box<dyn Application>,
//         gl_ctx: glutin::WindowedContext<glutin::PossiblyCurrent>,
//         graphics: Graphices,
//     }

//     struct GlowRenderCompositor {
//         rerender_proxy: Arc<egui::mutex::Mutex<EventLoopProxy<ReRenderEvent>>>,
//         app_name: String,
//         settings: Settings,
//         runtime: Option<GlowRenderRuntime>,
//         app_creator: Option<application::AppCreator>,
//         is_focused: bool,
//     }

//     impl GlowRenderCompositor {
//         pub fn new(
//             event_loop: &EventLoop<ReRenderEvent>,
//             app_name: &str,
//             settings: Settings,
//             app_creator: application::AppCreator,
//         ) -> Self {
//             Self {
//                 rerender_proxy: Arc::new(egui::mutex::Mutex::new(event_loop.create_proxy())),
//                 app_name: app_name.into(),
//                 settings,
//                 runtime: None,
//                 app_creator: Some(app_creator),
//                 is_focused: true,
//             }
//         }

//         fn create_context(
//             event_loop: &EventLoopWindowTarget<ReRenderEvent>,
//             title: &String,
//             settings: &Settings,
//         ) -> (
//             glutin::WindowedContext<glutin::PossiblyCurrent>,
//             egui_glow::glow::Context,
//         ) {
//             let window_builder = window_builder(settings).with_title(title);
//             let gl_ctx = unsafe {
//                 glutin::ContextBuilder::new()
//                     .with_hardware_acceleration(Some(true))
//                     .with_depth_buffer(0)
//                     .with_multisampling(0)
//                     .with_srgb(true)
//                     .with_vsync(false)
//                     .build_windowed(window_builder, event_loop)
//                     .unwrap()
//                     .make_current()
//                     .unwrap()
//             };
//             let gl = unsafe {
//                 egui_glow::glow::Context::from_loader_function(|s| gl_ctx.get_proc_address(s))
//             };
//             (gl_ctx, gl)
//         }

//         fn with_runtime(&mut self, event_loop: &EventLoopWindowTarget<ReRenderEvent>) {
//             let (gl_ctx, gl) = Self::create_context(event_loop, &self.app_name, &self.settings);
//             gl_ctx.window().set_ime_allowed(true);
//             let gl = Arc::new(gl);
//             let painter = egui_glow::Painter::new(gl.clone(), None, "")
//                 .unwrap_or_else(|error| panic!("Some OpenGL error occurred {}\n", error));
//             let graphics = Graphices::new(
//                 event_loop,
//                 painter.max_texture_side(),
//                 gl_ctx.window(),
//                 Some(gl.clone()),
//             );
//             {
//                 let event_loop_proxy = self.rerender_proxy.clone();
//                 graphics.ctx.set_request_repaint_callback(move || {
//                     event_loop_proxy.lock().send_event(ReRenderEvent).ok();
//                 });
//             }
//             let app_creator =
//                 std::mem::take(&mut self.app_creator).expect("App only support call once");
//             let app = app_creator(&ApplicationContext {});

//             self.runtime = Some(GlowRenderRuntime {
//                 gl_ctx,
//                 gl,
//                 painter,
//                 app,
//                 graphics,
//             })
//         }
//     }

//     impl WindowApp for GlowRenderCompositor {
//         fn is_focused(&self) -> bool {
//             self.is_focused
//         }

//         fn window(&self) -> Option<&winit::window::Window> {
//             self.runtime.as_ref().map(|rt| rt.gl_ctx.window())
//         }

//         fn close(&mut self) {
//             if let Some(mut rt) = self.runtime.take() {
//                 rt.painter.destroy();
//             }
//         }

//         fn render(&mut self) -> RenderEvent {
//             if let Some(rt) = &mut self.runtime {
//                 let window = rt.gl_ctx.window();
//                 let screen_size_in_pixels: [u32; 2] = window.inner_size().into();
//                 egui_glow::painter::clear(
//                     &rt.gl,
//                     screen_size_in_pixels,
//                     rt.app.clear_color(&rt.graphics.ctx.style().visuals),
//                 );
//                 let full_output = rt.graphics.update(rt.app.as_mut(), window);
//                 let clipped_primitives = rt.graphics.ctx.tessellate(full_output.shapes);
//                 rt.painter.paint_and_update_textures(
//                     screen_size_in_pixels,
//                     rt.graphics.ctx.pixels_per_point(),
//                     &clipped_primitives,
//                     &full_output.textures_delta,
//                 );
//                 rt.gl_ctx.swap_buffers().unwrap();

//                 let control_flow = if rt.graphics.is_close() {
//                     RenderEvent::Exit
//                 } else if full_output.repaint_after.is_zero() {
//                     RenderEvent::ReRender
//                 } else if let Some(render_after_instant) =
//                     std::time::Instant::now().checked_add(full_output.repaint_after)
//                 {
//                     RenderEvent::ReRenderAfter(render_after_instant)
//                 } else {
//                     RenderEvent::Wait
//                 };
//                 if !self.is_focused {
//                     std::thread::sleep(std::time::Duration::from_millis(10));
//                 }
//                 control_flow
//             } else {
//                 RenderEvent::Wait
//             }
//         }

//         fn on_event(
//             &mut self,
//             event_loop: &EventLoopWindowTarget<ReRenderEvent>,
//             event: &winit::event::Event<'_, ReRenderEvent>,
//         ) -> RenderEvent {
//             match event {
//                 winit::event::Event::Resumed => {
//                     if self.runtime.is_none() {
//                         self.with_runtime(event_loop)
//                     }
//                     RenderEvent::ReRender
//                 }
//                 winit::event::Event::Suspended => RenderEvent::Wait,
//                 winit::event::Event::WindowEvent { event, .. } => {
//                     if let Some(rt) = &mut self.runtime {
//                         match &event {
//                             winit::event::WindowEvent::Focused(foc) => {
//                                 self.is_focused = *foc;
//                             }
//                             winit::event::WindowEvent::Resized(phy_size) => {
//                                 if phy_size.width > 0 && phy_size.height > 0 {
//                                     rt.gl_ctx.resize(*phy_size);
//                                 }
//                             }
//                             winit::event::WindowEvent::ScaleFactorChanged {
//                                 new_inner_size,
//                                 ..
//                             } => {
//                                 rt.gl_ctx.resize(**new_inner_size);
//                             }
//                             winit::event::WindowEvent::CloseRequested if rt.graphics.is_close() => {
//                                 return RenderEvent::Exit;
//                             }
//                             _ => {}
//                         }
//                         let event_reponse = rt.graphics.on_event(rt.app.as_mut(), event);
//                         if rt.graphics.is_close() {
//                             RenderEvent::Exit
//                         } else if event_reponse {
//                             RenderEvent::ReRender
//                         } else {
//                             RenderEvent::Wait
//                         }
//                     } else {
//                         RenderEvent::Wait
//                     }
//                 }
//                 _ => RenderEvent::Wait,
//             }
//         }
//     }
//     // pub fn compositor(
//     //     event_loop: &EventLoop<ReRenderEvent>,
//     //     app_name: &str,
//     //     settings: Settings,
//     //     app_creator: application::AppCreator,
//     // ) -> impl WindowApp + 'static {
//     //     GlowRenderCompositor::new(event_loop, app_name, settings, app_creator)
//     // }
// }
// // export
// // pub use render::compositor;

// fn run_loop(
//     event_loop: EventLoop,
//     mut winit_app: impl super::window::WindowApp + 'static,
// ) -> ! {
//     let mut next_rerender_time = Instant::now();
//     event_loop.run(move |event, event_loop, control_flow| {
//         let event_result = match event {
//             winit::event::Event::LoopDestroyed => RenderEvent::Exit,
//             winit::event::Event::RedrawEventsCleared if cfg!(windows) => {
//                 next_rerender_time = Instant::now() + Duration::from_secs(1_000_000_000);
//                 winit_app.render()
//             }
//             winit::event::Event::RedrawRequested(_) if cfg!(windows) => {
//                 next_rerender_time = Instant::now() + Duration::from_secs(1_000_000_000);
//                 winit_app.render()
//             }
//             winit::event::Event::UserEvent(ReRenderEvent)
//             | winit::event::Event::NewEvents(winit::event::StartCause::ResumeTimeReached {
//                 ..
//             }) => RenderEvent::ReRender,
//             event => winit_app.on_event(event_loop, &event),
//         };
//         match event_result {
//             RenderEvent::Wait => {}
//             RenderEvent::ReRender => {
//                 next_rerender_time = Instant::now();
//             }
//             RenderEvent::ReRenderAfter(rerender_time) => {
//                 next_rerender_time = next_rerender_time.min(rerender_time);
//             }
//             RenderEvent::Exit => {
//                 winit_app.close();
//                 std::process::exit(0);
//             }
//         }
//         *control_flow = match next_rerender_time.checked_duration_since(Instant::now()) {
//             None => {
//                 if let Some(window) = winit_app.window() {
//                     window.request_redraw();
//                 }
//                 ControlFlow::Poll
//             }
//             Some(time_until_next_rerender) => {
//                 ControlFlow::WaitUntil(Instant::now() + time_until_next_rerender)
//             }
//         }
//     })
// }

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run_app(app_name: &str, mut settings: Settings, app_creator: AppCreator) {
    // let event_loop = EventLoopBuilder::<ReRenderEvent>::with_user_event().build();
    // super::window::center_window_postion(event_loop.available_monitors().next(), &mut settings);
    // let app = super::window::compositor(&event_loop, app_name, settings, app_creator);
    // start(event_loop, app)
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
    let event_loop = EventLoop::new();
    super::window::center_window_postion(event_loop.available_monitors().next(), &mut settings);
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450, 400));
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("main")?;
                let canvs = web_sys::Element::from(window.canvas());
                dst.append_child(&canvs).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.")
    }
    // run_loop(event_loop);
}
