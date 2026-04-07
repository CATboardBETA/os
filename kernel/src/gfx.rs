use core::fmt::Write;
use embedded_graphics::mono_font::iso_8859_1::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use limine::framebuffer::Framebuffer;

#[macro_export]
macro_rules! println {
    () => {};
}

#[derive(Copy, Clone, Default)]
pub(crate) struct TextInfo {
    pos: (i32, i32),
}

pub struct Display {
    pub(crate) inner: &'static Framebuffer,
    pub(crate) text_info: TextInfo,
}

impl Write for Display {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.text_info.pos = match embedded_graphics::text::Text::new(
            s,
            self.text_info.pos.into(),
            MonoTextStyle::new(&FONT_10X20, Rgb888::WHITE),
        )
        .draw(self)
        {
            Ok(i) => i.into(),
            Err(()) => panic!(),
        };
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
            if let Ok((x, y)) = coord.try_into()
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
