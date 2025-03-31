use crate::bindings::nspireio::{
    NIO_COLOR_LIGHTMAGENTA, NIO_COLOR_LIGHTYELLOW, NIO_COLOR_MAGENTA, NIO_COLOR_YELLOW,
};

use super::widgets::Corners;

/// Rounded corners
///
/// .----------------.
/// |                |
/// |                |
/// |                |
/// `----------------'
pub const CORNERS_ROUND: Corners = Corners {
    top_left: Some('.'),
    top_right: Some('.'),
    bottom_left: Some('`'),
    bottom_right: Some('\''),
};

/// Empty corners
///
///  ----------------
/// |                |
/// |                |
/// |                |
///  ----------------
pub const CORNERS_NONE: Corners = Corners {
    top_left: None,
    top_right: None,
    bottom_left: None,
    bottom_right: None,
};

// Colors
pub const COLOR_WHITE: u8 = 0x0F;
pub const COLOR_MAGENTA: u8 = NIO_COLOR_MAGENTA as u8;
pub const COLOR_LIGHTMAGENTA: u8 = NIO_COLOR_LIGHTMAGENTA as u8;
pub const COLOR_YELLOW: u8 = NIO_COLOR_YELLOW as u8;
pub const COLOR_LIGHTYELLOW: u8 = NIO_COLOR_LIGHTYELLOW as u8;
