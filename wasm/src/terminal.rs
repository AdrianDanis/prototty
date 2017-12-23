use prototty_traits::*;
use prototty_grid::*;

const BOLD_BIT: u8 = 1 << 0;
const UNDERLINE_BIT: u8 = 1 << 1;

pub struct Terminal {
    chars: Vec<u32>,
    style: Vec<u8>,
    fg_colour: Vec<u8>,
    bg_colour: Vec<u8>,
}

macro_rules! create_buf {
    ($size:expr) => {
        {
            let mut buf = Vec::with_capacity($size);
            buf.resize($size, 0);
            buf
        }
    }
}

impl Terminal {
    pub fn new() -> Self {

        let size = unsafe {
            (ffi::get_width() * ffi::get_height()) as usize
        };

        let mut chars = create_buf!(size);
        let mut style = create_buf!(size);
        let mut fg_colour = create_buf!(size);
        let mut bg_colour = create_buf!(size);

        unsafe {
            ffi::set_bufs(chars.as_mut_ptr(), style.as_mut_ptr(), fg_colour.as_mut_ptr(), bg_colour.as_mut_ptr());
        }

        Self {
            chars,
            style,
            fg_colour,
            bg_colour,
        }
    }

    pub fn draw_grid(&mut self, grid: &Grid<Cell>) {
        for (cell, chars_entry, style_entry, fg_colour_entry, bg_colour_entry) in
            izip! {
                grid.iter(),
                self.chars.iter_mut(),
                self.style.iter_mut(),
                self.fg_colour.iter_mut(),
                self.bg_colour.iter_mut(),
            }
        {
            *chars_entry = cell.ch as u32;

            *style_entry =
                if cell.bold { BOLD_BIT } else { 0 } |
                    if cell.underline { UNDERLINE_BIT } else { 0 };

            *fg_colour_entry = cell.fg.code();
            *bg_colour_entry = cell.bg.code();
        }

        unsafe {
            ffi::render();
        }
    }

    pub fn size(&self) -> Size {
        unsafe { Size::new(ffi::get_width(), ffi::get_height()) }
    }
}

mod ffi {
    extern "C" {
        pub fn get_width() -> u16;
        pub fn get_height() -> u16;
        pub fn render();
        pub fn set_bufs(chars: *mut u32, style: *mut u8, fg_colour: *mut u8, bg_colour: *mut u8);
    }
}