mod framebuffer;
mod game_of_life;
mod patterns;

use framebuffer::Framebuffer;
use game_of_life::GameOfLife;
use raylib::prelude::*;
use std::borrow::Cow;
use std::fs::File;
use std::thread;
use std::time::Duration;

fn setup_game(game: &mut GameOfLife) {
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
}

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

fn generate_gif(game: &mut GameOfLife) {
    let scale: usize = 4;
    let gif_width = (game.width * scale) as u16;
    let gif_height = (game.height * scale) as u16;
    let total_frames = 60;
    let delay = 10u16;

    let color_map = &[
        0u8, 0, 0,
        0, 255, 70,
    ];

    let mut file = File::create("demo.gif").expect("No se pudo crear demo.gif");
    let mut encoder = gif::Encoder::new(&mut file, gif_width, gif_height, color_map)
        .expect("No se pudo crear el encoder GIF");
    encoder.set_repeat(gif::Repeat::Infinite).unwrap();

    for _ in 0..total_frames {
        let mut pixels = vec![0u8; gif_width as usize * gif_height as usize];

        for y in 0..game.height {
            for x in 0..game.width {
                let color_idx = if game.is_alive(x, y) { 1u8 } else { 0u8 };
                for sy in 0..scale {
                    for sx in 0..scale {
                        let px = x * scale + sx;
                        let py = y * scale + sy;
                        pixels[py * gif_width as usize + px] = color_idx;
                    }
                }
            }
        }

        let frame = gif::Frame {
            width: gif_width,
            height: gif_height,
            delay,
            buffer: Cow::Owned(pixels),
            ..gif::Frame::default()
        };
        encoder.write_frame(&frame).expect("No se pudo escribir el frame GIF");
        game.step();
    }

    println!("GIF guardado en demo.gif");
}

fn main() {
    let framebuffer_width: usize = 100;
    let framebuffer_height: usize = 100;

    let mut gif_game = GameOfLife::new(framebuffer_width, framebuffer_height);
    setup_game(&mut gif_game);
    generate_gif(&mut gif_game);

    let window_width = 800;
    let window_height = 800;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Conway's Game of Life")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(framebuffer_width as u32, framebuffer_height as u32);
    framebuffer.set_background_color(Color::BLACK);

    let mut game = GameOfLife::new(framebuffer_width, framebuffer_height);
    setup_game(&mut game);

    while !window.window_should_close() {
        game.step();

        render(&mut framebuffer, &game);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(100));
    }
}