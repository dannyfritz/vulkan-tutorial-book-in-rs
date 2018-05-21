// #[macro_use]
extern crate vulkano;
// #[macro_use]
// extern crate vulkano_shader_derive;
extern crate vulkano_win;
extern crate winit;

mod vulkan_helpers;

use vulkan_helpers::*;
use vulkano::swapchain::Surface;
use std::sync::Arc;

struct Application {
    events_loop: winit::EventsLoop,
    surface: Arc<Surface<winit::Window>>,
}

impl Application {
    fn new() -> Application {
        let events_loop = init_events_loop();
        let surface = init_vulkan(&events_loop);
        Application {
            events_loop,
            surface,
        }
    }
    fn run(&mut self) {
        self.main_loop();
    }
    fn main_loop(&mut self) {
        loop {
            let mut done = false;
            self.events_loop.poll_events(|ev| match ev {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::Closed,
                    ..
                } => done = true,
                _ => (),
            });
            if done {
                return;
            }
        }
    }
}

fn main() {
    let mut app = Application::new();
    app.run();
}
