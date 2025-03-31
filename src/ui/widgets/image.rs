#[cfg(feature = "calculator-build")]
use crate::prelude::*;
use crate::ui::rendering::{create_canvas, Canvas, Widget};

pub struct Image<'a> {
    pub data: &'a str,
}

impl<'a> Image<'a> {
    /// Creates a new Image widget from the given ASCII art data.
    pub fn new(data: &'a str) -> Self {
        Self { data }
    }
}

impl<'a> Widget for Image<'a> {
    fn min_size(&self) -> (u16, u16) {
        // Split the string into lines.
        let lines: Vec<&str> = self.data.lines().collect();
        let height = lines.len() as u16;
        // Determine the maximum length (in characters) among all lines.
        let width = lines
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0) as u16;
        (width, height)
    }
    fn render(&self, width: u16, height: u16) -> Canvas {
        // Create a canvas with the given allocated dimensions.
        let mut canvas = create_canvas(width, height);
        // Split the data into lines.
        let lines: Vec<&str> = self.data.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if i as u16 >= height {
                break;
            }
            for (j, ch) in line.chars().enumerate() {
                if j as u16 >= width {
                    break;
                }
                canvas[i][j] = ch;
            }
        }
        canvas
    }
}

/// Creates an Image widget from the given ASCII art data string.
///
/// The image is rendered starting at the top-left corner and occupies its natural size
/// determined by the number of lines and the longest line. Extra space in the allocated
/// area remains blank.
///
/// # Example
///
/// ```rust
/// use widgets::image::{image};
///
/// const PIKACHU: &str = "  ,___,\n  [O.o]\n /)  )\\\n  \"--\"";
///
/// let img = image(PIKACHU);
/// ```
pub fn image(data: &str) -> Image {
    Image::new(data)
}
