pub mod consts;
mod core;
use crate::core::{processor, ram, rom};
use std::env;
use std::rc::Rc;

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

    let mut chip8 = processor::Processor {
        stack_pointer: 0,
        ram: Rc::new(ram_),
        ..Default::default()
    };
    chip8.init_ram(&prog, &consts::FONT_SET)?;
    println!("{:?}", chip8);
    Ok(())
}
