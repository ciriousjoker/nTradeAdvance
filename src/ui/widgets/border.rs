#[cfg(feature = "calculator-build")]
use crate::prelude::*;
use crate::ui::rendering::{create_canvas, overlay, Canvas, IntoWidget, Widget};

/// Represents the characters used for each side of a border.
/// If an Option is None, that side is not drawn.
#[derive(Clone, Copy)]
pub struct Borders {
    /// Character for the left border.
    pub left: Option<char>,
    /// Character for the right border.
    pub right: Option<char>,
    /// Character for the top border.
    pub top: Option<char>,
    /// Character for the bottom border.
    pub bottom: Option<char>,
}

/// Represents the characters used for the corners of a border.
/// If an Option is None, that corner is not drawn.
#[derive(Clone, Copy)]
pub struct Corners {
    /// Character for the top‐left corner.
    pub top_left: Option<char>,
    /// Character for the top‐right corner.
    pub top_right: Option<char>,
    /// Character for the bottom‐left corner.
    pub bottom_left: Option<char>,
    /// Character for the bottom‐right corner.
    pub bottom_right: Option<char>,
}

pub struct Border {
    pub child: Box<dyn Widget>,
    pub borders: Borders,
    pub corners: Corners,
}

impl Border {
    pub fn new<T: IntoWidget>(child: T) -> Self {
        Self {
            child: child.into_widget(),
            borders: Borders {
                left: Some('|'),
                right: Some('|'),
                top: Some('-'),
                bottom: Some('-'),
            },
            corners: Corners {
                top_left: Some('+'),
                top_right: Some('+'),
                bottom_left: Some('+'),
                bottom_right: Some('+'),
            },
        }
    }

    /// Sets the border characters.
    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = borders;
        self
    }

    /// Sets the corner characters.
    pub fn corners(mut self, corners: Corners) -> Self {
        self.corners = corners;
        self
    }
}

impl Widget for Border {
    fn min_size(&self) -> (u16, u16) {
        let (child_w, child_h) = self.child.min_size();
        let extra_w = if self.borders.left.is_some() { 1 } else { 0 }
            + if self.borders.right.is_some() { 1 } else { 0 };
        let extra_h = if self.borders.top.is_some() { 1 } else { 0 }
            + if self.borders.bottom.is_some() { 1 } else { 0 };
        (child_w + extra_w, child_h + extra_h)
    }
    fn render(&self, width: u16, height: u16) -> Canvas {
        // Ensure the canvas is at least as big as min_size.
        let (min_w, min_h) = self.min_size();
        let width = if width < min_w { min_w } else { width };
        let height = if height < min_h { min_h } else { height };

        let mut canvas = create_canvas(width, height);

        // Draw horizontal borders.
        if let Some(top_char) = self.borders.top {
            for x in 0..width as usize {
                canvas[0][x] = top_char;
            }
        }
        if let Some(bottom_char) = self.borders.bottom {
            for x in 0..width as usize {
                canvas[(height - 1) as usize][x] = bottom_char;
            }
        }

        // Draw vertical borders.
        if let Some(left_char) = self.borders.left {
            for y in 0..height as usize {
                canvas[y][0] = left_char;
            }
        }
        if let Some(right_char) = self.borders.right {
            for y in 0..height as usize {
                canvas[y][(width - 1) as usize] = right_char;
            }
        }

        // Draw corners if corresponding borders exist.
        if self.borders.top.is_some() && self.borders.left.is_some() {
            canvas[0][0] = match self.corners.top_left {
                Some(c) => c,
                None => ' ',
            };
        }
        if self.borders.top.is_some() && self.borders.right.is_some() {
            canvas[0][(width - 1) as usize] = match self.corners.top_right {
                Some(c) => c,
                None => ' ',
            };
        }
        if self.borders.bottom.is_some() && self.borders.left.is_some() {
            canvas[(height - 1) as usize][0] = match self.corners.bottom_left {
                Some(c) => c,
                None => ' ',
            };
        }
        if self.borders.bottom.is_some() && self.borders.right.is_some() {
            canvas[(height - 1) as usize][(width - 1) as usize] = match self.corners.bottom_right {
                Some(c) => c,
                None => ' ',
            };
        }

        // Compute inner region for the child.
        let x_offset = if self.borders.left.is_some() { 1 } else { 0 };
        let y_offset = if self.borders.top.is_some() { 1 } else { 0 };
        let inner_width = width - x_offset - if self.borders.right.is_some() { 1 } else { 0 };
        let inner_height = height - y_offset - if self.borders.bottom.is_some() { 1 } else { 0 };

        let child_canvas = self.child.render(inner_width, inner_height);
        overlay(&mut canvas, &child_canvas, x_offset, y_offset);

        canvas
    }
}

/// Creates a Border widget that wraps a child widget with a border and optional corners.
///
/// # Example
///
/// ```rust
/// use widgets::border::{border, Borders, Corners};
/// use widgets::text::text;
///
/// let bordered = border(text("Hello"))
///     .borders(Borders {
///         left: None,
///         right: None,
///         top: Some('-'),
///         bottom: Some('-'),
///     })
///     .corners(Corners {
///         top_left: None,
///         top_right: None,
///         bottom_left: None,
///         bottom_right: None,
///     });
/// ```
pub fn border<T: IntoWidget>(child: T) -> Border {
    Border::new(child)
}
