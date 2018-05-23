# Presentation

## Notes

### Window Surface

I guess I had already created the surface when I did WindowBuilder.

Added validation for the QueueFamily and Surface check.

Already have my queues from the Device.

### Swap Chain

Woot for `Arc`! Made this simpler than it could have been with the borrow checker.

### Image View

I'm not positive, but I think with `Vulkano` this was returned from `Swapchain::new`.

## Random Thoughts

The tutorial appears to have brought me in a certain direction for validating.
But, then it reversed it and changed fundamentally how it needed to be validated.
All of the `is_suitable` type methods might need a good rework!
Otherwise, it might choose a device that isn't suitable in terms of queues and extensions.