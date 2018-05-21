// #[macro_use]
extern crate vulkano;
// #[macro_use]
// extern crate vulkano_shader_derive;
extern crate vulkano_win;
extern crate winit;

use std::sync::Arc;
use vulkano::instance::{self, Instance, InstanceExtensions, debug::DebugCallback};
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
            Instance::new(
                None,
                &Application::init_vulkan_instance_extensions(),
                //INFO (danny): https://github.com/vulkano-rs/vulkano/issues/336
                Application::init_vulkan_layers()
                    .iter()
                    .map(|ln| ln.as_str())
                    .collect::<Vec<&str>>()
                    .iter(),
            ).expect("failed to create Vulkan instance")
        };
        Application::init_vulkan_debug_callbacks(&instance.clone());
        Application::init_window()
            .build_vk_surface(&events_loop, instance.clone())
            .unwrap()
    }
    #[cfg(feature = "vk_debug")]
    fn init_vulkan_layers() -> Vec<String> {
        println!("Layers:");
        instance::layers_list()
            .unwrap()
            .filter(|layer| {
                let name = layer.name();
                let to_activate = name.contains("RENDERDOC") || name.contains("LUNARG");
                if to_activate {
                    print!("  ✔️ ");
                } else {
                    print!("  ❌ ");
                }
                println!(
                    "{} @ {} - {}",
                    layer.name(),
                    layer.implementation_version(),
                    layer.description()
                );
                to_activate
            })
            .map(|l| String::from(l.name()))
            .collect()
    }
    #[cfg(not(feature = "vk_debug"))]
    fn init_vulkan_layers() -> Vec<String> {
        vec![]
    }
    #[cfg(feature = "vk_debug")]
    fn init_vulkan_instance_extensions() -> InstanceExtensions {
        println!("Instance Extensions:");
        let mut extensions = vulkano_win::required_extensions();
        extensions.ext_debug_report = true;
        let supported = InstanceExtensions::supported_by_core().unwrap();
        print!("  ✔️ ");
        println!("{:?}", supported.intersection(&extensions));
        print!("  ❌ ");
        println!("{:?}", supported.difference(&extensions));
        extensions
    }
    #[cfg(not(feature = "vk_debug"))]
    fn init_vulkan_instance_extensions() -> InstanceExtensions {
        vulkano_win::required_extensions()
    }
    #[cfg(feature = "vk_debug")]
    fn init_vulkan_debug_callbacks(instance: &Arc<Instance>) {
        println!("Setting Up Debug Callbacks.");
        DebugCallback::errors_and_warnings(&instance, |msg| {
            println!("Debug callback: {:?}", msg.description);
        }).ok();
    }
    #[cfg(not(feature = "vk_debug"))]
    fn init_vulkan_debug_callbacks(instance: &Arc<Instance>) {}
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
