use crate::consts;
use crate::core::{ram, rom};
use crate::utils;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::rc::Rc;

pub enum CycleStatus {
    RedrawScreen,
    Continue,
}

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
    pub display_buffer: Rc<ram::DisplayBuffer>,
    pub keyboard_buffer: Rc<ram::KeyboardBuffer>,
    rng: ThreadRng,
}

impl Processor {
    pub fn new(
        ram_: ram::Ram,
        display_ram_: ram::DisplayBuffer,
        keyboard_buffer_: ram::KeyboardBuffer,
    ) -> Self {
        Processor {
            pc: 0x200,
            ram: Rc::new(ram_),
            display_buffer: Rc::new(display_ram_),
            keyboard_buffer: Rc::new(keyboard_buffer_),
            rng: rand::thread_rng(),
            ..Default::default()
        }
    }
    pub fn init_ram(&mut self, rom: &rom::Rom, fonts: &[u8]) -> Result<(), &'static str> {
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
    pub fn cycle(&mut self) -> Option<CycleStatus> {
        let instr_nibbles = utils::nibble_split(
            &(self.ram.buffer
                [(self.pc) as usize..((self.pc + (consts::OP_CODE_BYTES as u16)) as usize)]),
        );
        self.pc += consts::OP_CODE_BYTES as u16;
        let (opcode, x, y, n) = instr_nibbles;
        let nn = (y << 4) | n;
        let nnn = ((x as u16) << 8) | ((y as u16) << 4) | (n as u16);
        let keyboard = self.keyboard_buffer.buffer.clone();

        match (opcode, x, y, n) {
            // Clears screen
            (0, 0, 0xE, 0) => {
                (Rc::get_mut(&mut self.display_buffer))?
                    .buffer
                    .iter_mut()
                    .for_each(|x| *x = [0 as u8; consts::CHIP8_WIDTH]);

                return Some(CycleStatus::RedrawScreen);
            }

            // Draw on display
            (0xD, _, _, _) => {
                let x_coord = self.registers[x as usize] % (consts::CHIP8_WIDTH as u8);
                let y_coord = self.registers[y as usize] % (consts::CHIP8_HEIGHT as u8);
                let sprite_vals = &self.ram.buffer
                    [(self.idx_register as usize)..((self.idx_register + (n as u16)) as usize)];
                let vram = &mut Rc::get_mut(&mut self.display_buffer)?.buffer;
                for i in 0..n {
                    let curr_sprite_val = sprite_vals[i as usize];
                    for shift_pos in 0..8 {
                        if utils::bounds_check(
                            (x_coord + shift_pos) as usize,
                            (y_coord + i) as usize,
                            consts::CHIP8_WIDTH,
                            consts::CHIP8_HEIGHT,
                        ) {
                            let mask = (1 << (7 - i)) as u8;
                            let should_flip = (mask & curr_sprite_val) >> (7 - i);
                            if should_flip == 1 {
                                if vram[(y_coord + i) as usize][(x_coord + shift_pos) as usize] == 1
                                {
                                    self.registers[0xF] = 1;
                                }
                                vram[(y_coord + i) as usize][(x_coord + shift_pos) as usize] ^= 1;
                            }
                        } else {
                            break;
                        }
                    }
                }
                return Some(CycleStatus::RedrawScreen);
            }

            // Jump to subroutine
            (1, _, _, _) => {
                self.pc = nnn;
            }
            (0xB, _, _, _) => {
                self.pc = nnn.wrapping_add(self.registers[0] as u16);
            }

            // Subroutines: enter and exit
            (0, 0, 0xE, 0xE) => {
                self.stack_pointer -= 1;
                self.pc = self.stack[self.stack_pointer as usize];
            }
            (2, _, _, _) => {
                self.stack[self.stack_pointer as usize] = self.pc;
                self.stack_pointer += 1;
                self.pc = nnn;
            }

            // Conditional skips
            (3, _, _, _) => {
                let vx_data = self.registers[x as usize];
                if vx_data == nn {
                    self.pc += consts::OP_CODE_BYTES as u16;
                }
            }
            (4, _, _, _) => {
                let vx_data = self.registers[x as usize];
                if vx_data != nn {
                    self.pc += consts::OP_CODE_BYTES as u16;
                }
            }
            (5, _, _, 0) => {
                let vx_data = self.registers[x as usize];
                let vy_data = self.registers[y as usize];
                if vx_data == vy_data {
                    self.pc += consts::OP_CODE_BYTES as u16;
                }
            }
            (9, _, _, 0) => {
                let vx_data = self.registers[x as usize];
                let vy_data = self.registers[y as usize];
                if vx_data != vy_data {
                    self.pc += consts::OP_CODE_BYTES as u16;
                }
            }

            // Set register
            (6, _, _, _) => {
                self.registers[x as usize] = nn;
            }
            (8, _, _, 0) => {
                self.registers[x as usize] = self.registers[y as usize];
            }
            (0xA, _, _, _) => {
                self.idx_register = nnn;
            }

            // Add/subtract instructions
            (7, _, _, _) => {
                self.registers[x as usize] = self.registers[x as usize].wrapping_add(nn);
            }
            (8, _, _, 4) => {
                if ((self.registers[x as usize] as u16) + (self.registers[y as usize] as u16)) > 255
                {
                    self.registers[0xF as usize] = 1;
                } else {
                    self.registers[0xF as usize] = 0;
                }
                self.registers[x as usize] =
                    self.registers[y as usize].wrapping_add(self.registers[y as usize]);
            }
            (8, _, _, 5) => {
                if self.registers[x as usize] > self.registers[y as usize] {
                    self.registers[0xF as usize] = 1;
                } else {
                    self.registers[0xF as usize] = 0;
                }
                self.registers[x as usize] =
                    self.registers[x as usize].wrapping_sub(self.registers[y as usize]);
            }
            (8, _, _, 7) => {
                if self.registers[y as usize] > self.registers[x as usize] {
                    self.registers[0xF as usize] = 1;
                } else {
                    self.registers[0xF as usize] = 0;
                }
                self.registers[x as usize] =
                    self.registers[y as usize].wrapping_sub(self.registers[x as usize]);
            }

            // Logical instructions
            (8, _, _, 1) => {
                self.registers[x as usize] |= self.registers[y as usize];
            }
            (8, _, _, 2) => {
                self.registers[x as usize] &= self.registers[y as usize];
            }
            (8, _, _, 3) => {
                self.registers[x as usize] ^= self.registers[y as usize];
            }

            // Shifting instructions
            (8, _, _, 6) => {
                self.registers[0xF] = self.registers[x as usize] & 0b00000001;
                self.registers[x as usize] >>= 1;
            }
            (8, _, _, 0xE) => {
                self.registers[0xF] = (self.registers[x as usize] & 0b10000000) >> 7;
                self.registers[x as usize] <<= 1;
            }

            // Generate randomness
            (0xC, _, _, _) => {
                let rand_val: u8 = self.rng.gen();
                self.registers[x as usize] = nn & rand_val;
            }

            // Skip on keypress
            (0xE, _, 9, 0xE) => {
                if keyboard[self.registers[x as usize] as usize] == 1 {
                    self.pc += consts::OP_CODE_BYTES as u16;
                }
            }
            (0xE, _, 0xA, 1) => {
                if keyboard[self.registers[x as usize] as usize] != 1 {
                    self.pc += consts::OP_CODE_BYTES as u16;
                }
            }

            // Change timers (delay/sound)
            (0xF, _, 0, 7) => {
                self.registers[x as usize] = self.delay_timer;
            }
            (0xF, _, 1, 5) => {
                self.delay_timer = self.registers[x as usize];
            }
            (0xF, _, 1, 8) => {
                self.sound_timer = self.registers[x as usize];
            }

            // Update index register
            (0xF, _, 1, 0xE) => {
                self.idx_register += self.registers[x as usize] as u16;
            }

            // Halt till keyboard interrupt
            (0xF, _, 0, 0xA) => {
                if keyboard.iter().all(|x| *x == 0) {
                    self.pc -= consts::OP_CODE_BYTES as u16;
                } else {
                    for i in 0..consts::KEYBOARD_SIZE {
                        if keyboard[i] == 1 {
                            self.registers[x as usize] = i as u8;
                            break;
                        }
                    }
                }
            }

            // Point index to font character
            (0xF, _, 2, 9) => {
                self.idx_register = (self.registers[x as usize] * 5) as u16;
            }

            // Binary byte to decimal string representation conversion
            (0xF, _, 3, 3) => {
                let num = self.registers[x as usize];
                let first_digit = num / 100;
                let second_digit = (num % 100) / 10;
                let third_digit = num % 10;
                let ram_ref = &mut Rc::get_mut(&mut self.ram)?.buffer;
                ram_ref[self.idx_register as usize] = first_digit;
                ram_ref[(self.idx_register + 1) as usize] = second_digit;
                ram_ref[(self.idx_register + 2) as usize] = third_digit;
            }

            // Store and load memory
            (0xF, _, 5, 5) => {
                let ram_ref = &mut Rc::get_mut(&mut self.ram)?.buffer;
                for i in 0..(x + 1) {
                    ram_ref[(self.idx_register + i as u16) as usize] = self.registers[i as usize];
                }
            }
            (0xF, _, 6, 5) => {
                let ram_ref = self.ram.buffer;
                for i in 0..(x + 1) {
                    self.registers[i as usize] = ram_ref[(self.idx_register + i as u16) as usize];
                }
            }

            // Invalid/unsupported opcodes
            (0, _, _, _) => {
                panic!("Calling machine language routine, unsupported on this architecture")
            }
            (_, _, _, _) => {
                panic!(
                    "Invalid instruction, received opcode: {}, x: {}, y: {}, n: {}",
                    opcode, x, y, n
                )
            }
        }
        Some(CycleStatus::Continue)
    }
}

#[cfg(test)]
mod tests {
    use crate::consts;
    use crate::processor::Processor;
    use crate::{ram, rom};
    use std::rc::Rc;

    const START_PC: u16 = 0xF00;
    const NEXT_PC: u16 = START_PC + (consts::OP_CODE_BYTES as u16);
    const SKIPPED_PC: u16 = START_PC + ((2 * consts::OP_CODE_BYTES) as u16);

    fn update_buffer(buffer: &mut [u8], address: usize, value: u8) {
        buffer[address] = value;
    }

    fn build_processor() -> Result<Processor, &'static str> {
        let ram_ = ram::Ram {
            ..Default::default()
        };

        let display_ram_ = ram::DisplayBuffer {
            ..Default::default()
        };

        let keyboard_buffer_ = ram::KeyboardBuffer {
            ..Default::default()
        };
        let mut proc = Processor::new(ram_, display_ram_, keyboard_buffer_);
        proc.pc = START_PC;
        proc.registers = [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7];

        Ok(proc)
    }

    #[test]
    fn test_initial_state() -> Result<(), &'static str> {
        let ram_ = ram::Ram {
            ..Default::default()
        };

        let display_ram_ = ram::DisplayBuffer {
            ..Default::default()
        };

        let keyboard_buffer_ = ram::KeyboardBuffer {
            ..Default::default()
        };
        let mut proc = Processor::new(ram_, display_ram_, keyboard_buffer_);
        assert_eq!(proc.pc, 0x200);
        assert_eq!(proc.stack_pointer, 0);
        assert_eq!(proc.stack, [0; 16]);

        proc.init_ram(&rom::Rom::default(), &consts::FONT_SET)?;

        // First char in font: 0
        assert_eq!(proc.ram.buffer[0..5], [0xF0, 0x90, 0x90, 0x90, 0xF0]);
        // Last char in font: F
        assert_eq!(
            proc.ram.buffer[consts::FONT_SET.len() - 5..consts::FONT_SET.len()],
            [0xF0, 0x80, 0xF0, 0x80, 0x80]
        );
        Ok(())
    }

    #[test]
    fn test_opcode_00e0() -> Result<(), &'static str> {
        let mut processor = build_processor()?;

        let display_ram = match Rc::get_mut(&mut processor.display_buffer) {
            Some(t) => &mut t.buffer,
            None => return Err("Failed test, could not retrieve display buffer"),
        };

        let ram = match Rc::get_mut(&mut processor.ram) {
            Some(t) => &mut t.buffer,
            None => return Err("Failed test, could not retrieve ram buffer"),
        };

        *display_ram = [[128; consts::CHIP8_WIDTH]; consts::CHIP8_HEIGHT];
        update_buffer(ram, (START_PC + 1) as usize, 0xE0);

        processor.cycle();

        for y in 0..consts::CHIP8_HEIGHT {
            for x in 0..consts::CHIP8_WIDTH {
                assert_eq!(processor.display_buffer.buffer[y][x], 0);
            }
        }
        assert_eq!(processor.pc, NEXT_PC);
        Ok(())
    }

    #[test]
    fn test_opcode_00ee() -> Result<(), &'static str> {
        let mut processor = build_processor()?;

        let ram = match Rc::get_mut(&mut processor.ram) {
            Some(t) => &mut t.buffer,
            None => return Err("Failed test, could not retrieve ram buffer"),
        };

        update_buffer(ram, (START_PC + 1) as usize, 0xEE);

        processor.stack_pointer = 3;
        processor.stack[2] = 0x1234;

        processor.cycle();

        assert_eq!(processor.stack_pointer, 2);
        assert_eq!(processor.pc, 0x1234);
        Ok(())
    }

    #[test]
    fn test_opcode_1nnn() -> Result<(), &'static str> {
        let mut processor = build_processor()?;

        let ram = match Rc::get_mut(&mut processor.ram) {
            Some(t) => &mut t.buffer,
            None => return Err("Failed test, could not retrieve ram buffer"),
        };

        update_buffer(ram, (START_PC) as usize, 0x11);
        update_buffer(ram, (START_PC + 1) as usize, 0x23);
        processor.cycle();
        assert_eq!(processor.pc, 0x0123);
        assert_eq!(processor.stack_pointer, 0);
        Ok(())
    }

    #[test]
    fn test_opcode_2nnn() -> Result<(), &'static str> {
        let mut processor = build_processor()?;

        let ram = match Rc::get_mut(&mut processor.ram) {
            Some(t) => &mut t.buffer,
            None => return Err("Failed test, could not retrieve ram buffer"),
        };

        update_buffer(ram, (START_PC) as usize, 0x21);
        update_buffer(ram, (START_PC + 1) as usize, 0x23);
        processor.cycle();
        assert_eq!(processor.pc, 0x0123);
        assert_eq!(processor.stack_pointer, 1);
        assert_eq!(processor.stack[0], NEXT_PC);
        Ok(())
    }
}
