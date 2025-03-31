use crate::ui::rendering::{create_canvas, Canvas, Widget};

/// Orientation for the divider.
#[derive(Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct Divider {
    pub ch: char,
    pub orientation: Orientation,
}

impl Divider {
    pub fn new(ch: char) -> Self {
        Self {
            ch,
            orientation: Orientation::Horizontal,
        }
    }
    pub fn vertical(mut self) -> Self {
        self.orientation = Orientation::Vertical;
        self
    }
}

impl Widget for Divider {
    fn min_size(&self) -> (u16, u16) {
        (1, 1)
    }
    fn render(&self, width: u16, height: u16) -> Canvas {
        match self.orientation {
            Orientation::Horizontal => {
                let mut canvas = create_canvas(width, 1);
                for x in 0..width as usize {
                    canvas[0][x] = self.ch;
                }
                canvas
            }
            Orientation::Vertical => {
                let mut canvas = create_canvas(width, height);
                let col = if width > 1 { width / 2 } else { 0 };
                for r in 0..height as usize {
                    canvas[r][col as usize] = self.ch;
                }
                canvas
            }
        }
    }
}

/// Creates a Divider widget that draws a line using the given character.
///
/// # Example
///
/// ```rust
/// use widgets::divider::divider;
///
/// let horizontal_divider = divider('-');
/// let vertical_divider = divider('-').vertical();
/// ```
pub fn divider(ch: char) -> Divider {
    Divider::new(ch)
}
