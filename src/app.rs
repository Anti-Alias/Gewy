use wgpu::*;
use winit::window::WindowBuilder;
use winit::event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::{create_pipeline, Mesh, Painter, Color, GpuMesh};

pub struct App {
    _state: State
}

/// Stores the application in a window.
impl App {
    
    /// Starts the application in a window with the resolution specified.
    pub fn start(width: u32, height: u32) {
        
        // Opens window and handle high-level event
        let size = PhysicalSize::new(width, height);
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap();

        // Creates application state
        let mut state = pollster::block_on(State::new(window));

        // Runs event loop
        event_loop.run(move |event, _, flow| {
            match event {
                Event::WindowEvent { event, .. } => Self::handle_window_event(event, &mut state, flow),
                Event::RedrawRequested( .. ) => Self::handle_redraw_event(&mut state, flow),
                Event::MainEventsCleared => { state.window().request_redraw() }
                _ => {}
            }
        });
    }

    // Handle window-related events.
    fn handle_window_event(event: WindowEvent<'_>, state: &mut State, flow: &mut ControlFlow) {
        if state.input(&event) { return }
        match event {
            WindowEvent::Resized(size) => state.resize(size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => state.resize(*new_inner_size),
            WindowEvent::CloseRequested => *flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
                ..
            } => *flow = ControlFlow::Exit,
            _ => {}
        }
    }

    // Handle redraw events events.
    fn handle_redraw_event(state: &mut State, flow: &mut ControlFlow) {
        state.update();
        match state.render() {
            Ok(_) => {}
            Err(SurfaceError::Lost) => state.resize(state.size),
            Err(SurfaceError::OutOfMemory) => {
                log::error!("WGPU ran out of memory");
                *flow = ControlFlow::Exit
            },
            Err(e) => log::error!("WGPU error: {:?}", e),
        }
    }
}


struct State {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    render_pipeline: RenderPipeline,
    mesh: GpuMesh
}

impl State {
    
    async fn new(window: Window) -> Self {
       
        // WGPU instance
        let size = window.inner_size();
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });
        
        // Surface and adapter
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance.request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        // Device and queue
        let (device, queue) = adapter.request_device(
            &DeviceDescriptor {
                features: Features::empty(),
                label: None,
                ..Default::default()
            },
            None,
        ).await.unwrap();

        // Configures surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
            .unwrap_or(surface_caps.formats[0]);
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // Builds render pipeline
        let render_pipeline = create_pipeline(&device, surface_format);

        // Creates a mesh and paints a quad to it
        let mut mesh = Mesh::new();
        let mut painter = Painter::new(&mut mesh);
        let v = 0.1;
        painter
            .set_color(Color::BLUE)
            .quad([[-v, -v], [v, -v], [v, v], [-v, v]]);
        let mesh = mesh.to_gpu(&device);

        // Done
        Self { window, surface, device, queue, config, size, render_pipeline, mesh }
    }

    pub fn window(&self) -> &Window { &self.window }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {

    }

    fn render(&mut self) -> Result<(), SurfaceError> {

        // Gets surface texture
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());

        // Encodes render pass
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK.into()),
                    store: true
                },
            })],
            depth_stencil_attachment: None
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.mesh.vertices.slice(..));
        render_pass.set_index_buffer(self.mesh.indices.slice(..), IndexFormat::Uint16);
        render_pass.draw(0..3, 0..1);
        drop(render_pass);

        // Submits encoded draw calls
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}