mod framebuffer;
mod game_of_life;

use framebuffer::Framebuffer;
use game_of_life::GameOfLife;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;

fn render(framebuffer: &mut Framebuffer, game: &GameOfLife) {
    for y in 0..game.height {
        for x in 0..game.width {
            if game.is_alive(x, y) {
                framebuffer.set_current_color(Color::WHITE);
            } else {
                framebuffer.set_current_color(Color::BLACK);
            }
            framebuffer.point(x as u32, y as u32);
        }
    }
}

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

    let mut game = GameOfLife::new(framebuffer_width as usize, framebuffer_height as usize);

    game.set_alive(10, 10, true);
    game.set_alive(11, 10, true);
    game.set_alive(12, 10, true);

    while !window.window_should_close() {
        render(&mut framebuffer, &game);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        game.step();

        thread::sleep(Duration::from_millis(100));
    }
}