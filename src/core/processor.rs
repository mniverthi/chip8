use crate::consts;
use crate::core::{ram, rom};
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct Processor {
    pub stack: [u16; consts::STACK_SIZE],
    pub registers: [u8; consts::REG_COUNT],
    pub idx_register: u16,
    pub pc: u16,
    pub stack_pointer: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub ram: Rc<ram::Ram>,
}

impl Processor {
    pub fn init_ram(&mut self, rom: &rom::Rom, fonts: &[u8]) -> Result<(), &str> {
        let _ = match Rc::get_mut(&mut self.ram) {
            Some(s) => {
                s.buffer[0..consts::FONT_SET_SIZE].clone_from_slice(fonts);
                s.buffer[consts::PROG_OFFSET..].clone_from_slice(&rom.buffer);
                Ok(())
            }
            None => Err("Could not copy ROM into RAM"),
        };
        Ok(())
    }
}
