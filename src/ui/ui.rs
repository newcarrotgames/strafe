use rusttype::{point, Font, Scale};
use image::{Rgba, RgbaImage};

use crate::renderer::texture::UITexture;

pub struct UserInterface {
    pub ui_texture: UITexture,
}

impl UserInterface {
    pub fn new(window_size: [u32; 2]) -> Self {
        let ui = unsafe { UITexture::new() };
        log::info!("created UI texture with id {}", ui.id);
        let mut img = RgbaImage::new(window_size[0], window_size[1]);
        let font: &[u8] = include_bytes!("../../assets/fonts/ps.ttf") as &[u8];
        let font = Font::try_from_bytes(font).unwrap();
        let scale = Scale::uniform(42.0);
        let text = "♥";
        let colour = (255, 0, 0);
        let v_metrics = font.v_metrics(scale);
        let glyphs: Vec<_> = font
            .layout(
                text,
                scale,
                point(window_size[0] as f32 - 100.0, 20.0 + v_metrics.ascent),
            )
            .collect();
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    img.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        // Turn the coverage into an alpha value
                        Rgba([colour.0, colour.1, colour.2, (v * 255.0) as u8]),
                    )
                });
            }
        }

        // match image::save_buffer(
        //     &Path::new("ui.png"),
        //     img.as_bytes(),
        //     img.width(),
        //     img.height(),
        //     image::ColorType::Rgba8,
        // ) {
        //     Ok(_) => log::info!("saved image"),
        //     Err(_) => log::info!("could not save image"),
        // };

        unsafe {
            ui.load(&img);
        };

        UserInterface { ui_texture: ui }
    }
}
