mod level;
mod models;
mod renderer;
mod ui;

use std::ffi::CStr;
use std::os::raw::c_void;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::level::level::Level;
use crate::models::cube::Cube;
use crate::renderer::camera::Camera;
use crate::renderer::texture::{TextureArray};
use crate::renderer::Renderer;
use crate::ui::ui::UserInterface;
use gl::types::{GLchar, GLenum, GLsizei, GLuint};
use glam::Vec3;
use glutin::dpi::{LogicalPosition, LogicalSize};
use glutin::event::{ElementState, Event, KeyboardInput, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::monitor::MonitorHandle;
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest};
use log::LevelFilter;
use rand::rngs::ThreadRng;
use rand::Rng;

use simple_logger::SimpleLogger;

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

fn get_rand_ceiling_tile(rng: &mut ThreadRng) -> u32 {
    let n = rng.gen_range(0..10);
    if n > 8 {
        return 24;
    } else if n > 4 && n > 7 {
        return 12;
    } else {
        return 0;
    }
}

fn get_rand_floor_tile(rng: &mut ThreadRng) -> u32 {
    let n = rng.gen_range(0..10);
    if n > 8 {
        return 42;
    } else if n > 4 && n <= 7 {
        return 49;
    } else {
        return 48;
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .env()
        .init()
        .unwrap();
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
        LAST_TIME = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
    }

    unsafe {
        // gl::Enable(gl::TEXTURE_2D);
        gl::ActiveTexture(gl::TEXTURE0);
    }

    let texture;
    unsafe {
        log::info!("loading tiles");
        texture = TextureArray::new();
        texture.load(Path::new("assets/tiles"));
    }

    let mut level = Level::new();
    level.build();

    let mut cubes: Vec<Cube> = Vec::new();
    for y in 0..64 {
        for x in 0..64 {
            cubes.push(Cube::new(
                Vec3::new(x as f32, 1.0, y as f32),
                get_rand_ceiling_tile(&mut rng),
            )); // ceiling
            if level.data[y][x] > 0 {
                cubes.push(Cube::new(
                    Vec3::new(x as f32, 0.0, y as f32),
                    level.data[y][x],
                ));
            }
            cubes.push(Cube::new(
                Vec3::new(x as f32, -1.0, y as f32),
                get_rand_floor_tile(&mut rng),
            )); // floor
        }
    }

    let mut camera = Camera::new(level.spawn);
    let mut ui = UserInterface::new([WINDOW_HEIGHT, WINDOW_WIDTH]);
    let mut renderer = Renderer::new(cubes, ui).expect("Cannot create renderer");

    log::info!("starting game loop");
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
                    // eye[0] += 1.0;
                    camera.turn(45.0);
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
                    camera.turn(-45.0);
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
                    camera.walk(1);
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
                    camera.walk(-1);
                }
                WindowEvent::Resized(physical_size) => gl_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::MainEventsCleared => {
                gl_context.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                renderer.draw(&camera);
                gl_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
