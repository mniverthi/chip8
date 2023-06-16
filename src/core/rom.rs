use std::fs::File;
use std::io::prelude::*;

use crate::consts;

#[derive(Debug)]
pub struct Rom {
    pub buffer: [u8; consts::MAX_ROM_BYTES],
}

impl Default for Rom {
    fn default() -> Self {
        Rom {
            buffer: [0; consts::MAX_ROM_BYTES],
        }
    }
}

impl Rom {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut data: Rom = Default::default();
        let mut file = File::open(path)?;
        let status = file.read(&mut data.buffer)?;
        if status <= data.buffer.len() {
            return Ok(data);
        }
        return Err("Mismatch in expected ROM size and number of bytes read".into());
    }
}
