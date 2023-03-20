mod renderer;

use std::ffi::CStr;
use std::os::raw::c_void;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::cube::Cube;
use crate::renderer::Renderer;
use gl::types::{GLchar, GLenum, GLsizei, GLuint};
use glam::Vec3;
use glutin::dpi::{LogicalPosition, LogicalSize};
use glutin::event::{ElementState, Event, KeyboardInput, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::monitor::MonitorHandle;
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest};
use simple_logger::SimpleLogger;

mod models {
    pub mod cube;
    pub mod position;
}

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;

static mut DELTA_TIME: f64 = 0.0;
static mut LAST_TIME: f64 = 0.0;

const FIRST_ROOM: [[u16; 8]; 8] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 2, 0, 0, 4, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 3, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];

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

extern "system" fn debug_callback(
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    _length: GLsizei,
    message: *const GLchar,
    _user_param: *mut c_void,
) {
    let source_str = match source {
        gl::DEBUG_SOURCE_API => "API",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "WINDOW_SYSTEM",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "SHADER_COMPILER",
        gl::DEBUG_SOURCE_THIRD_PARTY => "THIRD_PARTY",
        gl::DEBUG_SOURCE_APPLICATION => "APPLICATION",
        gl::DEBUG_SOURCE_OTHER => "OTHER",
        _ => "UNKNOWN",
    };

    let type_str = match type_ {
        gl::DEBUG_TYPE_ERROR => "ERROR",
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "DEPRECATED_BEHAVIOR",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "UNDEFINED_BEHAVIOR",
        gl::DEBUG_TYPE_PORTABILITY => "PORTABILITY",
        gl::DEBUG_TYPE_PERFORMANCE => "PERFORMANCE",
        gl::DEBUG_TYPE_OTHER => "OTHER",
        _ => "UNKNOWN",
    };

    let severity_str = match severity {
        gl::DEBUG_SEVERITY_HIGH => "HIGH",
        gl::DEBUG_SEVERITY_MEDIUM => "MEDIUM",
        gl::DEBUG_SEVERITY_LOW => "LOW",
        gl::DEBUG_SEVERITY_NOTIFICATION => "NOTIFICATION",
        _ => "UNKNOWN",
    };

    let message_str = unsafe { CStr::from_ptr(message).to_string_lossy() };

    log::error!(
        "GL Debug ({}:{} [{}]): {}",
        source_str,
        type_str,
        severity_str,
        message_str
    );
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
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(Some(debug_callback), std::ptr::null());
    }

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

    let mut cubes: Vec<Cube> = Vec::new();
    for (y, row) in FIRST_ROOM.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if FIRST_ROOM[y][x] > 0 {
                let _x: i16 = 4 - (x as i16);
                let _y: i16 = 4 - (y as i16);
                cubes.push(Cube::new(Vec3::new(_x as f32, _y as f32, 0.0)));
            }
        }
    }

    // cubes.push(Cube::new(Vec3::new(0.0, 0.0, 0.0)));
    // cubes.push(Cube::new(Vec3::new(0.0, 1.0, 0.0)));
    // cubes.push(Cube::new(Vec3::new(0.0, 2.0, 0.0)));

    let mut eye:Vec3 = Vec3::new(0.0, 0.0, 20.0);

    let mut renderer = Renderer::new(cubes, &mut eye).expect("Cannot create renderer");
    event_loop.run(move |event, _, control_flow| {
        // let next_frame_time =
        //     std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(0);
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
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(glutin::event::VirtualKeyCode::Left),
                            ..
                        },
                    ..
                } => {
                    log::info!("left?");
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(glutin::event::VirtualKeyCode::Right),
                            ..
                        },
                    ..
                } => {
                    log::info!("right?");
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(glutin::event::VirtualKeyCode::Up),
                            ..
                        },
                    ..
                } => {
                    log::info!("up?");
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(glutin::event::VirtualKeyCode::Down),
                            ..
                        },
                    ..
                } => {
                    log::info!("down?");
                }
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
