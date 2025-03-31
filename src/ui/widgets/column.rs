#[cfg(feature = "calculator-build")]
use crate::prelude::*;
use crate::ui::rendering::{create_canvas, overlay, Canvas, IntoWidget, Widget};

pub struct Column {
    pub children: Vec<Box<dyn Widget>>,
}

impl Column {
    pub fn new<T: IntoWidget>(children: Vec<T>) -> Self {
        Self {
            children: children.into_iter().map(|c| c.into_widget()).collect(),
        }
    }
}

impl Widget for Column {
    fn min_size(&self) -> (u16, u16) {
        let mut total_height = 0;
        let mut max_width = 0;
        for child in &self.children {
            let (w, h) = child.min_size();
            total_height += h;
            if w > max_width {
                max_width = w;
            }
        }
        (max_width, total_height)
    }
    fn render(&self, width: u16, height: u16) -> Canvas {
        let mut canvas = create_canvas(width, height);
        let n = self.children.len();
        let mut allocated_heights = vec![0u16; n];
        let mut total_min = 0;
        let mut total_flex = 0;
        for (i, child) in self.children.iter().enumerate() {
            let (_, h) = child.min_size();
            allocated_heights[i] = h;
            total_min += h;
            if let Some(f) = child.flex_factor() {
                total_flex += f;
            }
        }
        let remaining = if height > total_min {
            height - total_min
        } else {
            0
        };
        if total_flex > 0 {
            for (i, child) in self.children.iter().enumerate() {
                if let Some(f) = child.flex_factor() {
                    let extra = (remaining as i32 * f) / total_flex;
                    allocated_heights[i] += extra as u16;
                }
            }
        }
        let mut y_offset = 0;
        for (i, child) in self.children.iter().enumerate() {
            let child_canvas = child.render(width, allocated_heights[i]);
            overlay(&mut canvas, &child_canvas, 0, y_offset);
            y_offset += allocated_heights[i];
            if y_offset >= height {
                break;
            }
        }
        canvas
    }
}

/// Creates a Column widget that arranges its children vertically.
///
/// # Example
///
/// ```rust
/// use widgets::column::column;
/// use widgets::text::text;
///
/// let col = column(vec![
///     text("Line 1"),
///     text("Line 2"),
/// ]);
/// ```
pub fn column<T: IntoWidget>(children: Vec<T>) -> Column {
    Column::new(children)
}
