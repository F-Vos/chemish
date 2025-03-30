# Chemish - A tiny 3D game engine in rust

A tiny 3D game engine made in rust. Using ash.


## Documentation
!note: This is my current understanding and may be inaccurate.

### Window creation
This game engine currently uses GLFW for window creation.

### Vulkan Instance
A Vulkan instance must be created upon startup and destroyed when closing the game. We use the Vulkan SDK to create this instance. We check for the available functionality on the host device and determine which version of Vulkan is available.

### Validation Layers
Vulkan comes with no logging by default. To understand what's happening, you need to implement validation layers. They also help prevent errors or unintended behaviors, but my understanding of this is limited. When shipping a release build, validation layers can be omitted for improved performance.

### Physical device creation
This provides a read-only view of the GPU. We can select a GPU based on preferences and validate that the GPU supports all required layers/extensions.

### Logical device creation
Since we can't directly manipulate the physical device, we create a logical device abstraction to interact with the GPU.

### Surface
The surface is letting the Vulkan API know about GLFW.

### Swapchain
A swap chain manages multiple frames that rotate between rendering and presenting to the screen. While the next frame is being rendered, the previous frame is displayed. Common implementations include double-buffering (two frames) or triple-buffering (three frames). A poor implementation can lead to visual artifacts such as screen tearing. There are a couple of present modes possible with Vulkan like FIFO and Mailbox.