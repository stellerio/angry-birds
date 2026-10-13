use std::{sync::Arc, time::Instant};

use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use wgpu::util::DeviceExt;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes},
};

const SLING: Vec2 = Vec2::new(-0.66, -0.43);
const BIRD_RADIUS: f32 = 0.045;
const GRAVITY: f32 = 1.65;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x3],
        }
    }
}

#[derive(Clone, Copy)]
struct Block {
    center: Vec2,
    size: Vec2,
    alive: bool,
}

#[derive(Clone, Copy)]
struct Pig {
    center: Vec2,
    alive: bool,
}

struct Game {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    size: PhysicalSize<u32>,
    mouse: Vec2,
    dragging: bool,
    bird: Vec2,
    velocity: Vec2,
    flying: bool,
    blocks: Vec<Block>,
    pigs: Vec<Pig>,
    score: u32,
    last_frame: Instant,
}

impl Game {
    async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();
        let surface = instance
            .create_surface(window.clone())
            .expect("could not create a graphics surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("no compatible graphics adapter found");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("angry-birds-rust-device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            })
            .await
            .expect("could not create graphics device");

        let capabilities = surface.get_capabilities(&adapter);
        let format = capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("simple-2d-shader"),
            source: wgpu::ShaderSource::Wgsl(
                r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
};
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};
@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(input.position, 0.0, 1.0);
    output.color = input.color;
    return output;
}
@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}
"#
                .into(),
            ),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("2d-pipeline-layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("2d-render-pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Vertex::layout()],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        });

        let mut game = Self {
            window,
            surface,
            device,
            queue,
            config,
            pipeline,
            size,
            mouse: SLING,
            dragging: false,
            bird: SLING,
            velocity: Vec2::ZERO,
            flying: false,
            blocks: Vec::new(),
            pigs: Vec::new(),
            score: 0,
            last_frame: Instant::now(),
        };
        game.reset();
        game
    }

    fn reset(&mut self) {
        self.bird = SLING;
        self.velocity = Vec2::ZERO;
        self.flying = false;
        self.dragging = false;
        self.score = 0;
        self.blocks = vec![
            Block { center: Vec2::new(0.12, -0.37), size: Vec2::new(0.07, 0.25), alive: true },
            Block { center: Vec2::new(0.43, -0.37), size: Vec2::new(0.07, 0.25), alive: true },
            Block { center: Vec2::new(0.275, -0.23), size: Vec2::new(0.38, 0.065), alive: true },
            Block { center: Vec2::new(0.275, -0.08), size: Vec2::new(0.07, 0.24), alive: true },
            Block { center: Vec2::new(0.57, -0.37), size: Vec2::new(0.07, 0.25), alive: true },
            Block { center: Vec2::new(0.72, -0.23), size: Vec2::new(0.32, 0.065), alive: true },
        ];
        self.pigs = vec![
            Pig { center: Vec2::new(0.275, -0.13), alive: true },
            Pig { center: Vec2::new(0.72, -0.13), alive: true },
        ];
        self.window.set_title("Angry Birds Rust | Drag the bird, release to launch | R = reset");
    }

    fn cursor_to_world(&self, x: f64, y: f64) -> Vec2 {
        Vec2::new(
            (x as f32 / self.size.width.max(1) as f32) * 2.0 - 1.0,
            1.0 - (y as f32 / self.size.height.max(1) as f32) * 2.0,
        )
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }
        self.size = size;
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }

    fn update(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame).as_secs_f32().min(0.033);
        self.last_frame = now;

        if self.dragging {
            let offset = self.mouse - SLING;
            self.bird = SLING + offset.clamp_length_max(0.23);
        } else if self.flying {
            self.velocity.y -= GRAVITY * dt;
            self.bird += self.velocity * dt;

            for block in &mut self.blocks {
                if block.alive
                    && (self.bird.x - block.center.x).abs() < block.size.x * 0.5 + BIRD_RADIUS
                    && (self.bird.y - block.center.y).abs() < block.size.y * 0.5 + BIRD_RADIUS
                {
                    block.alive = false;
                    self.score += 100;
                    self.velocity *= 0.72;
                }
            }

            for pig in &mut self.pigs {
                if pig.alive && self.bird.distance(pig.center) < 0.085 {
                    pig.alive = false;
                    self.score += 500;
                    self.velocity *= 0.62;
                }
            }

            if self.bird.x > 1.2 || self.bird.y < -1.1 || self.bird.x < -1.2 {
                self.flying = false;
            }
            self.window.set_title(&format!(
                "Angry Birds Rust | Score: {} | R = reset",
                self.score
            ));
        }
    }

    fn launch(&mut self) {
        if !self.dragging {
            return;
        }
        let pull = SLING - self.bird;
        self.dragging = false;
        if pull.length() > 0.025 {
            self.velocity = pull * 3.25;
            self.flying = true;
        } else {
            self.bird = SLING;
        }
    }

    fn vertices(&self) -> Vec<Vertex> {
        let mut out = Vec::with_capacity(1800);
        let sky = [0.48, 0.78, 0.96];
        let ground = [0.30, 0.68, 0.30];
        let wood = [0.58, 0.34, 0.16];
        let pig_green = [0.25, 0.80, 0.32];

        rect(&mut out, Vec2::new(0.0, -0.78), Vec2::new(2.0, 0.44), ground);
        rect(&mut out, Vec2::new(0.0, -0.55), Vec2::new(2.0, 0.025), [0.18, 0.48, 0.20]);

        // Slingshot posts and elastic.
        quad_line(&mut out, Vec2::new(-0.70, -0.61), Vec2::new(-0.66, -0.39), 0.018, [0.30, 0.17, 0.08]);
        quad_line(&mut out, Vec2::new(-0.66, -0.39), Vec2::new(-0.59, -0.59), 0.018, [0.30, 0.17, 0.08]);
        if self.dragging {
            quad_line(&mut out, Vec2::new(-0.66, -0.40), self.bird, 0.008, [0.22, 0.12, 0.07]);
            quad_line(&mut out, self.bird, Vec2::new(-0.59, -0.42), 0.008, [0.22, 0.12, 0.07]);
        }

        for block in &self.blocks {
            if block.alive {
                rect(&mut out, block.center, block.size, wood);
                rect(
                    &mut out,
                    block.center + Vec2::new(0.0, block.size.y * 0.30),
                    Vec2::new(block.size.x * 0.82, 0.012),
                    [0.78, 0.56, 0.31],
                );
            }
        }

        for pig in &self.pigs {
            if pig.alive {
                circle(&mut out, pig.center, 0.055, pig_green, 24);
                circle(&mut out, pig.center + Vec2::new(-0.018, 0.008), 0.006, [0.04, 0.12, 0.04], 8);
                circle(&mut out, pig.center + Vec2::new(0.018, 0.008), 0.006, [0.04, 0.12, 0.04], 8);
                rect(&mut out, pig.center + Vec2::new(0.0, -0.014), Vec2::new(0.035, 0.014), [0.42, 0.90, 0.48]);
            }
        }

        // Bird body and a tiny beak.
        circle(&mut out, self.bird, BIRD_RADIUS, [0.88, 0.18, 0.15], 28);
        circle(&mut out, self.bird + Vec2::new(0.014, 0.012), 0.007, [1.0, 1.0, 1.0], 10);
        circle(&mut out, self.bird + Vec2::new(0.016, 0.012), 0.0035, [0.05, 0.05, 0.05], 8);
        rect(&mut out, self.bird + Vec2::new(0.045, -0.002), Vec2::new(0.025, 0.014), [1.0, 0.72, 0.08]);

        // Aim hint while the bird is being pulled back.
        if self.dragging {
            let pull = SLING - self.bird;
            for i in 1..=5 {
                let p = self.bird + pull * (i as f32 * 0.22);
                circle(&mut out, p, 0.008, [0.95, 0.95, 0.82], 8);
            }
        }

        let _ = sky; // sky is supplied as the render-pass clear color.
        out
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let vertices = self.vertices();
        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("scene-vertices"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("frame-encoder"),
        });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("main-render-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.48,
                            g: 0.78,
                            b: 0.96,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            pass.draw(0..vertices.len() as u32, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }
}

fn vertex(position: Vec2, color: [f32; 3]) -> Vertex {
    Vertex { position: position.to_array(), color }
}

fn rect(out: &mut Vec<Vertex>, center: Vec2, size: Vec2, color: [f32; 3]) {
    let half = size * 0.5;
    let a = center + Vec2::new(-half.x, -half.y);
    let b = center + Vec2::new(half.x, -half.y);
    let c = center + Vec2::new(half.x, half.y);
    let d = center + Vec2::new(-half.x, half.y);
    out.extend_from_slice(&[
        vertex(a, color), vertex(b, color), vertex(c, color),
        vertex(a, color), vertex(c, color), vertex(d, color),
    ]);
}

fn quad_line(out: &mut Vec<Vertex>, start: Vec2, end: Vec2, thickness: f32, color: [f32; 3]) {
    let delta = end - start;
    if delta.length_squared() < 0.000001 {
        return;
    }
    let side = Vec2::new(-delta.y, delta.x).normalize() * thickness * 0.5;
    let a = start + side;
    let b = start - side;
    let c = end - side;
    let d = end + side;
    out.extend_from_slice(&[
        vertex(a, color), vertex(b, color), vertex(c, color),
        vertex(a, color), vertex(c, color), vertex(d, color),
    ]);
}

fn circle(out: &mut Vec<Vertex>, center: Vec2, radius: f32, color: [f32; 3], segments: usize) {
    for i in 0..segments {
        let a = i as f32 / segments as f32 * std::f32::consts::TAU;
        let b = (i + 1) as f32 / segments as f32 * std::f32::consts::TAU;
        out.extend_from_slice(&[
            vertex(center, color),
            vertex(center + Vec2::new(a.cos(), a.sin()) * radius, color),
            vertex(center + Vec2::new(b.cos(), b.sin()) * radius, color),
        ]);
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("could not create event loop");
    let attrs: WindowAttributes = Window::default_attributes()
        .with_title("Angry Birds Rust")
        .with_inner_size(PhysicalSize::new(1280, 720));
    let window = Arc::new(event_loop.create_window(attrs).expect("could not create window"));
    let mut game = pollster::block_on(Game::new(window.clone()));

    event_loop
        .run(move |event, target| {
            target.set_control_flow(winit::event_loop::ControlFlow::Poll);
            match event {
                Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested => target.exit(),
                    WindowEvent::Resized(size) => game.resize(size),
                    WindowEvent::CursorMoved { position, .. } => {
                        game.mouse = game.cursor_to_world(position.x, position.y);
                        if game.dragging {
                            let offset = game.mouse - SLING;
                            game.bird = SLING + offset.clamp_length_max(0.23);
                        }
                    }
                    WindowEvent::MouseInput { state, button: MouseButton::Left, .. } => {
                        match state {
                            ElementState::Pressed => {
                                if !game.flying && game.bird.distance(game.mouse) < 0.13 {
                                    game.dragging = true;
                                    game.bird = game.mouse;
                                }
                            }
                            ElementState::Released => game.launch(),
                        }
                    }
                    WindowEvent::KeyboardInput { event, .. }
                        if event.state == ElementState::Pressed =>
                    {
                        match event.physical_key {
                            PhysicalKey::Code(KeyCode::Escape) => target.exit(),
                            PhysicalKey::Code(KeyCode::KeyR) => game.reset(),
                            _ => {}
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        game.update();
                        match game.render() {
                            Ok(()) => {}
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                game.surface.configure(&game.device, &game.config);
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => target.exit(),
                            Err(wgpu::SurfaceError::Timeout) => {}
                            Err(_) => {}
                        }
                    }
                    _ => {}
                },
                Event::AboutToWait => window.request_redraw(),
                _ => {}
            }
        })
        .expect("event loop failed");
}
