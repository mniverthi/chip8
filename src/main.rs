pub mod consts;
pub mod core;
pub mod utils;

use crate::core::{processor, ram, rom};
use std::env;

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
    let mut chip8 = processor::Processor::new(ram_, display_ram_, keyboard_buffer_);
    chip8.init_ram(&prog, &consts::FONT_SET)?;
    println!("{:?}", chip8);
    Ok(())
}
