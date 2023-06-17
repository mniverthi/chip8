use crate::consts;
pub fn nibble_split(bytes: &[u8]) -> (u8, u8, u8, u8) {
    assert!(bytes.len() == consts::OP_CODE_BYTES);
    (
        (bytes[0] & 0xF0) >> 4,
        bytes[0] & 0x0F,
        (bytes[1] & 0xF0) >> 4,
        bytes[1] & 0x0F,
    )
}

pub fn bounds_check(x: usize, y: usize, width: usize, height: usize) -> bool {
    x < width && y < height
}
