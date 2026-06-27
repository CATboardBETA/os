//! All font rendering. Primary type is a wrapper type around [`Gfx`], [`GfxFont`].

use crate::gfx::Gfx;
use core::ops::Deref;
use swash::scale::{Render, ScaleContext, Source, StrikeWith};
use swash::zeno::{Format, Placement, Vector};
use swash::FontRef;

/// Simple wrapper around [`Gfx`], but allows for drawing with a font.
pub struct GfxFont<'gfx> {
    /// Internal graphics struct. Contains a reference to the framebuffer.
    ///
    /// All methods from [`Gfx`] can be accessed directly, because [`GfxFont`] implements
    /// [`Deref`] with  `Target=Gfx`.
    pub(super) gfx: &'gfx Gfx<'gfx>,
    /// The font. This struct comes from [`swash`], which we use for rendering.
    pub(super) font: FontRef<'gfx>,
    /// Used to build fonts given font size and other parameters. Provided by [`swash`]
    pub(super) ctx: ScaleContext,
    /// Font size
    pub(super) size: f32,
    /// Whether to [hint](https://en.wikipedia.org/wiki/Font_hinting) the font or not
    pub(super) hint: bool,
}

impl<'gfx> Deref for GfxFont<'gfx> {
    type Target = Gfx<'gfx>;

    fn deref(&self) -> &Self::Target {
        self.gfx
    }
}

impl GfxFont<'_> {
    /// Draws a char to the screen,
    pub unsafe fn draw_char(&mut self, c: char, x: f32, y: f32) {
        let mut scaler = self
            .ctx
            .builder(self.font)
            .size(self.size)
            .hint(self.hint)
            .build();
        let offset = Vector::new(x, y);
        let image = Render::new(&[
            Source::ColorOutline(0),
            Source::ColorBitmap(StrikeWith::BestFit),
            Source::Outline,
        ])
            .format(Format::Subpixel)
            .offset(offset)
            .render(&mut scaler, self.font.charmap().map(c)).unwrap();
        let Placement { left, top, width, height: _ } = image.placement;
        let data = image.data;
        unsafe {
            self.draw_image_unchecked(data, left as usize, top as usize, width as usize);
        }
    }
}
