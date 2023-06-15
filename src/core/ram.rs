use crate::consts;

#[derive(Debug)]
pub struct Ram {
    buffer: [u8; consts::RAM_BYTES],
}

impl Default for Ram {
    fn default() -> Self {
        Ram {
            buffer: [0; consts::RAM_BYTES],
        }
    }
}
