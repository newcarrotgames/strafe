use gl::types::*;
use image::{EncodableLayout, ImageError};
use std::{env, error::Error, fs::read_dir, path::Path, ptr::null};
use walkdir::WalkDir;

pub struct Texture {
    pub id: GLuint,
}

impl Texture {
    pub unsafe fn new() -> Self {
        let mut id: GLuint = 0;
        gl::GenTextures(1, &mut id);
        Self { id }
    }

    pub unsafe fn load(&self, path: &Path) -> Result<(), ImageError> {
        self.bind();

        let img = image::open(path)?.into_rgba8();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img.as_bytes().as_ptr() as *const _,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        Ok(())
    }

    pub unsafe fn set_wrapping(&self, mode: GLuint) {
        self.bind();
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, mode as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, mode as GLint);
    }

    pub unsafe fn set_filtering(&self, mode: GLuint) {
        self.bind();
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, mode as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mode as GLint);
    }

    pub unsafe fn bind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.id)
    }

    pub unsafe fn activate(&self, unit: GLuint) {
        gl::ActiveTexture(unit);
        self.bind();
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}

pub struct TextureArray {
    pub id: GLuint,
}

impl TextureArray {
    pub unsafe fn new() -> Self {
        let mut id: GLuint = 0;
        gl::GenTextures(1, &mut id);
        Self { id }
    }

    pub unsafe fn load(&self, path: &Path) {
        self.bind();

        let num_paths = read_dir(path).unwrap().count();

        gl::TexStorage3D(gl::TEXTURE_2D_ARRAY, 1, gl::RGBA8, 16, 16, 91);

        let mut all_img_data: Vec<u8> = Vec::new();

        let dir = match env::current_dir() {
            Ok(it) => it,
            Err(e) => {
                log::error!("{}", e);
                return;
            }
        };

        log::info!("current_dir: {}", dir.display());

        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if !entry.path().is_file() {
                continue;
            }
            log::info!("loading tile: {}", entry.path().display());
            let img_result = image::open(entry.path());
            let img = match img_result {
                Ok(it) => it,
                Err(e) => {
                    log::error!("could not load tile image: {}", e);
                    return;
                }
            };
            let mut img_data = img.into_rgba8().into_vec();
            all_img_data.append(&mut img_data);
        }

        gl::TexSubImage3D(
            gl::TEXTURE_2D_ARRAY,
            0,
            0,
            0,
            0,
            16,
            16,
            num_paths as i32,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            all_img_data.as_bytes().as_ptr() as *const _,
        );

        self.set_filtering(gl::NEAREST);
        self.set_wrapping(gl::REPEAT);
    }

    pub unsafe fn set_wrapping(&self, mode: GLuint) {
        self.bind();
        gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_S, mode as GLint);
        gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_T, mode as GLint);
    }

    pub unsafe fn set_filtering(&self, mode: GLuint) {
        self.bind();
        gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_MIN_FILTER, mode as GLint);
        gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_MAG_FILTER, mode as GLint);
    }

    pub unsafe fn bind(&self) {
        gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.id)
    }

    pub unsafe fn activate(&self, unit: GLuint) {
        gl::ActiveTexture(unit);
        self.bind();
    }
}

impl Drop for TextureArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}
