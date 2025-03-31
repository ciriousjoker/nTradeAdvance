use crate::prelude::*;
use crate::ui::rendering::{create_canvas, Canvas, Widget};

pub struct ProgressBar {
    /// A value between 0.0 and 1.0 indicating progress.
    pub fraction: f32,
    pub background: char,
    pub foreground: char,
    pub tip: char,
}

impl ProgressBar {
    pub fn new(fraction: f32, background: char, foreground: char, tip: char) -> Self {
        Self {
            fraction,
            background,
            foreground,
            tip,
        }
    }
}

impl Widget for ProgressBar {
    fn min_size(&self) -> (u16, u16) {
        // Minimum size is 5 columns by 1 row.
        (5, 1)
    }
    fn render(&self, width: u16, height: u16) -> Canvas {
        let mut canvas = create_canvas(width, height);
        // Fill the entire area with the background character.
        for row in canvas.iter_mut() {
            for ch in row.iter_mut() {
                *ch = self.background;
            }
        }
        // Clamp the fraction between 0 and 1.
        let fraction = self.fraction.max(0.0).min(1.0);
        let active_cells = ((width as f32) * fraction).floor() as usize;
        // For every row, fill the active area.
        for r in 0..height as usize {
            for x in 0..(width as usize).min(active_cells) {
                if x == active_cells.saturating_sub(1) && active_cells > 0 && fraction < 1.0 {
                    canvas[r][x] = self.tip;
                } else {
                    canvas[r][x] = self.foreground;
                }
            }
        }
        canvas
    }
}

/// Creates a ProgressBar widget.
///
/// # Arguments
///
/// * `fraction` - A value between 0.0 and 1.0 representing progress.
/// * `foreground` - The character to use for the active portion.
/// * `tip` - The character to use for the tip of the active portion.
/// * `background` - The character to use for the inactive portion.
///
/// # Example
///
/// ```rust
/// use widgets::progress_bar::progress_bar;
///
/// // Create a progress bar showing 30% progress.
/// let pb = progress_bar(0.3, '/', '#', '>');
/// ```
pub fn progress_bar(fraction: f32, foreground: char, tip: char, background: char) -> ProgressBar {
    ProgressBar::new(fraction, background, foreground, tip)
}
