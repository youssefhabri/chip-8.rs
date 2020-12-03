extern crate sdl2;

use sdl2::event::Event;
use std::io;

mod cpu;
mod display;
mod keypad;

use cpu::Cpu;
use display::Display;

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let display = Display::new(sdl_context.to_owned());
    let mut cpu = Cpu::new(display);

    println!("Give the name of the game that you want to load:");
    let mut input_value: String = String::new();
    io::stdin()
        .read_line(&mut input_value)
        .ok()
        .expect("Failed to read line");

    let game = format!("games/{}", input_value);

    cpu.load_game(game);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => cpu.keypad.press(key, true),
                Event::KeyUp {
                    keycode: Some(key), ..
                } => cpu.keypad.press(key, false),
                _ => {}
            }
        }

        cpu.emulate_cycle();
        cpu.display.draw_screen();
    }
}
