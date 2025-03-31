#[cfg(feature = "calculator-build")]
use crate::prelude::*;
use crate::ui::rendering::{create_canvas, overlay, Canvas, IntoWidget, Widget};

pub struct Stack {
    pub children: Vec<Box<dyn Widget>>,
}

impl Stack {
    pub fn new<T: IntoWidget>(children: Vec<T>) -> Self {
        Self {
            children: children.into_iter().map(|c| c.into_widget()).collect(),
        }
    }
}

impl Widget for Stack {
    fn min_size(&self) -> (u16, u16) {
        let mut max_w = 0;
        let mut max_h = 0;
        for child in &self.children {
            let (w, h) = child.min_size();
            if w > max_w {
                max_w = w;
            }
            if h > max_h {
                max_h = h;
            }
        }
        (max_w, max_h)
    }
    fn render(&self, width: u16, height: u16) -> Canvas {
        let mut canvas = create_canvas(width, height);
        for child in &self.children {
            let child_canvas = child.render(width, height);
            overlay(&mut canvas, &child_canvas, 0, 0);
        }
        canvas
    }
}

/// Creates a Stack widget that overlays its children.
///
/// # Example
///
/// ```rust
/// use widgets::stack::stack;
/// use widgets::text::text;
///
/// let s = stack(vec![
///     text("Background"),
///     text("Foreground"),
/// ]);
/// ```
pub fn stack<T: IntoWidget>(children: Vec<T>) -> Stack {
    Stack::new(children)
}
