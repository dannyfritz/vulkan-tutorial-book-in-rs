# Overview

## Notes

### What is Takes to Draw a Triangle

The book is covering the stages of a Vulkan program.

Roughly, it is grouping the API into 8 discrete stages:

0. Physical Device Selection
0. Logical Device and Queue Family
0. Window Surface and Swap Chain
0. Image Views and Framebuffers
0. Render Passes
0. Graphics Pipeline
0. Command Pools and Command Buffers
0. Main Loop

In more Vulkan detail, it went on to list these API calls:

0. Create a VkInstance
0. Select a supported Graphics Card, VkPhysicalDevice
0. Create a VkDevice and VkQueue
0. Create a Window, Window Surface, and SwapChain
0. Wrap SwapChain Images in a VkImageView
0. Create a render pass that specifies render targets and usage
0. Create framebuffers for render pass
0. Allocate and record a command buffer for each possible SwapChain Image
0. Draw frames by acquiring Images, submitting the right draw command buffer and returning the images to the SwapChain

### API Concepts

> I'm wondering if this Vulkan SDK is important to me or I can skip it.

## Random Thoughts

Searching through crates.io, I see 2 prominent Vulkan crates: Vulkano and Ash.
They are both thin wrappers, but Vulkano tries to be more Rust-like.
Ash is a thin wrapper and provides no ergonomics on top of Vulkan.
They are both made by prominent members of the Rust graphics community.


It is very tempting to pick up Ash, but I'm going to go with Vulkano for now.
As this progresses, I might revisit this decision.


One thing that is really cool about the Vulkan API as opposed to OpenGL is that it provides a low-level interface of essentially primitives.
OpenGL always tried to be a general purpose graphics library, but as more and more use cases in graphics appeared, the less and less capable OpenGL was at delivering without writing what looks like hacks and workarounds.
The advent of OpenGL shader programs is fantastic and a huge step in the right direction, but Vulkan is essentially jumping straight to the goal line on what the driver should be.
You can, and I'm sure we'll start seeing, implementations of OpenGL written on top of Vulkan instead of implemented in a graphics driver.


There are already implementations of Vulkan and OpengL on top of Metal.
Metal and DirectX 12 are OS specific implementations of a low level graphics API by OSX and Microsoft.
They were written to provide a need for next generation graphics.
This need was more control over optimizations, threading, and the graphics pipeline.


Khronos is the standards for OpengL body and saw this.
Khronos jumped on the bandwagon and announced Vulkan, a low level API.
I believe Vulkan is largely based off of AMD's Mantle.


No more will graphics driver writers try to profile and optimize its Vulkan driver for every game in existence like what is done for OpenGL.
OpenCL is even being merged into Vulkan.
No more writing games in both DirectX and OpenGL.
We finally have a unifying low level API that can be used across the board.

This is why I am learning Vulkan.