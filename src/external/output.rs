use crate::consts;
use crate::core::ram::DisplayBuffer;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cell::RefCell;
use std::rc::Rc;

pub struct DisplayDriver {
    pub screen: Canvas<Window>,
    pub display_buffer: Rc<RefCell<DisplayBuffer>>,
}

impl DisplayDriver {
    pub fn new(
        context: &sdl2::Sdl,
        display_buffer_: &Rc<RefCell<DisplayBuffer>>,
    ) -> Result<Self, &'static str> {
        let video_subsystem = match context.video() {
            Ok(v) => v,
            Err(_) => return Err("Could not obtain video context"),
        };
        let window = video_subsystem
            .window("CHIP-8 Window", consts::DISPL_WIDTH, consts::DISPL_HEIGHT)
            .build()
            .unwrap();
        let mut canvas: Canvas<Window> = window.into_canvas().present_vsync().build().unwrap();

        canvas.clear();
        canvas.present();

        Ok(DisplayDriver {
            screen: canvas,
            display_buffer: Rc::clone(display_buffer_),
        })
    }
    pub fn draw(&mut self) -> Result<(), &'static str> {
        for (y, row) in self.display_buffer.borrow_mut().buffer.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let i = (x as u32) * consts::SCALE_FACTOR;
                let j = (y as u32) * consts::SCALE_FACTOR;

                self.screen.set_draw_color(match col {
                    0 => Color {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                    1 => Color {
                        r: 0,
                        g: 255,
                        b: 0,
                        a: 0,
                    },
                    _ => return Err("Invalid (non-binary) pixel value"),
                });
                let _ = self.screen.fill_rect(Rect::new(
                    i as i32,
                    j as i32,
                    consts::SCALE_FACTOR,
                    consts::SCALE_FACTOR,
                ));
            }
        }
        self.screen.present();
        Ok(())
    }
}

// Based on https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/audio-squarewave.rs
pub struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
pub struct AudioDriver {
    pub speaker: AudioDevice<SquareWave>,
    pub sound_timer: Rc<RefCell<u8>>,
}

impl AudioDriver {
    pub fn new(context: &sdl2::Sdl, sound_timer_: &Rc<RefCell<u8>>) -> Result<Self, &'static str> {
        let audio_subsystem = match context.audio() {
            Ok(r) => r,
            Err(_) => return Err("Could not obtain audio context"),
        };
        let device = match audio_subsystem.open_playback(
            None,
            &AudioSpecDesired {
                freq: Some(44100),
                channels: Some(1),
                samples: None,
            },
            |spec| SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            },
        ) {
            Ok(r) => r,
            Err(_) => return Err("Failed to initialize audio device"),
        };
        Ok(AudioDriver {
            speaker: device,
            sound_timer: Rc::clone(sound_timer_),
        })
    }
}
