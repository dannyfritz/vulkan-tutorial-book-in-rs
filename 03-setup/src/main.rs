#[macro_use]
extern crate vulkano;
// #[macro_use]
// extern crate vulkano_shader_derive;
extern crate vulkano_win;
extern crate winit;

use vulkano::instance::{self, Instance, InstanceExtensions, LayersIterator};
use vulkano::swapchain::Surface;
use vulkano_win::VkSurfaceBuild;

struct Application {
    events_loop: winit::EventsLoop,
    surface: std::sync::Arc<Surface<winit::Window>>,
}

impl Application {
    fn new() -> Application {
        let events_loop = Application::init_events_loop();
        let surface = Application::init_vulkan(&events_loop);
        Application {
            events_loop,
            surface,
        }
    }
    fn init_window() -> winit::WindowBuilder {
        winit::WindowBuilder::new()
            .with_dimensions(800, 600)
            .with_title("Vulkan")
    }
    fn init_events_loop() -> winit::EventsLoop {
        winit::EventsLoop::new()
    }
    fn init_vulkan(
        events_loop: &winit::EventsLoop,
    ) -> std::sync::Arc<vulkano::swapchain::Surface<winit::Window>> {
        let instance = {
            let extensions = vulkano_win::required_extensions();
            let supported = InstanceExtensions::supported_by_core().unwrap();
            println!("Instance Extensions:");
            println!("  required:  {:?}", extensions);
            println!("  supported: {:?}", supported);
            println!("  unused:    {:?}", supported.difference(&extensions));
            //INFO (danny): https://github.com/vulkano-rs/vulkano/issues/336
            let layers = Application::init_vulkan_layers();
            let layers: Vec<&str> = layers.iter().map(|ln| ln.as_str()).collect();
            Instance::new(None, &extensions, layers.iter())
                .expect("failed to create Vulkan instance")
        };
        Application::init_window()
            .build_vk_surface(&events_loop, instance.clone())
            .unwrap()
    }
    #[cfg(feature = "vk_debug")]
    fn init_vulkan_layers() -> Vec<String> {
        println!("Available layers:");
        instance::layers_list().unwrap().for_each(|layer| {
            println!(
                "  {}@{} - {}",
                layer.name(),
                layer.implementation_version(),
                layer.description()
            );
        });
        println!("Activated layers:");
        instance::layers_list()
            .unwrap()
            .filter(|l| l.name().contains("RENDERDOC") || l.name().contains("LUNARG"))
            .for_each(|layer| {
                println!(
                    "  {}@{} - {}",
                    layer.name(),
                    layer.implementation_version(),
                    layer.description()
                );
            });
        instance::layers_list()
            .unwrap()
            .filter(|l| l.name().contains("RENDERDOC") || l.name().contains("LUNARG"))
            .map(|l| String::from(l.name()))
            .collect()
    }
    #[cfg(not(feature = "vk_debug"))]
    fn init_vulkan_layers() -> Vec<String> {
        vec![]
    }
    fn run(&mut self) {
        self.main_loop();
        self.cleanup();
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
    fn cleanup(&mut self) {}
}

fn main() {
    let mut app = Application::new();
    app.run();
}
