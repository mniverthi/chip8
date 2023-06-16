use crate::consts;

#[derive(Debug)]
pub struct Ram {
    pub buffer: [u8; consts::RAM_BYTES],
}

impl Default for Ram {
    fn default() -> Self {
        Ram {
            buffer: [0; consts::RAM_BYTES],
        }
    }
}

#[derive(Debug)]
pub struct DisplayRam {
    pub buffer: [[u8; consts::CHIP8_WIDTH]; consts::CHIP8_HEIGHT],
}

impl Default for DisplayRam {
    fn default() -> Self {
        DisplayRam {
            buffer: [[0; consts::CHIP8_WIDTH]; consts::CHIP8_HEIGHT],
        }
    }
}
