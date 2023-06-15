use crate::consts;
use crate::core::ram;
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct Processor {
    stack: [u16; consts::STACK_SIZE],
    registers: [u8; consts::REG_COUNT],
    idx_register: u16,
    pc: u16,
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
    ram: Rc<ram::Ram>,
}

impl Processor {}
