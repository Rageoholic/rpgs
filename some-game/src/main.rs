use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, event_loop::EventLoop,
    window::Window,
};

#[derive(Debug)]
enum App {
    Uninit,
    Init { win: Window },
    Destroyed,
}

static DEFAULT_WIDTH: u32 = 1280;
static DEFAULT_HEIGHT: u32 = 720;

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let App::Uninit = self {
            log::trace!("Initializing Application");
            let win = match event_loop.create_window(
                Window::default_attributes()
                    .with_inner_size(LogicalSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT)),
            ) {
                Ok(w) => w,
                Err(e) => {
                    log::error!(target: "main_application", "OSError creating window {}", e);
                    event_loop.exit();
                    return;
                }
            };

            *self = App::Init { win }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let App::Init { win } = self {
            match event {
                WindowEvent::CloseRequested if window_id == win.id() => {
                    *self = App::Destroyed;
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested if window_id == win.id() => {
                    //draw
                }
                _ => {}
            }
        }
    }
    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let App::Init { win } = self {
            win.request_redraw();
        }
    }
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let event_loop = EventLoop::new()?;
    let mut app = App::Uninit;
    event_loop.run_app(&mut app)?;
    Ok(())
}
