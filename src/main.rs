pub mod consts;
pub mod core;
pub mod external;
pub mod utils;

use crate::core::{processor, ram, rom};
use crate::external::{input, output};
use std::env;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        Err("Need to specify rom path")?;
    }
    let rom_path = &args[1];
    let prog = rom::Rom::new(rom_path.as_str())?;

    let ram_ = ram::Ram {
        ..Default::default()
    };

    let display_ram_ = ram::DisplayBuffer {
        ..Default::default()
    };

    let keyboard_buffer_ = ram::KeyboardBuffer {
        ..Default::default()
    };

    let sdl_context = sdl2::init()?;
    let mut chip8 = processor::Processor::new(ram_, display_ram_, keyboard_buffer_);
    let mut keyboard = input::KeyboardDriver::new(&sdl_context, &chip8.keyboard_buffer)?;
    let mut display = output::DisplayDriver::new(&sdl_context, &chip8.display_buffer)?;
    let audio = output::AudioDriver::new(&sdl_context, &chip8.sound_timer)?;

    chip8.init_ram(&prog, &consts::FONT_SET)?;

    loop {
        let keyboard_status = keyboard.poll().is_ok();
        if !keyboard_status {
            break;
        }
        let status = match chip8.cycle() {
            Some(a) => a,
            None => panic!("Failed during execution, exiting..."),
        };
        if *audio.sound_timer.as_ref().borrow() > 0 {
            audio.speaker.resume();
        } else {
            audio.speaker.pause();
        }
        match status {
            processor::CycleStatus::RedrawScreen => {
                display.draw()?;
                display.canvas.present();
            }
            _ => continue,
        }
        thread::sleep(std::time::Duration::from_millis(
            consts::CLOCK_PERIOD as u64,
        ));
    }
    Ok(())
}
