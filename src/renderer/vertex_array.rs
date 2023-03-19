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
        Self { id }
    }

    pub unsafe fn set_attribute(
        &self,
        attrib_pos: GLuint,
        size: GLint,
        offset: GLint,
    ) {
        self.bind();
        gl::VertexAttribPointer(
            attrib_pos,
            size,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<f32>() as GLint,
            offset as *const _,
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