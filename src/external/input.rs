use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::core::ram::KeyboardBuffer;
use std::cell::RefCell;
use std::rc::Rc;

pub struct KeyboardDriver {
    events: sdl2::EventPump,
    pub keyboard_buffer: Rc<RefCell<KeyboardBuffer>>,
}

impl KeyboardDriver {
    pub fn new(
        context: &sdl2::Sdl,
        keyboard_buffer_: &Rc<RefCell<KeyboardBuffer>>,
    ) -> Result<Self, &'static str> {
        Ok(KeyboardDriver {
            events: match context.event_pump() {
                Ok(t) => t,
                Err(_) => return Err("Could not obtain event context"),
            },
            keyboard_buffer: Rc::clone(keyboard_buffer_),
        })
    }

    pub fn poll(&mut self) -> Result<(), &'static str> {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. } => return Err("Received quit event"),
                _ => continue,
            }
        }

        let keys: Vec<Keycode> = self
            .events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        for key in keys {
            let index = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xC),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xD),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xE),
                Keycode::Z => Some(0xA),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xB),
                Keycode::V => Some(0xF),
                Keycode::Escape => return Err("Received interrupt, exiting..."),
                _ => None,
            };

            if let Some(i) = index {
                self.keyboard_buffer.borrow_mut().buffer[i] = 1;
            }
        }
        Ok(())
    }
}
