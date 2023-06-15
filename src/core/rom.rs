use std::fs::File;
use std::io::prelude::*;

use crate::consts;

#[derive(Debug)]
pub struct Rom {
    buffer: [u8; consts::MAX_ROM_BYTES],
}

impl Default for Rom {
    fn default() -> Self {
        Rom {
            buffer: [0; consts::MAX_ROM_BYTES],
        }
    }
}

impl Rom {
    fn new(path: &str) -> Result<Self, &str> {
        let mut data: Rom = Default::default();
        let mut file = match File::open(path) {
            Ok(s) => s,
            Err(a) => return Err("Failed to open file"),
        };
        let status = match file.read(&mut data.buffer) {
            Ok(s) => s,
            Err(a) => return Err("Failed to read file"),
        };
        if status <= data.buffer.len() {
            return Ok(data);
        }
        return Err("Mismatch in expected ROM size and number of bytes read");
    }
}
