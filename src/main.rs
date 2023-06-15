pub mod consts;
mod core;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        Err("Need to specify rom path")?;
    }
    let rom_path = &args[1];

    Ok(())
}
