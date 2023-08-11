use glam::Vec2;
use wgpu::*;
use winit::window::{WindowBuilder, CursorIcon};
use winit::event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode, MouseButton};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::wgpu::WgpuBackend;
use crate::Gewy;

/// A configurable application runner that runs a single [`Gewy`] in a single [`winit`] window.
pub struct WinitApp {
    pub gewy: Gewy,
    pub width: u32,
    pub height: u32,
    pub backend_kind: BackendKind,
    pub debug: bool,
    pub samples_per_pixel: u32,
}

/// Stores the application in a window.
impl WinitApp {

    pub fn new(gewy: Gewy, width: u32, height: u32, backend_kind: BackendKind) -> Self {
        Self {
            width,
            height,
            backend_kind,
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

    pub fn with_backend_type(mut self, backend_type: BackendKind) -> Self {
        self.backend_kind = backend_type;
        self
    }

    pub async fn start(self) -> ! {
        
        // Opens window and handle high-level events
        let Self { width, height, gewy, debug, samples_per_pixel, backend_kind } = self;
        let size = PhysicalSize::new(width, height);
        let event_loop = EventLoop::new();
        let winit_window = WindowBuilder::new()
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap();
        let mut window = GewyWindow::new(winit_window, gewy, debug, samples_per_pixel, backend_kind).await;

        // Runs event loop
        event_loop.run(move |event, _, flow| {
            match event {
                Event::WindowEvent { event, .. } => Self::handle_window_event(event, &mut window, flow),
                Event::RedrawRequested( .. ) => Self::handle_redraw_event(&mut window, flow),
                Event::MainEventsCleared => { window.winit_window().request_redraw() }
                _ => {}
            }
        });
    }

    // Handle window-related events.
    fn handle_window_event(event: WindowEvent<'_>, window: &mut GewyWindow, flow: &mut ControlFlow) {
        if window.input(&event) { return }
        match event {
            WindowEvent::Resized(size) => window.resize(size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => window.resize(*new_inner_size),
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
    fn handle_redraw_event(window: &mut GewyWindow, flow: &mut ControlFlow) {
        match window.render() {
            Ok(_) => {}
            Err(e) => {
                log::error!("Window error: {e}");
                *flow = ControlFlow::Exit
            }
        }
    }
}

/// Backend-specific window data + the backend itself.
pub enum GewyWindow {
    WgpuWindow(WgpuWindow),
}

impl GewyWindow {

    async fn new(window: Window, gewy: Gewy, debug: bool, samples_per_pixel: u32, backend_kind: BackendKind) -> Self {
        match backend_kind {
            BackendKind::Wgpu => Self::WgpuWindow(
                WgpuWindow::new(
                    window,
                    gewy,
                    samples_per_pixel,
                    debug
                ).await
            )
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        match self {
            GewyWindow::WgpuWindow(wgpu_window) => wgpu_window.resize(size)
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        let (gewy, window) = match self {
            GewyWindow::WgpuWindow(window) => (&mut window.gewy, &mut window.window)
        };
        match event {
            WindowEvent::CursorEntered { .. } => {
                let result = gewy.mapping().enter_cursor();
                if let Err(err) = result {
                    eprintln!("WindowEvent::CursorEntered caused an error: {}", err);
                }
            },
            WindowEvent::CursorLeft { .. } => {
                let result = gewy.mapping().exit_cursor();
                if let Err(err) = result {
                    eprintln!("WindowEvent::CursorLeft caused an error: {}", err);
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                let position = Vec2::new(position.x as f32, position.y as f32);
                let result = gewy.mapping().move_cursor(position);
                if let Err(err) = result {
                    eprintln!("WindowEvent::CursorMoved caused an error: {}", err);
                }
            },
            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                let result = gewy.mapping().press(crate::MouseButton::Left);
                if let Err(err) = result {
                    eprintln!("WindowEvent::MouseInput caused an error: {}", err);
                }
            },
            WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. } => {
                let result = gewy.mapping().release(crate::MouseButton::Left);
                if let Err(err) = result {
                    eprintln!("WindowEvent::MouseInput caused an error: {}", err);
                }
            },
            _ => return false
        }
        if let Some(cursor_icon) = gewy.mapping().take_cursor_icon() {
            window.set_cursor_icon(cursor_icon.into());
        }
        false
    }

    fn render(&mut self) -> anyhow::Result<()> {
        match self {
            Self::WgpuWindow(window) => window.render()?,
        }
        Ok(())
    }

    fn winit_window(&self) -> &Window {
        match self {
            Self::WgpuWindow(wgpu_window) => &wgpu_window.window
        }
    }
}

pub struct WgpuWindow {
    window: Window,
    gewy: Gewy,
    device: Device,
    queue: Queue,
    surface: Surface,
    config: SurfaceConfiguration,
    backend: WgpuBackend
}

impl WgpuWindow {

    async fn new(
        window: Window,
        gewy: Gewy,
        samples_per_pixel: u32,
        debug: bool
    ) -> Self {

        // Creates WGPU instance.
        let window_size = window.inner_size();
        let instance = Instance::new(InstanceDescriptor::default());

        // Creates surface from window, and queries for compatible adapter.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance.request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        // Creates device and queue.
        let (device, queue) = adapter.request_device(
            &DeviceDescriptor {
                features: features(debug),
                label: None,
                ..Default::default()
            },
            None,
        ).await.unwrap();
        
        // Configures surface.
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

        // Creates gewy backend
        let backend = WgpuBackend::new(&device, surface_format, config.width, config.height, samples_per_pixel, debug).await;
        Self { window, gewy, device, queue, surface, config, backend }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
        self.gewy.resize(Vec2::new(size.width as f32, size.height as f32));
        self.backend.resize(size.width, size.height, &self.device);
    }

    fn render(&mut self) -> anyhow::Result<()> {
        let surface_texture = self.surface.get_current_texture()?;
        let surface_view = surface_texture.texture.create_view(&TextureViewDescriptor::default());
        let draw_commands = self.gewy.paint();
        self.backend.render(
            draw_commands,
            &self.device,
            &self.queue,
            &surface_view
        )?;
        surface_texture.present();
        Ok(())
    }
}

/// Which backend to use
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BackendKind { Wgpu }

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


fn features(debug: bool) -> Features {
    let mut features = Features::empty();
    if debug { features |= Features::POLYGON_MODE_LINE }
    features |= Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES;
    features
}