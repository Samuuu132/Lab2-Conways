mod framebuffer;

use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    let window_width = 800;
    let window_height = 800;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Conway's Game of Life")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    framebuffer.set_background_color(Color::BLACK);

    while !window.window_should_close() {
        framebuffer.set_current_color(Color::WHITE);
        framebuffer.point(50, 50);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(100));
    }
}