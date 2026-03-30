//! 8×8 bitmap text (font8x8) → RGBA texture + wgpu alpha-blended quad.

use bytemuck::{Pod, Zeroable};
use font8x8::UnicodeFonts;
use font8x8::unicode::{BasicFonts, LatinFonts};
use wgpu::util::DeviceExt;

const MAX_COLS: usize = 96;
const MAX_ROWS: usize = 26;
const CELL: usize = 8;
const TEX_W: u32 = (MAX_COLS * CELL) as u32;
const TEX_H: u32 = (MAX_ROWS * CELL) as u32;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    pos: [f32; 2],
    uv: [f32; 2],
}

fn glyph_bytes(ch: char) -> [u8; 8] {
    let latin = LatinFonts::new();
    let basic = BasicFonts::new();
    latin
        .get(ch)
        .or_else(|| basic.get(ch))
        .unwrap_or_else(|| basic.get(' ').expect("space glyph"))
}

fn rasterize_lines(lines: &[String]) -> Vec<u8> {
    let mut rgba = vec![0u8; TEX_W as usize * TEX_H as usize * 4];
    for (row, line) in lines.iter().enumerate().take(MAX_ROWS) {
        let y0 = row * CELL;
        for (col, ch) in line.chars().enumerate().take(MAX_COLS) {
            let x0 = col * CELL;
            let g = glyph_bytes(ch);
            for (gy, bits) in g.iter().copied().enumerate() {
                for gx in 0..8 {
                    if (bits >> gx) & 1 == 0 {
                        continue;
                    }
                    let px = x0 + gx;
                    let py = y0 + gy;
                    let i = (py * TEX_W as usize + px) * 4;
                    if i + 3 < rgba.len() {
                        rgba[i] = 210;
                        rgba[i + 1] = 230;
                        rgba[i + 2] = 255;
                        rgba[i + 3] = 255;
                    }
                }
            }
        }
    }
    rgba
}

pub struct HudOverlay {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    texture: wgpu::Texture,
    vertex_buffer: wgpu::Buffer,
    win_px: (u32, u32),
}

impl HudOverlay {
    pub fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        win_w: u32,
        win_h: u32,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("hud_shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "hud.wgsl"
            ))),
        });

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("hud_texture"),
            size: wgpu::Extent3d {
                width: TEX_W,
                height: TEX_H,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("hud_sampler"),
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("hud_bind_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("hud_bind"),
            layout: &bind_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("hud_pipeline_layout"),
            bind_group_layouts: &[&bind_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("hud_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("hud_vb"),
            contents: bytemuck::cast_slice(&quad_vertices(win_w.max(1), win_h.max(1))),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            pipeline,
            bind_group,
            texture,
            vertex_buffer: vb,
            win_px: (win_w.max(1), win_h.max(1)),
        }
    }

    pub fn resize_with_queue(&mut self, queue: &wgpu::Queue, win_w: u32, win_h: u32) {
        self.win_px = (win_w.max(1), win_h.max(1));
        let verts = quad_vertices(self.win_px.0, self.win_px.1);
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&verts));
    }

    pub fn set_text(&mut self, queue: &wgpu::Queue, lines: &[String]) {
        let rgba = rasterize_lines(lines);
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * TEX_W),
                rows_per_image: None,
            },
            wgpu::Extent3d {
                width: TEX_W,
                height: TEX_H,
                depth_or_array_layers: 1,
            },
        );
    }

    pub fn render_pass<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..6, 0..1);
    }
}

fn quad_vertices(win_w: u32, win_h: u32) -> [Vertex; 6] {
    let tw = TEX_W as f32;
    let th = TEX_H as f32;
    let wf = win_w as f32;
    let hf = win_h as f32;
    let left = -1.0;
    let top = 1.0;
    let right = -1.0 + 2.0 * tw / wf;
    let bottom = 1.0 - 2.0 * th / hf;
    [
        Vertex {
            pos: [left, top],
            uv: [0.0, 0.0],
        },
        Vertex {
            pos: [right, top],
            uv: [1.0, 0.0],
        },
        Vertex {
            pos: [left, bottom],
            uv: [0.0, 1.0],
        },
        Vertex {
            pos: [right, top],
            uv: [1.0, 0.0],
        },
        Vertex {
            pos: [right, bottom],
            uv: [1.0, 1.0],
        },
        Vertex {
            pos: [left, bottom],
            uv: [0.0, 1.0],
        },
    ]
}
