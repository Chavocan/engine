//! Headed window with a single wgpu clear color — minimal playable viewport (platform P1/P2).
//!
//! Run locally (requires a GPU and display):
//!   cargo run -p aetherforge_platform --features windowed --bin aetherforge_window
//!
//! Optional: `AETHERFORGE_WINDOW_MAX_SEC` (default `300`) exits after N seconds so automation can close.

use std::sync::Arc;
use std::time::{Duration, Instant};

use pollster::block_on;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

struct Gfx {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

struct App {
    window: Option<Arc<Window>>,
    gfx: Option<Gfx>,
    start: Option<Instant>,
    max_run: Duration,
    painted_once: bool,
}

impl App {
    fn new() -> Self {
        let secs = std::env::var("AETHERFORGE_WINDOW_MAX_SEC")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(300);
        Self {
            window: None,
            gfx: None,
            start: None,
            max_run: Duration::from_secs(secs.max(1)),
            painted_once: false,
        }
    }

    fn init_gfx(&mut self, window: Arc<Window>) {
        let instance = wgpu::Instance::default();
        let surface = instance
            .create_surface(window.clone())
            .expect("wgpu create_surface");

        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("request_adapter");

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let (device, queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("aetherforge_window"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
            },
            None,
        ))
        .expect("request_device");

        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        self.gfx = Some(Gfx {
            surface,
            device,
            queue,
            config,
        });
        self.window = Some(window);
        self.start = Some(Instant::now());
    }

    fn render(&mut self) {
        let Some(gfx) = self.gfx.as_mut() else {
            return;
        };
        let Ok(frame) = gfx.surface.get_current_texture() else {
            return;
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = gfx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("aetherforge_clear"),
            });
        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("aetherforge_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.06,
                            g: 0.09,
                            b: 0.16,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }
        gfx.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
        self.painted_once = true;
    }

    fn resize(&mut self, w: u32, h: u32) {
        let Some(gfx) = self.gfx.as_mut() else {
            return;
        };
        let w = w.max(1);
        let h = h.max(1);
        gfx.config.width = w;
        gfx.config.height = h;
        gfx.surface.configure(&gfx.device, &gfx.config);
    }

    fn timeout_exit(&self, event_loop: &ActiveEventLoop) -> bool {
        if self
            .start
            .map(|t| t.elapsed() > self.max_run)
            .unwrap_or(false)
        {
            event_loop.exit();
            return true;
        }
        false
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes()
            .with_title("AetherForge")
            .with_inner_size(winit::dpi::LogicalSize::new(960.0, 540.0));
        let window = Arc::new(event_loop.create_window(attrs).expect("create_window"));
        self.init_gfx(window);
        if let Some(w) = self.window.as_ref() {
            w.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        if self.timeout_exit(event_loop) {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.resize(size.width, size.height);
                if let Some(w) = self.window.as_ref() {
                    w.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                self.render();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.timeout_exit(event_loop) {
            return;
        }
        if !self.painted_once {
            if let Some(w) = self.window.as_ref() {
                w.request_redraw();
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::builder().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}
