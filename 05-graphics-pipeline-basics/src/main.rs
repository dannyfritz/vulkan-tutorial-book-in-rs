extern crate vulkano;
extern crate vulkano_shaders;
extern crate glsl_to_spirv;
extern crate vulkano_win;
extern crate winit;

mod vulkan_helpers;

use vulkan_helpers::*;

struct Application {
    events_loop: winit::EventsLoop,
    vulkan_context: Box<VulkanContext>,
}

impl Application {
    fn new() -> Application {
        let events_loop = init_events_loop();
        let vulkan_context = init_vulkan(&events_loop);
        Application {
            events_loop,
            vulkan_context,
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
