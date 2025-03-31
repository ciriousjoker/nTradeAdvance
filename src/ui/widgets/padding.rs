#[cfg(feature = "calculator-build")]
use crate::prelude::*;
use crate::ui::rendering::{create_canvas, overlay, Canvas, IntoWidget, Widget};

pub struct Padding {
    pub child: Box<dyn Widget>,
    pub top: u16,
    pub left: u16,
    pub right: u16,
    pub bottom: u16,
}

impl Padding {
    pub fn new<T: IntoWidget>(child: T) -> Self {
        Self {
            child: child.into_widget(),
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
        }
    }
    /// Sets the top padding.
    #[allow(dead_code)]
    pub fn top(mut self, amount: u16) -> Self {
        self.top = amount;
        self
    }
    /// Sets the left padding.
    pub fn left(mut self, amount: u16) -> Self {
        self.left = amount;
        self
    }
    /// Sets the right padding.
    pub fn right(mut self, amount: u16) -> Self {
        self.right = amount;
        self
    }
    /// Sets the bottom padding.
    pub fn bottom(mut self, amount: u16) -> Self {
        self.bottom = amount;
        self
    }
    /// Sets the padding on all four sides.
    #[allow(dead_code)]
    pub fn all(mut self, amount: u16) -> Self {
        self.top = amount;
        self.left = amount;
        self.right = amount;
        self.bottom = amount;
        self
    }
    /// Sets the horizontal padding (left and right).
    pub fn horizontal(mut self, amount: u16) -> Self {
        self.left = amount;
        self.right = amount;
        self
    }
    /// Sets the vertical padding (top and bottom).
    pub fn vertical(mut self, amount: u16) -> Self {
        self.top = amount;
        self.bottom = amount;
        self
    }
}

impl Widget for Padding {
    fn min_size(&self) -> (u16, u16) {
        let (child_w, child_h) = self.child.min_size();
        (
            child_w + self.left + self.right,
            child_h + self.top + self.bottom,
        )
    }
    fn render(&self, width: u16, height: u16) -> Canvas {
        // Create a canvas with the allocated width and height.
        let mut canvas = create_canvas(width, height);
        // Determine inner region size for the child.
        let inner_width = width.saturating_sub(self.left + self.right);
        let inner_height = height.saturating_sub(self.top + self.bottom);
        // Render the child into the inner area.
        let child_canvas = self.child.render(inner_width, inner_height);
        // Overlay the child's canvas at the position defined by the padding.
        overlay(&mut canvas, &child_canvas, self.left, self.top);
        canvas
    }
}

/// Creates a Padding widget that adds space around the child widget.
///
/// # Example
///
/// ```rust
/// use widgets::padding::{padding};
/// use widgets::text::text;
///
/// // Add 2 rows of padding on top and bottom and 1 column on left and right.
/// let p = padding(text("Padded content")).vertical(2).horizontal(1);
///
/// // Or add uniform padding on all sides:
/// let p2 = padding(text("Padded content")).all(3);
/// ```
pub fn padding<T: IntoWidget>(child: T) -> Padding {
    Padding::new(child)
}
