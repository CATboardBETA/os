//! All font rendering. Primary type is a wrapper type around [`Gfx`], [`GfxFont`].

use crate::gfx::{DrawError, DrawResult, Gfx};
use ab_glyph::{Font, FontRef, ScaleFont};
use core::ops::Deref;

/// Simple wrapper around [`Gfx`], but allows for drawing with a font.
pub struct GfxFont<'gfx> {
    /// Internal graphics struct. Contains a reference to the framebuffer.
    ///
    /// All methods from [`Gfx`] can be accessed directly, because [`GfxFont`] implements
    /// [`Deref`] with  `Target=Gfx`.
    pub(super) gfx: &'gfx Gfx<'gfx>,
    /// The font. This struct comes from [`ab_glyph`], which we use for rasterizing.
    pub(super) font: FontRef<'gfx>,
    /// Font size
    pub(super) size: f32,
}

impl<'gfx> Deref for GfxFont<'gfx> {
    type Target = Gfx<'gfx>;

    fn deref(&self) -> &Self::Target {
        self.gfx
    }
}

impl GfxFont<'_> {
    /// Draws a string at the specified (x,y) coordinate. If `max_width` is `Some`, then this
    /// function will not draw past `x + max_width`
    pub fn draw_str<S: AsRef<str>>(
        &mut self,
        s: S,
        mut x: f32,
        mut y: f32,
        max_width: Option<f32>,
    ) -> DrawResult<(f32, f32)> {
        let og_x = x;
        for c in s.as_ref().chars() {
            match self.draw_char(c, x, y, max_width.map(|w| w + og_x)) {
                Ok((x_adv, y_adv)) => {
                    x += x_adv;
                    y += y_adv;
                }
                Err(DrawError::OutOfBounds) => {
                    x = og_x;
                    let font = self.font.as_scaled(self.size);
                    y += font.height() + font.line_gap();
                    let (x_adv, y_adv) = self.draw_char(c, x, y, max_width.map(|w| w + og_x))?;
                    x += x_adv;
                    y += y_adv;
                }
                Err(e) => return Err(e),
            }
        }
        Ok((x, y))
    }

    /// Draws a char to the screen, at (x, y)
    #[allow(clippy::cast_precision_loss)]
    pub fn draw_char(
        &mut self,
        c: char,
        x: f32,
        y: f32,
        max_px: Option<f32>,
    ) -> DrawResult<(f32, f32)> {
        let font = self.font.as_scaled(self.size);
        let mut glyphs = font.scaled_glyph(c);
        glyphs.position = (x, y).into();
        let id = glyphs.id;
        if let Some(max_px) = max_px
            && (x + font.h_advance(id)) >= max_px
        {
            return Err(DrawError::OutOfBounds);
        }
        if x < 0.
            || y < 0.
            || (x + font.h_advance(id)) >= self.fb.width as f32
            || (y + font.height() + font.line_gap()) >= self.gfx.fb.height as f32
        {
            return Err(DrawError::OutOfBounds);
        }

        let glyph = unsafe { self.font.outline_glyph(glyphs) };
        let mut h_adv = 0.;
        let mut v_adv = 0.;
        if c == '\n' {
            h_adv = 0.;
            v_adv = font.height() + font.line_gap();
        } else {
            h_adv = font.h_advance(id);
            v_adv = 0.;
        }
        if let Some(glyph) = glyph {
            let bounds = glyph.px_bounds();
            glyph.draw(|x, y, c| {
                let x = x + bounds.min.x as u32;
                let y = y + bounds.min.y as u32;
                let c = (c * 255.) as u8;
                unsafe {
                    self.write_px_unchecked(x as usize, y as usize, [c, c, c, c]);
                }
            });
        }
        Ok((h_adv, v_adv))
    }
    /// Draws a char to the screen, without bounds checks
    pub unsafe fn draw_char_unchecked(&mut self, c: char, x: f32, y: f32) -> (f32, f32) {
        let font = self.font.as_scaled(self.size);
        let mut glyphs = font.scaled_glyph(c);
        glyphs.position = (x, y).into();
        let id = glyphs.id;
        let glyph = unsafe { self.font.outline_glyph(glyphs) };
        let mut h_adv = 0.;
        let mut v_adv = 0.;
        if c == '\n' {
            h_adv = 0.;
            v_adv = font.height() + font.line_gap();
        } else {
            h_adv = font.h_advance(id);
            v_adv = 0.;
        }
        if let Some(glyph) = glyph {
            let bounds = glyph.px_bounds();
            glyph.draw(|x, y, c| {
                let x = x + bounds.min.x as u32;
                let y = y + bounds.min.y as u32;
                let c = (c * 255.) as u8;
                unsafe {
                    self.write_px_unchecked(x as usize, y as usize, [c, c, c, c]);
                }
            });
        }
        (h_adv, v_adv)
    }

    /// Updates the font size to `new`
    pub fn set_size(&mut self, new: f32) {
        self.size = new;
    }

    /// Returns the current font size
    pub fn get_size(&self) -> f32 {
        self.size
    }
}
