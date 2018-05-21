# Setup

## Notes

### Base Code

In C++ they are creating a class to init and start a loop.
We'll be using a struct + impl.

### Instance

For every `vkCreate` that is called, should be a `vkFree`.
Rust handles this for us.
No need to worry about cleanup.
When the pointers go out of scope, the library cleans them up.

Windowing libraries provide methods for getting the [required extensions](https://docs.rs/vulkano-win/0.9.0/vulkano_win/fn.required_extensions.html).
You can also query the driver for what extensions are supported.

### Validation Layers

[Layers in Vulkano](https://docs.rs/vulkano/0.9.0/vulkano/instance/struct.Instance.html#layers).

Validation Layers are useful for:

* Checking values and parameters for valid inputs
* Tracking creation and destruction for resource leaks
* Checking thread safety
* Logging every call and its parameters to stdout
* Tracing Vulkan calls for profiling and replaying

The idea of using validation layers for debug builds and disabling them for release builds is really cool.
Literally lets you control it how you want with a default of nothing.

Guide is using C++'s `#ifdef DEBUG` to do something when in `DEBUG` mode.
Rust doesn't have a direct analog, but can achieve this with `[features]` and `[cfg(feature="vk_debug")]`.
Then if you want to run with `debug` set, `cargo run --features "vk_debug"`

Getting the [`layers_list()`](https://docs.rs/vulkano/0.9.0/vulkano/instance/fn.layers_list.html) into the `Instance::new` method was really painful! 
I mean like really really painful.
https://github.com/vulkano-rs/vulkano/issues/336

Setting up the `DebugCallback` was a breeze with Rust.

### Physical Devices and Queue Familes

## Random Thoughts

Coming back and seeing a much more OOP class approach doesn't feel very good.
While `struct` isn't a huge advantage over `class`, `struct` does feel better overall.

The C++ Vulkan interface is terrible compared to the `Vulkano` one so far!

The challenge was to make sure each extension required is in the supported list.
`Vulkano` made that dead simple with [`difference`](https://docs.rs/vulkano/0.9.0/vulkano/instance/struct.InstanceExtensions.html#method.difference) and [`supported_by_core`](https://docs.rs/vulkano/0.9.0/vulkano/instance/struct.InstanceExtensions.html?search=#method.supported_by_core)