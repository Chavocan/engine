//! Headed window: wgpu clear + in-process [`aetherforge_sim::Simulation`] (platform P3).
//!
//! Run locally (GPU + display):
//!   cargo run -p aetherforge_platform --features windowed --bin aetherforge_window
//!
//! - Each frame applies the next intent from a short **farm stub** loop (same story as `farm_demo_loop`).
//! - **Window title** shows tick / mission / farm summary (debug HUD).
//! - Optional env: `AETHERFORGE_WINDOW_MAX_SEC`, `AETHERFORGE_WINDOW_SEED` (default `42`).

use std::sync::Arc;
use std::time::{Duration, Instant};

use aetherforge_sim::{
    Intent, MissionOutcome, Observation, Simulation, SimulationConfig,
};
use pollster::block_on;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

/// Same sequence as `examples/farm_demo_loop.json` (one plant→grow→harvest cycle).
const FARM_DEMO_INTENTS: &[&str] = &[
    "farm_plant",
    "farm_advance_day",
    "farm_advance_day",
    "farm_advance_day",
    "farm_harvest",
];

fn format_title(obs: &Observation) -> String {
    let mut s = format!("AetherForge | tick={}", obs.tick);
    if let Some(m) = &obs.mission {
        let o = match m.outcome {
            MissionOutcome::Won => "won",
            MissionOutcome::Lost => "lost",
        };
        s.push_str(&format!(" mission={o}"));
    }
    s.push(' ');
    s.push_str(obs.message.as_str());
    if let Some(f) = &obs.farm {
        s.push_str(&format!(
            " | day={} plots={} inv={}",
            f.day,
            f.plots.len(),
            f.inventory.items.len()
        ));
    }
    const MAX: usize = 220;
    if s.len() > MAX {
        s.truncate(MAX);
        s.push('…');
    }
    s
}

fn clear_for_tick(tick: u64) -> wgpu::Color {
    let phase = (tick % 64) as f64 / 64.0;
    wgpu::Color {
        r: 0.06 + 0.05 * phase,
        g: 0.09 + 0.04 * (1.0 - phase),
        b: 0.14 + 0.06 * (0.5 - (phase - 0.5).abs()),
        a: 1.0,
    }
}

struct Gfx {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

struct App {
    window: Option<Arc<Window>>,
    gfx: Option<Gfx>,
    sim: Option<Simulation>,
    intent_idx: usize,
    start: Option<Instant>,
    max_run: Duration,
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
            sim: None,
            intent_idx: 0,
            start: None,
            max_run: Duration::from_secs(secs.max(1)),
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

        let seed = std::env::var("AETHERFORGE_WINDOW_SEED")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(42);
        self.sim = Some(Simulation::with_config(SimulationConfig::new(
            "headed-window",
            seed,
        )));
    }

    fn step_sim_and_title(&mut self) {
        let Some(sim) = self.sim.as_mut() else {
            return;
        };
        let Some(w) = self.window.as_ref() else {
            return;
        };
        let kind = FARM_DEMO_INTENTS[self.intent_idx % FARM_DEMO_INTENTS.len()];
        self.intent_idx = self.intent_idx.saturating_add(1);
        sim.apply_intent(Intent {
            kind: kind.to_string(),
        });
        sim.step();
        let obs = sim.snapshot_observation();
        let title = format_title(&obs);
        w.set_title(&title);
    }

    fn render(&mut self) {
        let tick = self
            .sim
            .as_ref()
            .map(|s| s.snapshot_observation().tick)
            .unwrap_or(0);
        let Some(gfx) = self.gfx.as_mut() else {
            return;
        };
        let Ok(frame) = gfx.surface.get_current_texture() else {
            return;
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let clear = clear_for_tick(tick);
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
                        load: wgpu::LoadOp::Clear(clear),
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
                self.step_sim_and_title();
                self.render();
                if let Some(w) = self.window.as_ref() {
                    w.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let _ = self.timeout_exit(event_loop);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::builder().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}
