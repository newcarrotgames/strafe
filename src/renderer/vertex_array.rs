use gl::types::*;

pub struct VertexArray {
    pub id: GLuint,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}

impl VertexArray {
    pub unsafe fn new() -> Self {
        let mut id: GLuint = 0;
        gl::GenVertexArrays(1, &mut id);
        log::info!("created vertex array with id {}", id);
        Self { id }
    }

    pub unsafe fn set_attribute(
        &self,
        attrib_pos: GLuint,
        size: GLint,
        offset: GLint,
        stride: GLint,
    ) {
        log::info!("set attribute: {} {} {} {}", attrib_pos, size, offset, stride);
        self.bind();
        gl::VertexAttribPointer(
            attrib_pos,
            size,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (offset * std::mem::size_of::<f32>() as i32) as *const gl::types::GLvoid,
        );
        gl::EnableVertexAttribArray(attrib_pos);
    }

    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.id);
    }

    pub unsafe fn unbind(&self) {
        gl::BindVertexArray(0);
    }
}