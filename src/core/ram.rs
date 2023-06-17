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
pub struct DisplayBuffer {
    pub buffer: [[u8; consts::CHIP8_WIDTH]; consts::CHIP8_HEIGHT],
}

impl Default for DisplayBuffer {
    fn default() -> Self {
        DisplayBuffer {
            buffer: [[0; consts::CHIP8_WIDTH]; consts::CHIP8_HEIGHT],
        }
    }
}

#[derive(Default, Debug)]
pub struct KeyboardBuffer {
    pub buffer: [u8; consts::KEYBOARD_SIZE],
}
