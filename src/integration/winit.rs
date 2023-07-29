use glam::Vec2;
use wgpu::*;
use winit::window::{WindowBuilder, CursorIcon};
use winit::event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode, MouseButton};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::{create_pipeline, Color, Gewy, Painter};

pub struct WinitApp {
    pub width: u32,
    pub height: u32,
    pub gewy: Gewy,
    pub debug: bool,
    pub samples_per_pixel: u32
}

/// Stores the application in a window.
impl WinitApp {

    pub fn new(gewy: Gewy, width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            gewy,
            debug: false,
            samples_per_pixel: 8
        }
    }

    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    pub fn with_sampless_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn start(self) -> ! {
        
        // Opens window and handle high-level events
        let Self { width, height, gewy, debug, samples_per_pixel } = self;
        let size = PhysicalSize::new(width, height);
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap();
        let mut state = pollster::block_on(State::new(window, gewy, debug, samples_per_pixel));

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
    gewy: Gewy,
    painter: Painter,
    msaa_texture_view: Option<TextureView>,
    samples_per_pixel: u32
}

impl State {
   
    async fn new(window: Window, gewy: Gewy, debug: bool, samples_per_pixel: u32) -> Self {
       
        // WGPU instance
        let window_size = window.inner_size();
        let instance = Instance::new(InstanceDescriptor::default());
        
        // Surface and adapter
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance.request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        // Device and queue
        let (device, queue) = adapter.request_device(
            &DeviceDescriptor {
                features: Self::features(debug),
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
            width: window_size.width,
            height: window_size.height,
            present_mode: PresentMode::Fifo,
            alpha_mode: CompositeAlphaMode::Opaque,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // Builds render pipeline
        let render_pipeline = create_pipeline(&device, surface_format, debug, samples_per_pixel);

        // Creates a mesh/gpu mesh.
        let s = Vec2::new(window_size.width as f32, window_size.height as f32);
        let painter = Painter::new(&device, s, gewy.translation, gewy.scale);

        let msaa_texture = if samples_per_pixel == 0 { None } else {
            Self::create_msaa_texture_view(&device, window_size, surface_format, samples_per_pixel);
            None
        };

        // Done
        Self {
            window,
            surface,
            device,
            queue,
            config,
            size: window_size,
            render_pipeline,
            gewy,
            painter,
            msaa_texture_view: msaa_texture,
            samples_per_pixel
        }
    }

    pub fn window(&self) -> &Window { &self.window }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        let size = Vec2::new(new_size.width as f32, new_size.height as f32);
        self.gewy.resize(size);
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
        self.painter.resize(size, self.gewy.translation, self.gewy.scale, &self.device, &self.queue);
        if self.samples_per_pixel != 1 {
            self.msaa_texture_view = Some(Self::create_msaa_texture_view(&self.device, self.size, self.config.format, self.samples_per_pixel));
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorEntered { .. } => {
                let result = self.gewy.mapping().enter_cursor();
                if let Err(err) = result {
                    eprintln!("WindowEvent::CursorEntered caused an error: {}", err);
                }
            },
            WindowEvent::CursorLeft { .. } => {
                let result = self.gewy.mapping().exit_cursor();
                if let Err(err) = result {
                    eprintln!("WindowEvent::CursorLeft caused an error: {}", err);
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                let position = Vec2::new(position.x as f32, position.y as f32);
                let result = self.gewy.mapping().move_cursor(position);
                if let Err(err) = result {
                    eprintln!("WindowEvent::CursorMoved caused an error: {}", err);
                }
            },
            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                let result = self.gewy.mapping().press(crate::MouseButton::Left);
                if let Err(err) = result {
                    eprintln!("WindowEvent::MouseInput caused an error: {}", err);
                }
            },
            WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. } => {
                let result = self.gewy.mapping().release(crate::MouseButton::Left);
                if let Err(err) = result {
                    eprintln!("WindowEvent::MouseInput caused an error: {}", err);
                }
            },
            _ => return false
        }
        true
    }

    fn update(&mut self) {

    }

    fn render(&mut self) -> Result<(), SurfaceError> {

        // Paints ui and writes to GPU mesh
        self.gewy.paint(&mut self.painter);
        self.painter.flush(&self.device, &self.queue);

        // Maps cursor icon
        if let Some(cursor_icon) = self.gewy.mapping().take_cursor_icon() {
            self.window.set_cursor_icon(cursor_icon.into());
        }

        // Gets surface texture
        let output = self.surface.get_current_texture()?;
        let tex_view = output.texture.create_view(&TextureViewDescriptor::default());

        // Encodes render pass
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let color_attachment = if let Some(msaa_texture_view) = &self.msaa_texture_view {
            RenderPassColorAttachment {
                view: &msaa_texture_view,
                resolve_target: Some(&tex_view),
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK.into()),
                    store: true
                },
            }
        }
        else {
            RenderPassColorAttachment {
                view: &tex_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK.into()),
                    store: true
                },
            }
        };

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.painter.gpu_view.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.painter.gpu_mesh.vertices.slice(..));
        render_pass.set_index_buffer(self.painter.gpu_mesh.indices.slice(..), IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.painter.gpu_mesh.index_count, 0, 0..1);
        drop(render_pass);

        // Submits encoded draw calls
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }

    fn create_msaa_texture_view(device: &Device, size: PhysicalSize<u32>, format: TextureFormat, samples_per_pixel: u32) -> TextureView {
        device
            .create_texture(&TextureDescriptor {
                label: Some("MSAA Texture"),
                size: Extent3d { width: size.width, height: size.height, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: samples_per_pixel,
                dimension: TextureDimension::D2,
                format,
                usage: TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[format]
            })
            .create_view(&TextureViewDescriptor::default())
    }

    fn features(debug: bool) -> Features {
        let mut features = Features::empty();
        if debug { features |= Features::POLYGON_MODE_LINE }
        features |= Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES;
        features
    }
}

impl From<crate::CursorIcon> for CursorIcon {
    fn from(value: crate::CursorIcon) -> Self {
        match value {
            crate::CursorIcon::Default => CursorIcon::Default,
            crate::CursorIcon::Crosshair => CursorIcon::Default,
            crate::CursorIcon::Hand => CursorIcon::Hand,
            crate::CursorIcon::Arrow => CursorIcon::Arrow,
            crate::CursorIcon::Move => CursorIcon::Move,
            crate::CursorIcon::Text => CursorIcon::Text,
            crate::CursorIcon::Wait => CursorIcon::Wait,
            crate::CursorIcon::Help => CursorIcon::Help,
            crate::CursorIcon::Progress => CursorIcon::Progress,
            crate::CursorIcon::NotAllowed => CursorIcon::NotAllowed,
            crate::CursorIcon::ContextMenu => CursorIcon::ContextMenu,
            crate::CursorIcon::Cell => CursorIcon::Cell,
            crate::CursorIcon::VerticalText => CursorIcon::VerticalText,
            crate::CursorIcon::Alias => CursorIcon::Alias,
            crate::CursorIcon::Copy => CursorIcon::Copy,
            crate::CursorIcon::NoDrop => CursorIcon::NoDrop,
            crate::CursorIcon::Grab => CursorIcon::Grab,
            crate::CursorIcon::Grabbing => CursorIcon::Grabbing,
            crate::CursorIcon::AllScroll => CursorIcon::AllScroll,
            crate::CursorIcon::ZoomIn => CursorIcon::ZoomIn,
            crate::CursorIcon::ZoomOut => CursorIcon::ZoomOut,
            crate::CursorIcon::EResize => CursorIcon::EResize,
            crate::CursorIcon::NResize => CursorIcon::NResize,
            crate::CursorIcon::NeResize => CursorIcon::NeResize,
            crate::CursorIcon::NwResize => CursorIcon::NwResize,
            crate::CursorIcon::SResize => CursorIcon::SResize,
            crate::CursorIcon::SeResize => CursorIcon::SeResize,
            crate::CursorIcon::SwResize => CursorIcon::SwResize,
            crate::CursorIcon::WResize => CursorIcon::WResize,
            crate::CursorIcon::EwResize => CursorIcon::EwResize,
            crate::CursorIcon::NsResize => CursorIcon::NsResize,
            crate::CursorIcon::NeswResize => CursorIcon::NeswResize,
            crate::CursorIcon::NwseResize => CursorIcon::NwseResize,
            crate::CursorIcon::ColResize => CursorIcon::ColResize,
            crate::CursorIcon::RowResize => CursorIcon::RowResize
        }
    }
}
