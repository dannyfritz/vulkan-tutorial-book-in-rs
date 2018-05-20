#[macro_use]
extern crate vulkano;
// #[macro_use]
// extern crate vulkano_shader_derive;
extern crate vulkano_win;
extern crate winit;

use vulkano::instance::Instance;
use vulkano_win::VkSurfaceBuild;

fn main() {
    println!("Hello, world!");
    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
    };
    let mut events_loop = winit::EventsLoop::new();
    let surface = winit::WindowBuilder::new()
        .build_vk_surface(&events_loop, instance.clone())
        .unwrap();
    loop {
        let mut done = false;
        events_loop.poll_events(|ev| match ev {
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
 