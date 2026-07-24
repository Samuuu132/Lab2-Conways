mod framebuffer;
mod game_of_life;
mod patterns;

use framebuffer::Framebuffer;
use game_of_life::GameOfLife;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;

fn render(framebuffer: &mut Framebuffer, game: &GameOfLife) {
    for y in 0..game.height {
        for x in 0..game.width {
            if game.is_alive(x, y) {
                framebuffer.set_current_color(Color::new(0, 255, 70, 255));
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

    game.set_cells(5, 5, patterns::GLIDER);
    game.set_cells(20, 5, patterns::LWSS);
    game.set_cells(40, 5, patterns::PULSAR);
    game.set_cells(60, 5, patterns::BLOCK);
    game.set_cells(70, 5, patterns::BEEHIVE);

    game.set_cells(5, 30, patterns::BLINKER);
    game.set_cells(15, 30, patterns::TOAD);
    game.set_cells(25, 30, patterns::BEACON);
    game.set_cells(40, 30, patterns::LOAF);
    game.set_cells(55, 30, patterns::BOAT);

    game.set_cells(10, 60, patterns::GLIDER);
    game.set_cells(30, 60, patterns::LWSS);
    game.set_cells(50, 60, patterns::PULSAR);
    game.set_cells(75, 60, patterns::BLINKER);

    game.set_cells(15, 85, patterns::TOAD);
    game.set_cells(35, 85, patterns::BEACON);
    game.set_cells(60, 85, patterns::GLIDER);

    while !window.window_should_close() {
        game.step();

        render(&mut framebuffer, &game);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(100));
    }
}