#[cfg(feature = "calculator-build")]
use crate::prelude::*;
use crate::ui::rendering::{create_canvas, overlay, Canvas, IntoWidget, Widget};

pub struct SizedBox {
    pub child: Box<dyn Widget>,
    pub forced_width: Option<u16>,
    pub forced_height: Option<u16>,
}

impl SizedBox {
    pub fn new<T: IntoWidget>(child: T) -> Self {
        Self {
            child: child.into_widget(),
            forced_width: None,
            forced_height: None,
        }
    }

    /// Forces the widget to have the given width.
    pub fn width(mut self, w: u16) -> Self {
        self.forced_width = Some(w);
        self
    }

    /// Forces the widget to have the given height.
    pub fn height(mut self, h: u16) -> Self {
        self.forced_height = Some(h);
        self
    }
}

impl Widget for SizedBox {
    fn min_size(&self) -> (u16, u16) {
        let (child_w, child_h) = self.child.min_size();
        let w = self.forced_width.unwrap_or(child_w);
        let h = self.forced_height.unwrap_or(child_h);
        (w, h)
    }
    fn render(&self, width: u16, height: u16) -> Canvas {
        // Create a canvas of the allocated size.
        let mut canvas = create_canvas(width, height);
        // Determine the size to render the child: if forced, use that; otherwise, use the parent's allocation.
        let child_width = self.forced_width.unwrap_or(width);
        let child_height = self.forced_height.unwrap_or(height);
        // Render the child using these dimensions.
        let child_canvas = self.child.render(child_width, child_height);
        // Overlay the child's canvas at the top-left corner.
        overlay(&mut canvas, &child_canvas, 0, 0);
        canvas
    }
}

/// Creates a SizedBox widget that forces its child to occupy exactly the given dimensions.
///
/// # Example
///
/// ```rust
/// use widgets::sizedbox::sizedbox;
/// use widgets::text::text;
///
/// // Force the width to 10 columns; height determined by the child.
/// let box1 = sizedbox(text("Content")).width(10);
///
/// // Force the height to 5 rows; width determined by the child.
/// let box2 = sizedbox(text("Content")).height(5);
///
/// // Force both width and height.
/// let box3 = sizedbox(text("Content")).width(10).height(5);
/// ```
pub fn sizedbox<T: IntoWidget>(child: T) -> SizedBox {
    SizedBox::new(child)
}
