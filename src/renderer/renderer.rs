use crate::models::cube::Cube;
use crate::renderer::buffer::Buffer;
use crate::renderer::program::ShaderProgram;
use crate::renderer::shader::{Shader, ShaderError};
use crate::renderer::vertex_array::VertexArray;
use glam::{Mat4, Vec3};
use image::ImageError;
use std::ptr;
use thiserror::Error;

use super::camera::Camera;

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330
in vec3 position;
// in vec3 color;

uniform mat4 transform;

out vec3 pos;

void main()
{
    gl_Position = transform * vec4(position, 1.0f);
    pos = position;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330
in vec3 pos;
out vec4 FragColor;

void main() {
    FragColor = vec4(1 / abs(pos[0]), 1 / abs(pos[1]), 1 / abs(pos[2]), 1.0f);
}
"#;

// #[rustfmt::skip]
// const CUBE_VERTICES: [f32; 24] = [
//     -1.0, -1.0,  1.0,
//     1.0, -1.0,  1.0,
//     1.0,  1.0,  1.0,
//     -1.0,  1.0,  1.0,
//     -1.0, -1.0, -1.0,
//     1.0, -1.0, -1.0,
//     1.0,  1.0, -1.0,
//     -1.0,  1.0, -1.0
// ];

// #[rustfmt::skip]
// const CUBE_COLORS: [f32; 24] = [
//     0.0, 0.0, 0.0,
//     1.0, 0.0, 0.0,
//     1.0, 1.0, 0.0,
//     0.0, 1.0, 0.0,
//     0.0, 0.0, 1.0,
//     1.0, 0.0, 1.0,
//     1.0, 1.0, 1.0,
//     0.0, 1.0, 1.0
// ];

#[rustfmt::skip]
const CUBE_COLORS: [f32; 24] = [
    0.0, 0.0, 0.0,
    0.5, 0.0, 0.0,
    1.0, 1.0, 0.0,
    0.0, 0.5, 0.0,
    0.0, 0.0, 1.0,
    0.5, 0.0, 1.0,
    0.5, 1.0, 1.0,
    0.0, 1.0, 1.0
];

#[rustfmt::skip]
const CUBE_INDICES: [i32; 36] = [
    0, 1, 2,
    2, 3, 0,
    1, 5, 6,
    6, 2, 1,
    7, 6, 5,
    5, 4, 7,
    4, 0, 3,
    3, 7, 4,
    4, 5, 1,
    1, 0, 4,
    3, 2, 6,
    6, 7, 3
];

#[derive(Debug, Error)]
pub enum RendererInitError {
    #[error{"{0}"}]
    ImageError(#[from] ImageError),
    #[error{"{0}"}]
    ShaderError(#[from] ShaderError),
}

pub struct Renderer {
    program: ShaderProgram,
    _vertex_buffer: Buffer,
    _index_buffer: Buffer,
    // _color_buffer: Buffer,
    vertex_array: VertexArray,
    angle: f32,
    total_length: i32,
}

fn get_indices(index: i32) -> [i32; 36] {
    let mut new_indices: [i32; 36] = [0; 36];
    let mut i = 0;
    while i < 36 {
        new_indices[i] = CUBE_INDICES[i] + (index * 8);
        i += 1;
    }
    log::info!("new_indices: {}", format!("{:?}", new_indices));
    return new_indices;
}

impl Renderer {
    pub fn new(cubes: Vec<Cube>) -> Result<Self, RendererInitError> {
        unsafe {
            let vertex_shader = Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;

            let vertex_array = VertexArray::new();
            vertex_array.bind();

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            // let color_buffer = Buffer::new(gl::ARRAY_BUFFER);

            let mut verts: Vec<f32> = Vec::new();
            let mut indices: Vec<i32> = Vec::new();
            // let mut colors: Vec<f32> = Vec::new();

            let mut i = 0;
            for c in cubes.iter() {
                log::info!("Got: {}", c.loc());
                verts.extend_from_slice(&c.geom());
                indices.extend_from_slice(&get_indices(i));
                // colors.extend_from_slice(&CUBE_COLORS);
                i += 1;
            }
            log::info!("verts: {}", format!("{:?}", verts));
            log::info!("indices: {}", format!("{:?}", indices));
            // log::info!("colors: {}", format!("{:?}", colors));

            vertex_buffer.set_data(&verts, gl::STATIC_DRAW);
            // color_buffer.set_data(&colors, gl::STATIC_DRAW);
            index_buffer.set_data(&indices, gl::STATIC_DRAW);

            vertex_buffer.bind();
            let pos_attrib = program.get_attrib_location("position")?;
            vertex_array.set_attribute(pos_attrib, 3, 0);
            vertex_buffer.unbind();

            // color_buffer.bind();
            // let color_attrib = program.get_attrib_location("color")?;
            // vertex_array.set_attribute(color_attrib, 3, 0);
            // color_buffer.unbind();
            vertex_array.unbind();

            let angle = 0.0; // std::f32::consts::PI / 1.0;

            // Enable depth test
            gl::Enable(gl::DEPTH_TEST);

            // Accept fragment if it closer to the camera than the former one
            gl::DepthFunc(gl::LESS);

            let total_length = (cubes.len() * 36) as i32;

            log::info!("renderer done, total_length: {}", total_length);

            Ok(Self {
                program,
                _vertex_buffer: vertex_buffer,
                _index_buffer: index_buffer,
                // _color_buffer: color_buffer,
                vertex_array,
                angle,
                total_length
            })
        }
    }

    pub fn draw(&mut self, cam: &Camera) {
        let model = Mat4::from_rotation_x(self.angle);
        let view = Mat4::look_at_rh(cam.pos, cam.target, Vec3::new(0.0, 1.0, 0.0));
        let projection = Mat4::perspective_rh_gl(45.0f32.to_radians(), 1024.0 / 768.0, 0.1, 1000.0);
        let transform = projection * view * model;

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            self.vertex_array.bind();
            let _ = self.program.set_mat4_uniform("transform", transform);
            self.program.apply();
            gl::DrawElements(
                gl::TRIANGLES,
                self.total_length,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
    }
}
