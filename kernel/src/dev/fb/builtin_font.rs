use crate::debug;

use super::{FbFont, FbGlyph, FbGlyphDimensions};

pub struct BuiltinFont;

impl BuiltinFont {
  #[inline]
  pub const fn new() -> Self {
    Self
  }
}

impl FbFont for BuiltinFont {
  #[inline]
  fn dimensions(&self) -> super::FbGlyphDimensions {
    FbGlyphDimensions::new(BUILTIN_FONT_WIDTH, BUILTIN_FONT_HEIGHT)
  }
  #[inline]
  fn glyph<'a>(&'a self, c: char) -> super::FbGlyph<'a> {
    let start = if (c as usize) < 256 { c as usize } else { 0 } * BUILTIN_FONT_WIDTH * BUILTIN_FONT_HEIGHT / 8;
    FbGlyph::from(&BUILTIN_FONT[start..start + 16], BUILTIN_FONT_HEIGHT)
  }
}

const BUILTIN_FONT_WIDTH: usize = 8;
const BUILTIN_FONT_HEIGHT: usize = 16;
// Tamsyn Font 8x16r
const BUILTIN_FONT: [u8; 256 * 16] = [
  0x00, 0x00, 0x00, 0x00, 0x7e, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7e, 0x00,
  0x00, 0x00, 0x00, /* 0 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 1 */
  0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55,
  0xaa, 0x55, 0xaa, /* 2 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 3 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 4 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 5 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 6 */
  0x00, 0x00, 0x00, 0x38, 0x44, 0x44, 0x44, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 7 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 8 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 9 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 10 */
  0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 11 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0x08, 0x08, 0x08, 0x08, 0x08,
  0x08, 0x08, 0x08, /* 12 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x08, 0x08, 0x08, 0x08, 0x08,
  0x08, 0x08, 0x08, /* 13 */
  0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 14 */
  0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0xff, 0x08, 0x08, 0x08, 0x08, 0x08,
  0x08, 0x08, 0x08, /* 15 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 16 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 17 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 18 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 19 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 20 */
  0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x0f, 0x08, 0x08, 0x08, 0x08, 0x08,
  0x08, 0x08, 0x08, /* 21 */
  0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0xf8, 0x08, 0x08, 0x08, 0x08, 0x08,
  0x08, 0x08, 0x08, /* 22 */
  0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 23 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x08, 0x08, 0x08, 0x08, 0x08,
  0x08, 0x08, 0x08, /* 24 */
  0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08,
  0x08, 0x08, 0x08, /* 25 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 26 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 27 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 28 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 29 */
  0x00, 0x00, 0x00, 0x1c, 0x22, 0x20, 0x20, 0xf8, 0x20, 0x20, 0x72, 0x8c, 0x00,
  0x00, 0x00, 0x00, /* 30 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 31 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 32 */
  0x00, 0x00, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x00, 0x00, 0x10, 0x10, 0x00,
  0x00, 0x00, 0x00, /* 33 */
  0x00, 0x24, 0x24, 0x24, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 34 */
  0x00, 0x00, 0x00, 0x24, 0x24, 0x7e, 0x24, 0x24, 0x24, 0x7e, 0x24, 0x24, 0x00,
  0x00, 0x00, 0x00, /* 35 */
  0x00, 0x00, 0x08, 0x08, 0x1e, 0x20, 0x20, 0x1c, 0x02, 0x02, 0x3c, 0x08, 0x08,
  0x00, 0x00, 0x00, /* 36 */
  0x00, 0x00, 0x00, 0x30, 0x49, 0x4a, 0x34, 0x08, 0x16, 0x29, 0x49, 0x06, 0x00,
  0x00, 0x00, 0x00, /* 37 */
  0x00, 0x00, 0x30, 0x48, 0x48, 0x48, 0x30, 0x31, 0x49, 0x46, 0x46, 0x39, 0x00,
  0x00, 0x00, 0x00, /* 38 */
  0x00, 0x10, 0x10, 0x10, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 39 */
  0x00, 0x00, 0x04, 0x08, 0x08, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x08, 0x08,
  0x04, 0x00, 0x00, /* 40 */
  0x00, 0x00, 0x20, 0x10, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x10,
  0x20, 0x00, 0x00, /* 41 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x18, 0x7e, 0x18, 0x24, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 42 */
  0x00, 0x00, 0x00, 0x00, 0x08, 0x08, 0x08, 0x7f, 0x08, 0x08, 0x08, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 43 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x08,
  0x08, 0x10, 0x00, /* 44 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7e, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 45 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00,
  0x00, 0x00, 0x00, /* 46 */
  0x00, 0x00, 0x02, 0x02, 0x04, 0x04, 0x08, 0x08, 0x10, 0x10, 0x20, 0x20, 0x40,
  0x40, 0x00, 0x00, /* 47 */
  0x00, 0x00, 0x00, 0x3c, 0x42, 0x46, 0x4a, 0x52, 0x62, 0x42, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 48 */
  0x00, 0x00, 0x00, 0x08, 0x18, 0x28, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 49 */
  0x00, 0x00, 0x00, 0x3c, 0x42, 0x02, 0x02, 0x04, 0x08, 0x10, 0x20, 0x7e, 0x00,
  0x00, 0x00, 0x00, /* 50 */
  0x00, 0x00, 0x00, 0x7e, 0x04, 0x08, 0x1c, 0x02, 0x02, 0x02, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 51 */
  0x00, 0x00, 0x00, 0x04, 0x0c, 0x14, 0x24, 0x44, 0x7e, 0x04, 0x04, 0x04, 0x00,
  0x00, 0x00, 0x00, /* 52 */
  0x00, 0x00, 0x00, 0x7e, 0x40, 0x40, 0x7c, 0x02, 0x02, 0x02, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 53 */
  0x00, 0x00, 0x00, 0x1c, 0x20, 0x40, 0x40, 0x7c, 0x42, 0x42, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 54 */
  0x00, 0x00, 0x00, 0x7e, 0x02, 0x04, 0x04, 0x08, 0x08, 0x10, 0x10, 0x10, 0x00,
  0x00, 0x00, 0x00, /* 55 */
  0x00, 0x00, 0x00, 0x3c, 0x42, 0x42, 0x42, 0x3c, 0x42, 0x42, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 56 */
  0x00, 0x00, 0x00, 0x3c, 0x42, 0x42, 0x42, 0x3e, 0x02, 0x02, 0x04, 0x38, 0x00,
  0x00, 0x00, 0x00, /* 57 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00,
  0x00, 0x00, 0x00, /* 58 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00, 0x18, 0x18, 0x08,
  0x08, 0x10, 0x00, /* 59 */
  0x00, 0x00, 0x00, 0x04, 0x08, 0x10, 0x20, 0x40, 0x20, 0x10, 0x08, 0x04, 0x00,
  0x00, 0x00, 0x00, /* 60 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7e, 0x00, 0x00, 0x7e, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 61 */
  0x00, 0x00, 0x00, 0x20, 0x10, 0x08, 0x04, 0x02, 0x04, 0x08, 0x10, 0x20, 0x00,
  0x00, 0x00, 0x00, /* 62 */
  0x00, 0x00, 0x3c, 0x42, 0x02, 0x04, 0x08, 0x10, 0x00, 0x00, 0x10, 0x10, 0x00,
  0x00, 0x00, 0x00, /* 63 */
  0x00, 0x00, 0x1c, 0x22, 0x41, 0x4f, 0x51, 0x51, 0x51, 0x53, 0x4d, 0x40, 0x20,
  0x1f, 0x00, 0x00, /* 64 */
  0x00, 0x00, 0x00, 0x18, 0x24, 0x42, 0x42, 0x42, 0x7e, 0x42, 0x42, 0x42, 0x00,
  0x00, 0x00, 0x00, /* 65 */
  0x00, 0x00, 0x00, 0x7c, 0x42, 0x42, 0x42, 0x7c, 0x42, 0x42, 0x42, 0x7c, 0x00,
  0x00, 0x00, 0x00, /* 66 */
  0x00, 0x00, 0x00, 0x1e, 0x20, 0x40, 0x40, 0x40, 0x40, 0x40, 0x20, 0x1e, 0x00,
  0x00, 0x00, 0x00, /* 67 */
  0x00, 0x00, 0x00, 0x78, 0x44, 0x42, 0x42, 0x42, 0x42, 0x42, 0x44, 0x78, 0x00,
  0x00, 0x00, 0x00, /* 68 */
  0x00, 0x00, 0x00, 0x7e, 0x40, 0x40, 0x40, 0x7c, 0x40, 0x40, 0x40, 0x7e, 0x00,
  0x00, 0x00, 0x00, /* 69 */
  0x00, 0x00, 0x00, 0x7e, 0x40, 0x40, 0x40, 0x7c, 0x40, 0x40, 0x40, 0x40, 0x00,
  0x00, 0x00, 0x00, /* 70 */
  0x00, 0x00, 0x00, 0x1e, 0x20, 0x40, 0x40, 0x46, 0x42, 0x42, 0x22, 0x1e, 0x00,
  0x00, 0x00, 0x00, /* 71 */
  0x00, 0x00, 0x00, 0x42, 0x42, 0x42, 0x42, 0x7e, 0x42, 0x42, 0x42, 0x42, 0x00,
  0x00, 0x00, 0x00, /* 72 */
  0x00, 0x00, 0x00, 0x3e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 73 */
  0x00, 0x00, 0x00, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x42, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 74 */
  0x00, 0x00, 0x00, 0x42, 0x44, 0x48, 0x50, 0x60, 0x50, 0x48, 0x44, 0x42, 0x00,
  0x00, 0x00, 0x00, /* 75 */
  0x00, 0x00, 0x00, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x7e, 0x00,
  0x00, 0x00, 0x00, /* 76 */
  0x00, 0x00, 0x00, 0x41, 0x63, 0x55, 0x49, 0x49, 0x41, 0x41, 0x41, 0x41, 0x00,
  0x00, 0x00, 0x00, /* 77 */
  0x00, 0x00, 0x00, 0x42, 0x62, 0x52, 0x4a, 0x46, 0x42, 0x42, 0x42, 0x42, 0x00,
  0x00, 0x00, 0x00, /* 78 */
  0x00, 0x00, 0x00, 0x3c, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 79 */
  0x00, 0x00, 0x00, 0x7c, 0x42, 0x42, 0x42, 0x7c, 0x40, 0x40, 0x40, 0x40, 0x00,
  0x00, 0x00, 0x00, /* 80 */
  0x00, 0x00, 0x00, 0x3c, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3c, 0x04,
  0x02, 0x00, 0x00, /* 81 */
  0x00, 0x00, 0x00, 0x7c, 0x42, 0x42, 0x42, 0x7c, 0x48, 0x44, 0x42, 0x42, 0x00,
  0x00, 0x00, 0x00, /* 82 */
  0x00, 0x00, 0x00, 0x3e, 0x40, 0x40, 0x20, 0x18, 0x04, 0x02, 0x02, 0x7c, 0x00,
  0x00, 0x00, 0x00, /* 83 */
  0x00, 0x00, 0x00, 0x7f, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00,
  0x00, 0x00, 0x00, /* 84 */
  0x00, 0x00, 0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 85 */
  0x00, 0x00, 0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x24, 0x24, 0x18, 0x18, 0x00,
  0x00, 0x00, 0x00, /* 86 */
  0x00, 0x00, 0x00, 0x41, 0x41, 0x41, 0x41, 0x49, 0x49, 0x49, 0x55, 0x63, 0x00,
  0x00, 0x00, 0x00, /* 87 */
  0x00, 0x00, 0x00, 0x41, 0x41, 0x22, 0x14, 0x08, 0x14, 0x22, 0x41, 0x41, 0x00,
  0x00, 0x00, 0x00, /* 88 */
  0x00, 0x00, 0x00, 0x41, 0x41, 0x22, 0x14, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00,
  0x00, 0x00, 0x00, /* 89 */
  0x00, 0x00, 0x00, 0x7e, 0x04, 0x08, 0x08, 0x10, 0x10, 0x20, 0x20, 0x7e, 0x00,
  0x00, 0x00, 0x00, /* 90 */
  0x00, 0x00, 0x1e, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
  0x1e, 0x00, 0x00, /* 91 */
  0x00, 0x00, 0x40, 0x40, 0x20, 0x20, 0x10, 0x10, 0x08, 0x08, 0x04, 0x04, 0x02,
  0x02, 0x00, 0x00, /* 92 */
  0x00, 0x00, 0x78, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08,
  0x78, 0x00, 0x00, /* 93 */
  0x00, 0x00, 0x10, 0x28, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 94 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0xff, 0x00, 0x00, /* 95 */
  0x00, 0x20, 0x10, 0x08, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 96 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x02, 0x02, 0x3e, 0x42, 0x42, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 97 */
  0x00, 0x00, 0x40, 0x40, 0x40, 0x7c, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7c, 0x00,
  0x00, 0x00, 0x00, /* 98 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x42, 0x40, 0x40, 0x40, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 99 */
  0x00, 0x00, 0x02, 0x02, 0x02, 0x3e, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 100 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x42, 0x42, 0x7e, 0x40, 0x40, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 101 */
  0x00, 0x00, 0x0e, 0x10, 0x10, 0x7e, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x00,
  0x00, 0x00, 0x00, /* 102 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3e, 0x02,
  0x02, 0x3c, 0x00, /* 103 */
  0x00, 0x00, 0x40, 0x40, 0x40, 0x7c, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x00,
  0x00, 0x00, 0x00, /* 104 */
  0x00, 0x00, 0x08, 0x08, 0x00, 0x38, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 105 */
  0x00, 0x00, 0x04, 0x04, 0x00, 0x1c, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
  0x04, 0x38, 0x00, /* 106 */
  0x00, 0x00, 0x40, 0x40, 0x40, 0x44, 0x48, 0x50, 0x70, 0x48, 0x44, 0x42, 0x00,
  0x00, 0x00, 0x00, /* 107 */
  0x00, 0x00, 0x38, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 108 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0x49, 0x49, 0x49, 0x49, 0x49, 0x49, 0x00,
  0x00, 0x00, 0x00, /* 109 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x7c, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x00,
  0x00, 0x00, 0x00, /* 110 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 111 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x7c, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7c, 0x40,
  0x40, 0x40, 0x00, /* 112 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3e, 0x02,
  0x02, 0x02, 0x00, /* 113 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x2e, 0x30, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00,
  0x00, 0x00, 0x00, /* 114 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x40, 0x20, 0x18, 0x04, 0x02, 0x7c, 0x00,
  0x00, 0x00, 0x00, /* 115 */
  0x00, 0x00, 0x00, 0x10, 0x10, 0x7e, 0x10, 0x10, 0x10, 0x10, 0x10, 0x0e, 0x00,
  0x00, 0x00, 0x00, /* 116 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 117 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x42, 0x42, 0x24, 0x24, 0x18, 0x18, 0x00,
  0x00, 0x00, 0x00, /* 118 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x41, 0x41, 0x49, 0x49, 0x55, 0x63, 0x00,
  0x00, 0x00, 0x00, /* 119 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x22, 0x14, 0x08, 0x14, 0x22, 0x41, 0x00,
  0x00, 0x00, 0x00, /* 120 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3e, 0x02,
  0x02, 0x3c, 0x00, /* 121 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x7e, 0x04, 0x08, 0x10, 0x20, 0x40, 0x7e, 0x00,
  0x00, 0x00, 0x00, /* 122 */
  0x00, 0x0e, 0x10, 0x10, 0x10, 0x10, 0x10, 0xe0, 0x10, 0x10, 0x10, 0x10, 0x10,
  0x0e, 0x00, 0x00, /* 123 */
  0x00, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08,
  0x08, 0x00, 0x00, /* 124 */
  0x00, 0x70, 0x08, 0x08, 0x08, 0x08, 0x08, 0x07, 0x08, 0x08, 0x08, 0x08, 0x08,
  0x70, 0x00, 0x00, /* 125 */
  0x00, 0x00, 0x31, 0x49, 0x46, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 126 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 127 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 128 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 129 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 130 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 131 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 132 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 133 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 134 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 135 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 136 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 137 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 138 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 139 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 140 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 141 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 142 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 143 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 144 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 145 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 146 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 147 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 148 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 149 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 150 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 151 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 152 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 153 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 154 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 155 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 156 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 157 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 158 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 159 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 160 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x10, 0x00, 0x00, 0x10, 0x10, 0x10, 0x10,
  0x10, 0x10, 0x00, /* 161 */
  0x00, 0x00, 0x08, 0x08, 0x1c, 0x22, 0x40, 0x40, 0x40, 0x22, 0x1c, 0x08, 0x08,
  0x00, 0x00, 0x00, /* 162 */
  0x00, 0x00, 0x00, 0x1c, 0x22, 0x20, 0x20, 0xf8, 0x20, 0x20, 0x72, 0x8c, 0x00,
  0x00, 0x00, 0x00, /* 163 */
  0x00, 0x00, 0x00, 0x00, 0x42, 0x3c, 0x24, 0x24, 0x24, 0x3c, 0x42, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 164 */
  0x00, 0x00, 0x00, 0x41, 0x22, 0x14, 0x08, 0x3e, 0x08, 0x3e, 0x08, 0x08, 0x00,
  0x00, 0x00, 0x00, /* 165 */
  0x00, 0x00, 0x00, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x08, 0x08, 0x08,
  0x08, 0x00, 0x00, /* 166 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 167 */
  0x22, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 168 */
  0x00, 0x00, 0x00, 0x1c, 0x22, 0x41, 0x4d, 0x51, 0x51, 0x4d, 0x41, 0x22, 0x1c,
  0x00, 0x00, 0x00, /* 169 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 170 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x12, 0x24, 0x48, 0x24, 0x12, 0x09, 0x00,
  0x00, 0x00, 0x00, /* 171 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 172 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7e, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 173 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 174 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 175 */
  0x00, 0x00, 0x00, 0x38, 0x44, 0x44, 0x44, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 176 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 177 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 178 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 179 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 180 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 181 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 182 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 183 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08,
  0x08, 0x30, 0x00, /* 184 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 185 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 186 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x24, 0x12, 0x09, 0x12, 0x24, 0x48, 0x00,
  0x00, 0x00, 0x00, /* 187 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 188 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 189 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 190 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x08, 0x00, 0x00, 0x08, 0x10, 0x20, 0x40,
  0x42, 0x3c, 0x00, /* 191 */
  0x20, 0x10, 0x00, 0x18, 0x18, 0x24, 0x24, 0x24, 0x7e, 0x42, 0x42, 0x42, 0x00,
  0x00, 0x00, 0x00, /* 192 */
  0x04, 0x08, 0x00, 0x18, 0x18, 0x24, 0x24, 0x24, 0x7e, 0x42, 0x42, 0x42, 0x00,
  0x00, 0x00, 0x00, /* 193 */
  0x18, 0x24, 0x00, 0x18, 0x18, 0x24, 0x24, 0x24, 0x7e, 0x42, 0x42, 0x42, 0x00,
  0x00, 0x00, 0x00, /* 194 */
  0x32, 0x4c, 0x00, 0x18, 0x18, 0x24, 0x24, 0x24, 0x7e, 0x42, 0x42, 0x42, 0x00,
  0x00, 0x00, 0x00, /* 195 */
  0x24, 0x24, 0x00, 0x18, 0x18, 0x24, 0x24, 0x24, 0x7e, 0x42, 0x42, 0x42, 0x00,
  0x00, 0x00, 0x00, /* 196 */
  0x18, 0x24, 0x24, 0x18, 0x18, 0x24, 0x24, 0x24, 0x7e, 0x42, 0x42, 0x42, 0x00,
  0x00, 0x00, 0x00, /* 197 */
  0x00, 0x00, 0x00, 0x0f, 0x14, 0x14, 0x24, 0x27, 0x3c, 0x44, 0x44, 0x47, 0x00,
  0x00, 0x00, 0x00, /* 198 */
  0x00, 0x00, 0x00, 0x1e, 0x20, 0x40, 0x40, 0x40, 0x40, 0x40, 0x20, 0x1e, 0x08,
  0x08, 0x30, 0x00, /* 199 */
  0x20, 0x10, 0x00, 0x7e, 0x40, 0x40, 0x40, 0x7c, 0x40, 0x40, 0x40, 0x7e, 0x00,
  0x00, 0x00, 0x00, /* 200 */
  0x04, 0x08, 0x00, 0x7e, 0x40, 0x40, 0x40, 0x7c, 0x40, 0x40, 0x40, 0x7e, 0x00,
  0x00, 0x00, 0x00, /* 201 */
  0x18, 0x24, 0x00, 0x7e, 0x40, 0x40, 0x40, 0x7c, 0x40, 0x40, 0x40, 0x7e, 0x00,
  0x00, 0x00, 0x00, /* 202 */
  0x24, 0x24, 0x00, 0x7e, 0x40, 0x40, 0x40, 0x7c, 0x40, 0x40, 0x40, 0x7e, 0x00,
  0x00, 0x00, 0x00, /* 203 */
  0x10, 0x08, 0x00, 0x3e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 204 */
  0x04, 0x08, 0x00, 0x3e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 205 */
  0x18, 0x24, 0x00, 0x3e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 206 */
  0x22, 0x22, 0x00, 0x3e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 207 */
  0x00, 0x00, 0x00, 0x3c, 0x22, 0x21, 0x21, 0x79, 0x21, 0x21, 0x22, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 208 */
  0x32, 0x4c, 0x00, 0x42, 0x62, 0x52, 0x4a, 0x46, 0x42, 0x42, 0x42, 0x42, 0x00,
  0x00, 0x00, 0x00, /* 209 */
  0x10, 0x08, 0x00, 0x1c, 0x22, 0x41, 0x41, 0x41, 0x41, 0x41, 0x22, 0x1c, 0x00,
  0x00, 0x00, 0x00, /* 210 */
  0x04, 0x08, 0x00, 0x1c, 0x22, 0x41, 0x41, 0x41, 0x41, 0x41, 0x22, 0x1c, 0x00,
  0x00, 0x00, 0x00, /* 211 */
  0x18, 0x24, 0x00, 0x1c, 0x22, 0x41, 0x41, 0x41, 0x41, 0x41, 0x22, 0x1c, 0x00,
  0x00, 0x00, 0x00, /* 212 */
  0x32, 0x4c, 0x00, 0x1c, 0x22, 0x41, 0x41, 0x41, 0x41, 0x41, 0x22, 0x1c, 0x00,
  0x00, 0x00, 0x00, /* 213 */
  0x24, 0x24, 0x00, 0x1c, 0x22, 0x41, 0x41, 0x41, 0x41, 0x41, 0x22, 0x1c, 0x00,
  0x00, 0x00, 0x00, /* 214 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x22, 0x14, 0x08, 0x14, 0x22, 0x00, 0x00,
  0x00, 0x00, 0x00, /* 215 */
  0x00, 0x00, 0x02, 0x3c, 0x42, 0x46, 0x4a, 0x52, 0x62, 0x42, 0x42, 0x3c, 0x40,
  0x00, 0x00, 0x00, /* 216 */
  0x20, 0x10, 0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 217 */
  0x04, 0x08, 0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 218 */
  0x18, 0x24, 0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 219 */
  0x24, 0x24, 0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 220 */
  0x04, 0x08, 0x00, 0x41, 0x41, 0x22, 0x14, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00,
  0x00, 0x00, 0x00, /* 221 */
  0x00, 0x00, 0x00, 0x40, 0x40, 0x7c, 0x42, 0x42, 0x42, 0x7c, 0x40, 0x40, 0x00,
  0x00, 0x00, 0x00, /* 222 */
  0x00, 0x00, 0x00, 0x3c, 0x42, 0x44, 0x4c, 0x42, 0x42, 0x42, 0x44, 0x58, 0x00,
  0x00, 0x00, 0x00, /* 223 */
  0x00, 0x00, 0x20, 0x10, 0x00, 0x3c, 0x02, 0x02, 0x3e, 0x42, 0x42, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 224 */
  0x00, 0x00, 0x04, 0x08, 0x00, 0x3c, 0x02, 0x02, 0x3e, 0x42, 0x42, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 225 */
  0x00, 0x18, 0x24, 0x00, 0x00, 0x3c, 0x02, 0x02, 0x3e, 0x42, 0x42, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 226 */
  0x00, 0x32, 0x4c, 0x00, 0x00, 0x3c, 0x02, 0x02, 0x3e, 0x42, 0x42, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 227 */
  0x00, 0x00, 0x24, 0x24, 0x00, 0x3c, 0x02, 0x02, 0x3e, 0x42, 0x42, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 228 */
  0x18, 0x24, 0x24, 0x18, 0x00, 0x3c, 0x02, 0x02, 0x3e, 0x42, 0x42, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 229 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x09, 0x39, 0x4f, 0x48, 0x48, 0x37, 0x00,
  0x00, 0x00, 0x00, /* 230 */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x42, 0x40, 0x40, 0x40, 0x42, 0x3c, 0x08,
  0x08, 0x30, 0x00, /* 231 */
  0x00, 0x00, 0x20, 0x10, 0x00, 0x3c, 0x42, 0x42, 0x7e, 0x40, 0x40, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 232 */
  0x00, 0x00, 0x04, 0x08, 0x00, 0x3c, 0x42, 0x42, 0x7e, 0x40, 0x40, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 233 */
  0x00, 0x18, 0x24, 0x00, 0x00, 0x3c, 0x42, 0x42, 0x7e, 0x40, 0x40, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 234 */
  0x00, 0x00, 0x24, 0x24, 0x00, 0x3c, 0x42, 0x42, 0x7e, 0x40, 0x40, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 235 */
  0x00, 0x00, 0x10, 0x08, 0x00, 0x38, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 236 */
  0x00, 0x00, 0x04, 0x08, 0x00, 0x38, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 237 */
  0x00, 0x18, 0x24, 0x00, 0x00, 0x38, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 238 */
  0x00, 0x00, 0x24, 0x24, 0x00, 0x38, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 239 */
  0x00, 0x09, 0x06, 0x1a, 0x01, 0x1d, 0x23, 0x41, 0x41, 0x41, 0x22, 0x1c, 0x00,
  0x00, 0x00, 0x00, /* 240 */
  0x00, 0x32, 0x4c, 0x00, 0x00, 0x7c, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x00,
  0x00, 0x00, 0x00, /* 241 */
  0x00, 0x00, 0x10, 0x08, 0x00, 0x3c, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 242 */
  0x00, 0x00, 0x04, 0x08, 0x00, 0x3c, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 243 */
  0x00, 0x18, 0x24, 0x00, 0x00, 0x3c, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 244 */
  0x00, 0x32, 0x4c, 0x00, 0x00, 0x3c, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 245 */
  0x00, 0x00, 0x24, 0x24, 0x00, 0x3c, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3c, 0x00,
  0x00, 0x00, 0x00, /* 246 */
  0x00, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00, 0x7e, 0x00, 0x00, 0x18, 0x18, 0x00,
  0x00, 0x00, 0x00, /* 247 */
  0x00, 0x00, 0x00, 0x00, 0x02, 0x3c, 0x46, 0x4a, 0x52, 0x62, 0x42, 0x3c, 0x40,
  0x00, 0x00, 0x00, /* 248 */
  0x00, 0x00, 0x20, 0x10, 0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 249 */
  0x00, 0x00, 0x04, 0x08, 0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 250 */
  0x00, 0x18, 0x24, 0x00, 0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 251 */
  0x00, 0x00, 0x24, 0x24, 0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3e, 0x00,
  0x00, 0x00, 0x00, /* 252 */
  0x00, 0x00, 0x04, 0x08, 0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3e, 0x02,
  0x02, 0x3c, 0x00, /* 253 */
  0x00, 0x00, 0x40, 0x40, 0x40, 0x5c, 0x62, 0x41, 0x41, 0x41, 0x62, 0x5c, 0x40,
  0x40, 0x40, 0x00, /* 254 */
  0x00, 0x00, 0x24, 0x24, 0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3e, 0x02,
  0x02, 0x3c, 0x00, /* 255 */
];