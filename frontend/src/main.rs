use chip_8_core::*;
use sdl2::event::Event;
use std::{env, fs};

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Please do cargo run path/to/game");
        return;
    }

    let mut emulator = Emu::new();
    read_rom(&mut emulator, &args[1]);

    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();
    let window = video_subsystem
        .window("Chip-8 Simulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl2_context.event_pump().unwrap();

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'gameloop,
                _ => (),
            }
        }
    }
}

fn read_rom(emulator: &mut Emu, filepath: &str) {
    let file: Vec<u8> = fs::read(filepath).unwrap();
}
