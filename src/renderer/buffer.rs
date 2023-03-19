use gl::types::*;

pub struct Buffer {
    pub id: GLuint,
    target: GLuint,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

impl Buffer {
    pub unsafe fn new(target: GLuint) -> Self {
        let mut id: GLuint = 0;
        gl::GenBuffers(1, &mut id);
        log::info!("created buffer - id: {}, target: {}", id, target);
        Self { id, target }
    }

    pub unsafe fn set_data<D>(&self, data: &[D], usage: GLuint) {
        self.bind();
        let (_, data_bytes, _) = data.align_to::<u8>();
        gl::BufferData(
            self.target,
            data_bytes.len() as GLsizeiptr,
            data_bytes.as_ptr() as *const _,
            usage,
        );
        // self.unbind();
    }

    pub unsafe fn bind(&self) {
        gl::BindBuffer(self.target, self.id);
    }

    pub unsafe fn unbind(&self) {
        gl::BindBuffer(self.target, 0);
    }
}
