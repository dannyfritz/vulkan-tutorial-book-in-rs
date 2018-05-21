use std::sync::Arc;
use vulkano::instance::{self, Instance, InstanceExtensions, debug::DebugCallback};
use vulkano::swapchain::Surface;
use vulkano_win::{self, VkSurfaceBuild};
use winit;

pub fn init_events_loop() -> winit::EventsLoop {
    winit::EventsLoop::new()
}

pub fn init_vulkan(events_loop: &winit::EventsLoop) -> Arc<Surface<winit::Window>> {
    let instance = init_vulkan_instance();
    init_vulkan_debug_callbacks(&instance.clone());
    init_window()
        .build_vk_surface(&events_loop, instance.clone())
        .unwrap()
}

fn init_window() -> winit::WindowBuilder {
    winit::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title("Vulkan")
}

fn init_vulkan_instance() -> Arc<Instance> {
    Instance::new(
        None,
        &init_vulkan_instance_extensions(),
        //INFO (danny): https://github.com/vulkano-rs/vulkano/issues/336
        init_vulkan_layers()
            .iter()
            .map(|ln| ln.as_str())
            .collect::<Vec<&str>>()
            .iter(),
    ).expect("failed to create Vulkan instance")
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
fn init_vulkan_debug_callbacks(instance: &Arc<Instance>) {
    println!("Setting Up Debug Callbacks.");
    DebugCallback::errors_and_warnings(&instance, |msg| {
        println!("Debug callback: {:?}", msg.description);
    }).ok();
}
#[cfg(not(feature = "vk_debug"))]
fn init_vulkan_debug_callbacks(instance: &Arc<Instance>) {}
