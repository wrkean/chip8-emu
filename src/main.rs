mod mods;
use std::time::{Duration, Instant};
use std::{env, thread};
use std::io::Result;

use mods::chip8::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

// Constants for display
const CHIP8_WIDTH: u32 = 64;
const CHIP8_HEIGHT: u32 = 32;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

// Scale for scaling up rendering
const SCALE_X: u32 = WIDTH / CHIP8_WIDTH;
const SCALE_Y: u32 = HEIGHT / CHIP8_HEIGHT;

// Constants for timing
const CPF: u32 = 15;        // Cycles per frame (CPF)
const TIMER_TICK: Duration = Duration::from_millis(16);

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let context = sdl2::init().unwrap();
    context.keyboard().set_mod_state(flags);
    let video = context.video().unwrap();

    let window = video.window("Chip8-Emulator", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

   let mut renderer = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();

    let chip8_fontset: Vec<u8> = vec![ 
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];

    let mut chip8 = Chip8::new();
    chip8.init(&args[1], chip8_fontset)?;              // Take `chip8_fontset`'s ownership here

    let mut last_frame = Instant::now();

    let mut event_poll = context.event_pump().unwrap();
    'running: loop {
        let frame_start = Instant::now();

        for event in event_poll.poll_iter() {
            // Handle the event
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { scancode: Some(sc), repeat, .. } => {
                    if let Some(i) = map_scancode(sc) {
                        if !repeat {
                            chip8.keypad[i] = 1;
                            println!("Scancode pressed: {:?}", sc);
                        }
                    }
                }
                Event::KeyUp { scancode: Some(sc), .. } => {
                    if let Some(i) = map_scancode(sc) {
                        chip8.keypad[i] = 0;
                        println!("Scancode upped: {:?}", sc);
                    }
                }
                _ => { }
            }
        }

        for _ in 0..CPF {
            chip8.emulate_cycle();
        }

        if last_frame.elapsed() >= TIMER_TICK {
            chip8.update_timers();
            last_frame += TIMER_TICK;
        }

        if chip8.draw_flag {
            render_display(&mut renderer, &chip8.display);
            chip8.draw_flag = false;
            renderer.present();
        }

        let elapsed = frame_start.elapsed();
        if elapsed < TIMER_TICK {
            thread::sleep(TIMER_TICK - elapsed);
        }
    }

    Ok(())
}

fn render_display(renderer: &mut Canvas<Window>, display: &[u8]) {
    for y in 0..CHIP8_HEIGHT {
        for x in 0..CHIP8_WIDTH {
            let pixel = display[(y * CHIP8_WIDTH + x) as usize];
            if pixel == 1 {
                renderer.set_draw_color(Color::WHITE);
            } else {
                renderer.set_draw_color(Color::BLACK);
            }

            let _ = renderer.fill_rect(Rect::new(
                (x as i32) * SCALE_X as i32,
                (y as i32) * SCALE_Y as i32,
                SCALE_X,
                SCALE_Y
            ));
        }
    }
}

// Keys are 4x4, from 0-F (0 - 15)
// Keys are mapped like this:
// 1   2   3   4
// Q   W   E   R
// A   S   D   F
// Z   X   C   V
#[allow(non_snake_case)]
fn map_scancode(sc: Scancode) -> Option<usize> {
    use Scancode::*;
    match sc {
        Num1 => Some(0x1),
        Num2 => Some(0x2),
        Num3 => Some(0x3),
        Num4 => Some(0xC),

        Q => Some(0x4),
        W => Some(0x5),
        E => Some(0x6),
        R => Some(0xD),

        A => Some(0x7),
        S => Some(0x8),
        D => Some(0x9),
        F => Some(0xE),

        Z => Some(0xA),
        X => Some(0x0),
        C => Some(0xB),
        V => Some(0xF),

        _ => None,
    }
}
