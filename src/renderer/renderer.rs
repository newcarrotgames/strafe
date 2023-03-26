use crate::models::cube::Cube;
use crate::renderer::buffer::Buffer;
use crate::renderer::program::ShaderProgram;
use crate::renderer::shader::{Shader, ShaderError};
use crate::renderer::vertex_array::VertexArray;
use crate::ui::ui::UserInterface;
use gl::types::GLint;
use glam::{Mat4, Vec3};
use image::ImageError;
use std::ptr;
use thiserror::Error;

use super::camera::Camera;
// use super::texture::{Texture, UITexture};

// const VERTEX_SHADER_SOURCE: &str = r#"
// #version 330
// in vec3 position;
// // in vec3 color;

// uniform mat4 transform;

// out vec3 pos;

// void main()
// {
//     gl_Position = transform * vec4(position, 1.0f);
//     pos = position;
// }
// "#;

// const FRAGMENT_SHADER_SOURCE: &str = r#"
// #version 330
// in vec3 pos;
// out vec4 FragColor;

// void main() {
//     FragColor = vec4(1 / abs(pos[0]), 1 / abs(pos[1]), 1 / abs(pos[2]), 1.0f);
// }
// "#;

//-----------------\\
// TEXTURE SHADERS \\
//-----------------\\

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 vertexUV;

out vec3 UV;

uniform mat4 transform;

void main() {
    gl_Position = transform * vec4(position, 1.0f);
    UV = vertexUV;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330

in vec3 UV;

out vec4 color;

uniform sampler2DArray t2da;
// uniform sampler2D uiTexture;

void main() {
    color = texture(t2da, UV);
}
"#;

//------------\\
// UI SHADERS \\
//------------\\

const UI_VERTEX_SHADER_SOURCE: &str = r#"
#version 330 core

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 vertexUV;

out vec2 UV;

void main() {
    gl_Position = vec4(position.x, position.y, 0.0, 1.0);
    UV = vertexUV;
}
"#;

const UI_FRAGMENT_SHADER_SOURCE: &str = r#"
#version 420

in vec2 UV;

out vec4 color;

// uniform sampler2DArray t2da;
layout (binding = 1) uniform sampler2D texture1;

void main() {
    // color = vec4(1.0, 0.5, 0.2, 0.5);
    color = texture(texture1, UV);
}
"#;

#[rustfmt::skip]
const CUBE_INDICES: [i32; 36] = [
     0,  1,  2,
     2,  3,  0,
     4,  5,  6,
     6,  7,  4,
     8,  9, 10,
    10, 11,  8,
    12, 13, 14,
    14, 15, 12,
    16, 17, 18,
    18, 19, 16,
    20, 21, 22,
    22, 23, 20,
];

// #[rustfmt::skip]
// const UI_VERTICES: [f32; 16] = [
// 	 1.0,  1.0,	1.0,  1.0,
// 	 1.0, -1.0, 1.0,  0.0,
//     -1.0, -1.0,	0.0,  0.0,
// 	-1.0,  1.0, 0.0,  1.0,
// ];

#[rustfmt::skip]
const UI_VERTICES: [f32; 16] = [
	 1.0,  1.0,	1.0,  0.0,
	 1.0, -1.0, 1.0,  1.0,
    -1.0, -1.0,	0.0,  1.0,
	-1.0,  1.0, 0.0,  0.0,
];

// #[rustfmt::skip]
// const UI_VERTICES: [f32; 16] = [
// 	 1.0,  1.0,	0.0,  1.0,
// 	 1.0, -1.0, 0.0,  0.0,
//     -1.0, -1.0,	1.0,  0.0,
// 	-1.0,  1.0, 1.0,  1.0,
// ];

#[rustfmt::skip]
const UI_INDICES: [i32; 6] = [
    0, 1, 3, 
    1, 2, 3
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
    vertex_array: VertexArray,
    ui_program: ShaderProgram,
    _ui_vertex_buffer: Buffer,
    _ui_index_buffer: Buffer,
    ui_vertex_array: VertexArray,
    angle: f32,
    total_length: i32,
    ui: UserInterface,
}

// todo: put this in the cube impl
fn get_indices(index: i32) -> [i32; 36] {
    let mut new_indices: [i32; 36] = [0; 36];
    let mut i = 0;
    while i < 36 {
        new_indices[i] = CUBE_INDICES[i] + (index * 24);
        i += 1;
    }
    log::debug!("new_indices: {}", format!("{:?}", new_indices));
    return new_indices;
}

impl Renderer {
    pub fn new(cubes: Vec<Cube>, ui: UserInterface) -> Result<Self, RendererInitError> {
        unsafe {
            // Level shader program and buffers
            let vertex_shader = Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;

            let vertex_array = VertexArray::new();
            vertex_array.bind();

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);

            let mut verts: Vec<f32> = Vec::new();
            let mut indices: Vec<i32> = Vec::new();

            let mut i = 0;
            for c in cubes.iter() {
                log::debug!("Got: {}", c.loc());
                verts.extend_from_slice(&c.geom());
                indices.extend_from_slice(&get_indices(i));
                i += 1;
            }
            log::debug!("verts: {}", format!("{:?}", verts));
            log::debug!("indices: {}", format!("{:?}", indices));

            vertex_buffer.set_data(&verts, gl::STATIC_DRAW);
            index_buffer.set_data(&indices, gl::STATIC_DRAW);

            let pos_attrib = program.get_attrib_location("position")?;
            vertex_array.set_attribute(pos_attrib, 3, 0, 6 * std::mem::size_of::<f32>() as GLint);

            let texture_attrib = program.get_attrib_location("vertexUV")?;
            vertex_array.set_attribute(
                texture_attrib,
                3,
                3,
                6 * std::mem::size_of::<f32>() as GLint,
            );

            // vertex_array.unbind();

            let total_length = (cubes.len() * 72) as i32;

            // UI shader program and buffers
            let ui_vertex_shader = Shader::new(UI_VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let ui_fragment_shader = Shader::new(UI_FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            let ui_program = ShaderProgram::new(&[ui_vertex_shader, ui_fragment_shader])?;

            let ui_vertex_array = VertexArray::new();
            ui_vertex_array.bind();

            let ui_vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            let ui_index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);

            ui_vertex_buffer.set_data(&UI_VERTICES, gl::STATIC_DRAW);
            ui_index_buffer.set_data(&UI_INDICES, gl::STATIC_DRAW);

            // ui_vertex_buffer.bind();
            // let ui_pos_attrib = ui_program.get_attrib_location("position")?;
            ui_vertex_array.set_attribute(0, 2, 0, 4 * std::mem::size_of::<f32>() as GLint);

            // let ui_texture_attrib = ui_program.get_attrib_location("vertexUV")?;
            ui_vertex_array.set_attribute(1, 2, 2, 4 * std::mem::size_of::<f32>() as GLint);

            // ui_vertex_array.unbind();

            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);

            log::info!("returning renderer");

            Ok(Self {
                program,
                _vertex_buffer: vertex_buffer,
                _index_buffer: index_buffer,
                vertex_array,
                ui_program,
                _ui_vertex_buffer: ui_vertex_buffer,
                _ui_index_buffer: ui_index_buffer,
                ui_vertex_array,
                angle: 0.0,
                total_length,
                ui
            })
        }
    }

    pub fn draw(&mut self, cam: &Camera) {
        let model = Mat4::from_rotation_x(self.angle);
        let view = Mat4::look_at_rh(cam.pos, cam.target, Vec3::new(0.0, 1.0, 0.0));
        let projection = Mat4::perspective_rh_gl(45.0f32.to_radians(), 1024.0 / 768.0, 0.1, 2000.0);
        let transform = projection * view * model;

        unsafe {
            // clear screen
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // self.ui_vertex_array.unbind();

            // log::info!("rendering level");

            // render level
            self.program.apply();
            self.vertex_array.bind();
            let _ = self.program.set_mat4_uniform("transform", transform);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::DrawElements(
                gl::TRIANGLES,
                self.total_length,
                gl::UNSIGNED_INT,
                ptr::null(),
            );

            // self.vertex_array.unbind();

            // log::info!("rendering ui");

            // render UI
            gl::Disable(gl::DEPTH_TEST);
            self.ui_program.apply();
            self.ui_vertex_array.bind();
            self.ui.ui_texture.activate(gl::TEXTURE1);
            gl::DrawElements(gl::TRIANGLES, 8, gl::UNSIGNED_INT, ptr::null());
        }
    }
}
