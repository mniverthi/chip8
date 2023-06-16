use crate::core::ram::DisplayRam;
use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct DisplayDriver {
    pub window: Canvas<Window>,
    pub vram: Rc<DisplayRam>,
}
