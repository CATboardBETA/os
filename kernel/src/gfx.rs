use core::fmt;
use core::fmt::{Arguments, Write};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::iso_8859_1::FONT_10X20;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;
use limine::framebuffer::Framebuffer;
use spin::{Mutex, Once};

pub struct Mutexx<T>(pub(crate) Mutex<T>);

// This is probably fine...
unsafe impl<T> Send for Mutexx<T> {}
unsafe impl<T> Sync for Mutexx<T> {}

pub(crate) static DISPLAY: Mutexx<Once<Display>> = Mutexx(Mutex::new(Once::new()));

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::gfx::_print(format_args!($($arg)*)));
}

pub fn _print(args: Arguments) {
    DISPLAY.0.lock().get_mut().unwrap().write_fmt(args).unwrap();
}

#[derive(Copy, Clone, Default)]
pub(crate) struct TextInfo {
    pub(crate) pos: (i32, i32),
}

pub struct Display {
    pub(crate) inner: &'static Framebuffer,
    pub(crate) text_info: TextInfo,
}

impl Write for Mutexx<Once<Display>> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.lock().get_mut().unwrap().write_str(s)
    }
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.0.lock().get_mut().unwrap().write_char(c)
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> fmt::Result {
        self.0.lock().get_mut().unwrap().write_fmt(args)
    }
}

impl Write for Display {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c)?;
        }
        Ok(())
    }
    fn write_char(&mut self, c: char) -> fmt::Result {
        let mut buf = [0u8; 4];
        if self.text_info.pos.0 + 1 > self.inner.width as i32 {
            self.text_info.pos.0 = 0;
            self.text_info.pos.1 += 20;
        }
        let text = Text::new(
            c.encode_utf8(&mut buf),
            self.text_info.pos.into(),
            MonoTextStyle::new(&FONT_10X20, Rgb888::WHITE),
        );
        self.text_info.pos = text.draw(self).map_err(|_| fmt::Error)?.into();
        if c == '\n' {
            self.text_info.pos.0 = 0;
        }
        Ok(())
    }
}

impl DrawTarget for Display {
    type Color = Rgb888;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            let width = self.inner.width as i32;
            let height = self.inner.height as i32;
            if let (x, y) = coord.into()
                && 0 <= x
                && x < width
                && 0 <= y
                && y < height
            {
                // Calculate the index in the framebuffer.
                let index = (x + y * width) as u32;
                unsafe {
                    (&mut *core::slice::from_raw_parts_mut(
                        self.inner.address() as *mut u32,
                        self.inner.size(),
                    ))[index as usize] = color.into_storage();
                }
            }
        }

        Ok(())
    }
}

impl OriginDimensions for Display {
    fn size(&self) -> Size {
        Size::new(self.inner.width as _, self.inner.height as _)
    }
}
