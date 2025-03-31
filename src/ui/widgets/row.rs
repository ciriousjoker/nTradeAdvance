#[cfg(feature = "calculator-build")]
use crate::prelude::*;
use crate::ui::rendering::{create_canvas, overlay, Canvas, IntoWidget, Widget};
pub struct Row {
    pub children: Vec<Box<dyn Widget>>,
}

impl Row {
    pub fn new<T: IntoWidget>(children: Vec<T>) -> Self {
        Self {
            children: children.into_iter().map(|c| c.into_widget()).collect(),
        }
    }
}

impl Widget for Row {
    fn min_size(&self) -> (u16, u16) {
        let mut total_width = 0;
        let mut max_height = 0;
        for child in &self.children {
            let (w, h) = child.min_size();
            total_width += w;
            if h > max_height {
                max_height = h;
            }
        }
        (total_width, max_height)
    }
    fn render(&self, width: u16, height: u16) -> Canvas {
        // Create a canvas with the full allocated width and height.
        let mut canvas = create_canvas(width, height);
        // Get each child's natural (minimum) width.
        let min_widths: Vec<u16> = self.children.iter().map(|c| c.min_size().0).collect();
        let total_min: u16 = min_widths.iter().sum();
        let extra = if width > total_min {
            width - total_min
        } else {
            0
        };

        // Sum up flex factors for flexible children.
        let mut total_flex: i32 = 0;
        for child in &self.children {
            if let Some(f) = child.flex_factor() {
                total_flex += f;
            }
        }

        // Allocate widths: non-flexible children get their min_width,
        // while flexible ones get extra space proportionally.
        let mut allocated_widths: Vec<u16> = if total_flex > 0 {
            self.children
                .iter()
                .enumerate()
                .map(|(i, child)| {
                    let base = min_widths[i];
                    let extra_i = if let Some(f) = child.flex_factor() {
                        ((extra as i32) * f) / total_flex
                    } else {
                        0
                    };
                    base + extra_i as u16
                })
                .collect()
        } else {
            min_widths.clone()
        };

        // Adjust for any rounding error: distribute leftover cells.
        let allocated_total: u16 = allocated_widths.iter().sum();
        let mut remainder = if width > allocated_total {
            width - allocated_total
        } else {
            0
        };
        for w in allocated_widths.iter_mut() {
            if remainder > 0 {
                *w += 1;
                remainder -= 1;
            }
        }

        // Render each child into its allocated width.
        let mut x_offset = 0;
        for (i, child) in self.children.iter().enumerate() {
            let child_canvas = child.render(allocated_widths[i], height);
            overlay(&mut canvas, &child_canvas, x_offset, 0);
            x_offset += allocated_widths[i];
            if x_offset >= width {
                break;
            }
        }
        canvas
    }
}

/// Creates a Row widget that arranges its children horizontally.
///
/// # Example
///
/// ```rust
/// use widgets::row::row;
/// use widgets::text::text;
/// use widgets::flexible::flexible;
///
/// // Only the flexible widget gets extra space.
/// let r = row(vec![
///     text("Left"),
///     flexible(1, text("Center")),
///     text("Right"),
/// ]);
/// ```
pub fn row<T: IntoWidget>(children: Vec<T>) -> Row {
    Row::new(children)
}
