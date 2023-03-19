mod renderer;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::cube::Cube;
use crate::renderer::Renderer;
use glutin::dpi::{LogicalPosition, LogicalSize};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::monitor::MonitorHandle;
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest};
use simple_logger::SimpleLogger;
use glam::Vec3;

mod models {
    pub mod position;
    pub mod cube;
}

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;

static mut DELTA_TIME: f64 = 0.0;
static mut LAST_TIME: f64 = 0.0;

fn get_center(monitor: MonitorHandle) -> LogicalPosition<u32> {
    let monitor_size = monitor.size();
    log::info!(
        "monitor size: {}, {}",
        monitor_size.width,
        monitor_size.height
    );
    let x = (monitor_size.width - WINDOW_WIDTH) / 3;
    let y = (monitor_size.height - WINDOW_HEIGHT) / 4;
    log::info!("window position: {}, {}", x, y);
    return LogicalPosition::new(x, y);
}

fn main() {
    SimpleLogger::new().env().init().unwrap();
    log::info!("starting gpthack");
    let event_loop = EventLoop::new();
    let primary_monitor = match event_loop.primary_monitor() {
        Some(m) => m,
        None => todo!(),
    };
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .with_position(get_center(primary_monitor))
        .with_title("GPT-Hack");

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    unsafe {
        LAST_TIME = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        // gl::FrontFace(gl::CW);
        // gl::CullFace(gl::BACK);
        // gl::Enable(gl::CULL_FACE);
        // gl::Enable(gl::DEPTH_TEST);
    }

    let mut cubes:Vec<Cube> = Vec::new();
    cubes.push(Cube::new(Vec3::new(2.0, 2.0, 2.0)));
    cubes.push(Cube::new(Vec3::new(2.0, 2.0, 0.0)));
    cubes.push(Cube::new(Vec3::new(4.0, 4.0, 0.0)));

    let mut renderer = Renderer::new(cubes).expect("Cannot create renderer");
    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        unsafe {
            let current = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            DELTA_TIME = current - LAST_TIME;
            LAST_TIME = current;
        }

        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => gl_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::MainEventsCleared => {
                gl_context.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                renderer.draw();
                gl_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
