pub struct FrameBuffer {
    pub id: u32
}

impl FrameBuffer {
    pub unsafe fn new() -> FrameBuffer {
        let mut id:u32 = 0;
        gl::GenFramebuffers(1, &mut id);
        log::info!("generated framebuffer with id: {}", id);
        FrameBuffer {
            id
        }
    }

    pub unsafe fn bind(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
    }

    pub unsafe fn unbind(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }
}
