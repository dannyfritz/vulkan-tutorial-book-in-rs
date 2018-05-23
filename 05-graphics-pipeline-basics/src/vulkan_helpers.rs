use glsl_to_spirv;
use std;
use std::io::Read;
use std::sync::Arc;
use vulkano::device::{Device, DeviceExtensions, Queue, QueuesIter};
use vulkano::image::SwapchainImage;
use vulkano::instance::{self, Features, Instance, InstanceExtensions, PhysicalDevice, QueueFamily,
                        debug::DebugCallback};
use vulkano::pipeline::shader::ShaderModule;
use vulkano::swapchain::{PresentMode, Surface, SurfaceTransform, Swapchain};
use vulkano_shaders::ShaderType;
use vulkano_win::{self, VkSurfaceBuild};
use winit;

pub struct VulkanContext {
    surface: Arc<Surface<winit::Window>>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain<winit::Window>>,
    images: Vec<Arc<SwapchainImage<winit::Window>>>,
}

pub fn init_events_loop() -> winit::EventsLoop {
    winit::EventsLoop::new()
}

pub fn init_vulkan(events_loop: &winit::EventsLoop) -> Box<VulkanContext> {
    let instance = init_vulkan_instance();
    init_vulkan_debug_callbacks(instance.clone());
    let surface = init_window()
        .build_vk_surface(&events_loop, instance.clone())
        .unwrap();
    let (device, mut queues_iter) = init_device(instance.clone(), surface.clone());
    let queue = queues_iter.next().unwrap();
    let (swapchain, images) = init_swapchain(device.clone(), queue.clone(), surface.clone());
    create_pipeline(device.clone());
    Box::new(VulkanContext {
        device,
        queue,
        surface,
        swapchain,
        images,
    })
}

fn init_window() -> winit::WindowBuilder {
    winit::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title("Vulkan")
}

fn init_swapchain(
    device: Arc<Device>,
    queue: Arc<Queue>,
    surface: Arc<Surface<winit::Window>>,
) -> (
    Arc<Swapchain<winit::Window>>,
    Vec<Arc<SwapchainImage<winit::Window>>>,
) {
    let caps = surface
        .capabilities(device.physical_device())
        .expect("failed to get surface capabilities");
    let dimensions = caps.current_extent.unwrap_or([800, 600]);
    let alpha = caps.supported_composite_alpha.iter().next().unwrap();
    let format = caps.supported_formats[0].0;
    Swapchain::new(
        device.clone(),
        surface.clone(),
        caps.min_image_count,
        format,
        dimensions,
        1,
        caps.supported_usage_flags,
        &queue,
        SurfaceTransform::Identity,
        alpha,
        PresentMode::Fifo,
        true,
        None,
    ).expect("failed to create swapchain")
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

fn init_device(
    instance: Arc<Instance>,
    surface: Arc<Surface<winit::Window>>,
) -> (Arc<Device>, QueuesIter) {
    println!("Picking PhysicalDevice");
    let device_extensions = init_vulkan_device_extensions();
    let physical_device = instance::PhysicalDevice::enumerate(&instance)
        .find(|&physical_device| is_device_suitable(physical_device, device_extensions))
        .expect("No suitable physical device found!");
    println!("Picking Queue Family");
    let queue_family = physical_device
        .queue_families()
        .find(|qf| is_queue_suitable(qf, surface.clone()))
        .expect("No suitable queue family found!");
    let features = Features::none();
    Device::new(
        physical_device,
        &features,
        &device_extensions,
        Some((queue_family, 1.0)),
    ).expect("Couldn't build device")
}

fn is_device_suitable(physical_device: PhysicalDevice, extensions: DeviceExtensions) -> bool {
    let minimal_features = Features {
        geometry_shader: true,
        ..Features::none()
    };
    let suitable = physical_device
        .supported_features()
        .superset_of(&minimal_features);
    if suitable {
        print!("  ✔️ ");
    } else {
        print!("  ❌ ");
    }
    println!(
        "{}, type: {:?}\n  supports: {}, driver: {}",
        physical_device.name(),
        physical_device.ty(),
        physical_device.api_version(),
        physical_device.driver_version(),
    );
    println!("  device extensions:");
    let supported = DeviceExtensions::supported_by_device(physical_device);
    print!("    ✔️ ");
    println!("{:?}", supported.intersection(&extensions));
    print!("    ❌ ");
    println!("{:?}", supported.difference(&extensions));
    suitable
}

fn is_queue_suitable(queue_family: &QueueFamily, surface: Arc<Surface<winit::Window>>) -> bool {
    let suitable =
        queue_family.supports_graphics() && surface.is_supported(*queue_family).unwrap_or(false);
    if suitable {
        print!("  ✔️ ");
    } else {
        print!("  ❌ ");
    }
    println!(
        "id: {}, queues_count: {}, graphics: {}, compute: {}, transfers: {}, sparse_binding: {}",
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
fn init_vulkan_device_extensions() -> DeviceExtensions {
    let mut extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };
    extensions.ext_debug_marker = true;
    extensions
}
#[cfg(not(feature = "vk_debug"))]
fn init_vulkan_device_extensions() -> DeviceExtensions {
    DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    }
}

fn create_pipeline(device: Arc<Device>) {
    let vertex_shader = create_shader(device.clone(), "./shaders/shader.vert", ShaderType::Vertex);
    let fragment_shader = create_shader(
        device.clone(),
        "./shaders/shader.frag",
        ShaderType::Fragment,
    );
}

fn create_shader(device: Arc<Device>, path: &str, shader_type: ShaderType) -> Arc<ShaderModule> {
    print!("Compiling {:?} shader from {} ", shader_type, path);
    let source = std::fs::read_to_string(path).expect("Could not read vertex shader file!");
    let mut shader = glsl_to_spirv::compile(source.as_str(), shader_type).unwrap();
    // vulkano_shaders::reflect("Shader", shader).unwrap();
    let mut vertex_bytes: Vec<u8> = Vec::new();
    shader.read_to_end(&mut vertex_bytes).unwrap();
    let shader_module = unsafe {
        ShaderModule::new(device.clone(), &vertex_bytes).expect("failed to create shader module")
    };
    println!("✔️");
    shader_module
}
