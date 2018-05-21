use std::sync::Arc;
use vulkano::device::{Device, DeviceExtensions, QueuesIter};
use vulkano::instance::{self, Features, Instance, InstanceExtensions, PhysicalDevice, QueueFamily,
                        debug::DebugCallback};
use vulkano::swapchain::Surface;
use vulkano_win::{self, VkSurfaceBuild};
use winit;

pub fn init_events_loop() -> winit::EventsLoop {
    winit::EventsLoop::new()
}

pub fn init_vulkan(events_loop: &winit::EventsLoop) -> Arc<Surface<winit::Window>> {
    let instance = init_vulkan_instance();
    init_vulkan_debug_callbacks(instance.clone());
    init_device(instance.clone());
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
fn init_vulkan_debug_callbacks(instance: Arc<Instance>) {
    println!("Setting Up Debug Callbacks.");
    DebugCallback::errors_and_warnings(&instance, |msg| {
        println!("Debug callback: {:?}", msg.description);
    }).ok();
}
#[cfg(not(feature = "vk_debug"))]
fn init_vulkan_debug_callbacks(instance: Arc<Instance>) {}

fn init_device(instance: Arc<Instance>) -> (Arc<Device>, QueuesIter) {
    println!("Picking PhysicalDevice");
    let physical_device = instance::PhysicalDevice::enumerate(&instance)
        .find(|&physical_device| is_device_suitable(physical_device))
        .expect("No suitable physical device found!");
    println!("Picking Queue Family");
    let queue_family = physical_device
        .queue_families()
        .find(|qf| is_queue_suitable(qf))
        .expect("No suitable queue family found!");
    let features = Features::none();
    Device::new(
        physical_device,
        &features,
        &init_vulkan_device_extensions(physical_device),
        Some((queue_family, 1.0)),
    ).expect("Couldn't build device")
}

fn is_device_suitable(device: PhysicalDevice) -> bool {
    let minimal_features = Features {
        geometry_shader: true,
        ..Features::none()
    };
    let suitable = device.supported_features().superset_of(&minimal_features);
    if suitable {
        print!("  ✔️ ");
    } else {
        print!("  ❌ ");
    }
    println!(
        "{}, type: {:?}\n      supports: {}, driver: {}",
        device.name(),
        device.ty(),
        device.api_version(),
        device.driver_version(),
    );
    suitable
}

fn is_queue_suitable(queue_family: &QueueFamily) -> bool {
    let suitable = queue_family.supports_graphics();
    if suitable {
        print!("  ✔️ ");
    } else {
        print!("  ❌ ");
    }
    println!(
        "  id: {}, queues_count: {}, graphics: {}, compute: {}, transfers: {}, sparse_binding: {}",
        queue_family.id(),
        queue_family.queues_count(),
        queue_family.supports_graphics(),
        queue_family.supports_compute(),
        queue_family.supports_transfers(),
        queue_family.supports_sparse_binding(),
    );
    suitable
}

#[cfg(feature = "vk_debug")]
fn init_vulkan_device_extensions(physical_device: PhysicalDevice) -> DeviceExtensions {
    println!("Device Extensions:");
    let mut extensions = DeviceExtensions::none();
    extensions.ext_debug_marker = true;
    let supported = DeviceExtensions::supported_by_device(physical_device);
    print!("  ✔️ ");
    println!("{:?}", supported.intersection(&extensions));
    print!("  ❌ ");
    println!("{:?}", supported.difference(&extensions));
    extensions
}
#[cfg(not(feature = "vk_debug"))]
fn init_vulkan_device_extensions() -> DeviceExtensions {
    DeviceExtensions::none()
}
