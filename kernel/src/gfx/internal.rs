//! Internal implementations of all graphics necessary for the kernel. All public items are
//! re-exported in [`gfx`](super),
//!
//! TODO: Make color generic

use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{max, Ordering};
use core::slice;
use limine::framebuffer::Framebuffer;

/// Intended only for likely unsafe implementations, or implementations that depend on other checks
/// outside the scope of a trait method.
trait UnsafeSliceFramebuffer {
    /// Converts into a mutable slice of u32; u32 is assumed to be the 4-byte color repr.
    unsafe fn to_mut_slice<'a>(&self) -> &'a mut [u32];
}

impl UnsafeSliceFramebuffer for Framebuffer {
    unsafe fn to_mut_slice<'a>(&self) -> &'a mut [u32] {
        unsafe {
            slice::from_raw_parts_mut(
                self.address().cast::<u32>(),
                (self.width * self.height) as usize,
            )
        }
    }
}

/// An error returned by a drawing function. Used as [`DrawResult`]'s `E` type
#[derive(Debug)]
pub enum DrawError {
    /// Tried to draw off the screen
    OutOfBounds,
    /// Character not in provided font.
    NoSuchGlyph(char),
}

/// A type alias of [`Result`], because all the checked drawing functions return the same type.
pub type DrawResult = Result<(), DrawError>;

///
/// TODO: Change to be generic, or an enum so that we can draw in multiple colorspaces
#[derive(Copy, Clone, Debug)]
pub struct Color {
    /// Red component
    pub r: u8,
    /// Green component
    pub g: u8,
    /// Blue component
    pub b: u8,
}

impl Color {
    // Allow: These associated constants seem pretty self-explanatory
    #![allow(clippy::missing_docs_in_private_items)]
    #![allow(missing_docs)]
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };
}

impl From<Color> for [u8; 4] {
    fn from(value: Color) -> Self {
        [255, value.r, value.g, value.b]
    }
}


/// The core `struct` in this module is [`Gfx`]. Its `impl`s hold all the drawing functions,
/// and it even contains a reference to the underlying `Framebuffer` passed from [`limine`]
///
/// To swap the buffers, use [`Gfx::swap_buffers()`]
pub struct Gfx<'fb> {
    /// Internal framebuffer. A vector of Framebuffers is passed in a limine request.
    fb: &'fb Framebuffer,
    /// The second buffer. This allows for drawing on a separate buffer, or switching swapping
    /// between buffers.
    /// TODO: Make functions generic over a `Buffer` type
    double_buffer: Vec<u32>,
    /// Pitch in bytes
    pitch: usize,
    /// width in pixels
    width: usize,
    /// height in pixels
    height: usize,
    /// Position of text. This should probably be extracted to a separate struct,
    /// something like `TextInfo` which has to be passed by `&mut` alongside text draws. Or a
    /// wrapper for [`Gfx`] exclusively for drawing text.
    pos: (usize, usize),
}

impl<'fb> From<&'fb Framebuffer> for Gfx<'fb> {
    fn from(value: &'fb Framebuffer) -> Self {
        let width = value.width as _;
        let height = value.height as _;
        let pitch = value.pitch as _;
        Self {
            fb: value,
            double_buffer: vec![0u32; height * width],
            pitch,
            width,
            height,
            pos: (0, 0),
        }
    }
}

impl Gfx<'_> {
    /// Clears the entire screen with a certain color. This should be  more efficient
    /// compared to other methods on this struct to no bounds checks being required whatsoever.
    pub fn clear(&self, color: Color) {
        unsafe {
            slice::from_raw_parts_mut(self.fb.address().cast::<u32>(), self.width * self.height)
                .fill(u32::from_be_bytes(color.into()));
        }
    }

    /// Swaps the Framebuffer's memory with the double buffer's memory
    pub fn swap_buffers(&mut self) {
        self.double_buffer
            .swap_with_slice(unsafe { self.fb.to_mut_slice() });
    }

    /// Writes a pixel, without bounds checking.
    ///
    /// # Safety:
    /// If `x` or `y` are off the screen, this is UB
    pub unsafe fn write_px_unchecked(&self, x: usize, y: usize, color: [u8; 4]) {
        let offset = y * self.width + x;
        unsafe {
            let slice = slice::from_raw_parts_mut(
                self.fb.address().cast::<u32>(),
                self.width * self.height,
            );
            slice[offset] = u32::from_be_bytes(color);
        }
    }

    /// Writes a pixel with
    pub fn write_px(&self, x: usize, y: usize, color: Color) -> DrawResult {
        if x > self.width || y > self.height {
            Err(DrawError::OutOfBounds)
        } else {
            unsafe {
                self.write_px_unchecked(x, y, color.into());
                Ok(())
            }
        }
    }

    /// Writes an iterator of pixels. Avoid this when possible, because bounds checking must be
    /// performed on every single point.
    pub fn write_px_iter(
        &self,
        iter: impl IntoIterator<Item = (usize, usize)>,
        color: Color,
    ) -> DrawResult {
        for px in iter {
            self.write_px(px.0, px.1, color)?;
        }
        Ok(())
    }

    /// Same as [`Self::write_px_iter`], but the iterator is unzipped.
    pub fn write_px_iter_split(
        &self,
        iter1: impl IntoIterator<Item = usize>,
        iter2: impl IntoIterator<Item = usize>,
        color: Color,
    ) -> DrawResult {
        for (x, y) in iter1.into_iter().zip(iter2) {
            self.write_px(x, y, color)?;
        }
        Ok(())
    }

    /// Draws a line, using Bresenham's algorithm. This does not perform antialiasing
    ///
    /// Bounds checking **is** performed.
    pub fn draw_line(
        &self,
        (x0, y0): (usize, usize),
        (x1, y1): (usize, usize),
        color: Color,
    ) -> DrawResult {
        if max(y0, y1) > self.height || max(x0, x1) > self.width {
            Err(DrawError::OutOfBounds)
        } else {
            unsafe { self.draw_line_unchecked((x0, y0), (x1, y1), color) }
            Ok(())
        }
    }


    /// Draws a line, using Bresenham's algorithm. This does not perform antialiasing
    ///
    /// Bounds checking **is not** performed.
    pub unsafe fn draw_line_unchecked(
        &self,
        (x0, y0): (usize, usize),
        (x1, y1): (usize, usize),
        color: Color,
    ) {
        let dx = x1.abs_diff(x0);
        let dy = y1.abs_diff(y0);
        match usize::cmp(&dy, &dx) {
            Ordering::Less => {
                if x0 > x1 {
                    unsafe { self.draw_line_low_unchecked(x1, y1, x0, y0, color) }
                } else {
                    unsafe { self.draw_line_low_unchecked(x0, y0, x1, y1, color) }
                }
            }
            Ordering::Equal => todo!(),
            Ordering::Greater => {
                if y0 > y1 {
                    unsafe { self.draw_line_high_unchecked(x1, y1, x0, y0, color) }
                } else {
                    unsafe { self.draw_line_high_unchecked(x0, y0, x1, y1, color) }
                }
            }
        }
    }

    /// Fills an entire rectangle, only bounds checking start + size.
    pub fn fill_rect(
        &self,
        start: (usize, usize),
        size: (usize, usize),
        color: Color,
    ) -> DrawResult {
        let (x0, y0) = start;
        let (x1, y1) = (x0 + size.0, y0 + size.1);
        if x1 > self.width || y1 > self.height {
            Err(DrawError::OutOfBounds)
        } else {
            unsafe {
                self.fill_rect_unchecked(x0, y0, x1, y1, color);
            }
            Ok(())
        }
    }

    /// Fills a rectangle. No bounds checking is performed.
    // I am quite confident this can be optimized more, let me know.
    pub unsafe fn fill_rect_unchecked(
        &self,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        color: Color,
    ) {
        let slice = unsafe {
            slice::from_raw_parts_mut(self.fb.address().cast::<u32>(), self.width * self.height)
        };
        let mut offset = y0 * self.width + x0;
        for y in y0..y1 {
            for x in x0..x1 {
                slice[offset + x] = u32::from_be_bytes(color.into());
            }

            offset += self.width;
        }
    }

    /// Implementation detail of the Bresenham algorithm
    unsafe fn draw_line_low_unchecked(
        &self,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        color: Color,
    ) {
        let dx = x1.cast_signed() - x0.cast_signed();
        let mut dy = y1.cast_signed() - y0.cast_signed();
        let yi: isize = if dy < 0 {
            dy = -dy;
            -1
        } else {
            1
        };
        let mut d = 2 * dy - dx;
        let mut y = y0.cast_signed();
        for x in x0..x1 {
            unsafe { self.write_px_unchecked(x, y.cast_unsigned(), color.into()) };
            if d > 0 {
                y += yi;
                d += 2 * (dy - dx);
            } else {
                d += 2 * dy;
            }
        }
    }

    /// Implementation detail of the Bresenham algorithm
    unsafe fn draw_line_high_unchecked(
        &self,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        color: Color,
    ) {
        let mut dx = x1.cast_signed() - x0.cast_signed();
        let dy = y1.cast_signed() - y0.cast_signed();
        let xi: isize = if dy < 0 {
            dx = -dx;
            -1
        } else {
            1
        };
        let mut d = 2 * dx - dy;
        let mut x = x0.cast_signed();
        for y in y0..y1 {
            unsafe { self.write_px_unchecked(x.cast_unsigned(), y, color.into()) };
            if d > 0 {
                x += xi;
                d += 2 * (dx - dy);
            } else {
                d += 2 * dx;
            }
        }
    }
}
