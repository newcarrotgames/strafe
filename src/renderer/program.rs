use crate::renderer::shader::{Shader, ShaderError};
use gl::types::*;
use glam::Mat4;
use std::ffi::CString;

pub struct ShaderProgram {
    pub id: GLuint,
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl ShaderProgram {
    pub unsafe fn new(shaders: &[Shader]) -> Result<Self, ShaderError> {
        let program = Self {
            id: gl::CreateProgram(),
        };

        for shader in shaders {
            gl::AttachShader(program.id, shader.id);
        }

        gl::LinkProgram(program.id);

        let mut success: GLint = 0;
        gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);

        if success == 1 {
            Ok(program)
        } else {
            let mut error_log_size: GLint = 0;
            gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetProgramInfoLog(
                program.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );
            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log)?;
            Err(ShaderError::LinkingError(log))
        }
    }

    pub unsafe fn apply(&self) {
        log::debug!("using program {}", self.id);
        gl::UseProgram(self.id);
    }

    pub unsafe fn get_attrib_location(&self, attrib: &str) -> Result<GLuint, ShaderError> {
        let attrib = CString::new(attrib)?;
        log::debug!("attrib: {}", format!("{:?}", attrib));
        Ok(gl::GetAttribLocation(self.id, attrib.as_ptr()) as GLuint)
    }

    // pub unsafe fn set_int_uniform(&self, name: &str, value: i32) -> Result<(), ShaderError> {
    //     self.apply();
    //     let uniform = CString::new(name)?;
    //     gl::Uniform1i(gl::GetUniformLocation(self.id, uniform.as_ptr()), value);
    //     Ok(())
    // }

    pub unsafe fn set_mat4_uniform(&self, name: &str, value: Mat4) -> Result<(), ShaderError> {
        // self.apply(); this function assumes you've called apply already
        let uniform = CString::new(name)?;
        let location_pos = gl::GetUniformLocation(self.id, uniform.as_ptr());
        gl::UniformMatrix4fv(location_pos, 1, gl::FALSE, &value.to_cols_array()[0]);
        Ok(())
    }
}
