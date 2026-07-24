use raylib::prelude::*;

#[allow(dead_code)]
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let background_color = Color::BLACK;
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);

        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(
            self.width as i32,
            self.height as i32,
            self.background_color,
        );
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn point(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer
                .draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    pub fn get_color(&mut self, x: u32, y: u32) -> Color {
        if x < self.width && y < self.height {
            self.color_buffer.get_color(x as i32, y as i32)
        } else {
            self.background_color
        }
    }

    pub fn swap_buffers(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
    ) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let screen_width = window.get_screen_width() as f32;
            let screen_height = window.get_screen_height() as f32;

            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.clear_background(Color::BLACK);

            let source_rec = Rectangle::new(
                0.0,
                0.0,
                self.width as f32,
                self.height as f32,
            );

            let dest_rec = Rectangle::new(
                0.0,
                0.0,
                screen_width,
                screen_height,
            );

            renderer.draw_texture_pro(
                &texture,
                source_rec,
                dest_rec,
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }
    }
}