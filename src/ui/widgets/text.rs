use crate::prelude::*;
use crate::ui::rendering::{create_canvas, Canvas, Widget};

pub struct Text {
    pub text: String,
    pub max_width: Option<u16>,
}

impl Text {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self {
            text: s.into(),
            max_width: None,
        }
    }

    /// Sets a maximum width. When set, the text wraps at word boundaries (spaces).
    pub fn max_width(mut self, width: u16) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Helper to word-wrap the text based on max width (if set).
    fn wrapped_lines(&self) -> Vec<String> {
        match self.max_width {
            Some(max_width) => {
                let mut lines = Vec::new();
                let mut current_line = String::new();
                for word in self.text.split_whitespace() {
                    if current_line.is_empty() {
                        current_line.push_str(word);
                    } else if current_line.len() + 1 + word.len() <= max_width as usize {
                        current_line.push(' ');
                        current_line.push_str(word);
                    } else {
                        lines.push(current_line);
                        current_line = word.to_string();
                    }
                }
                if !current_line.is_empty() {
                    lines.push(current_line);
                }
                lines
            }
            None => vec![self.text.clone()],
        }
    }
}

impl Widget for Text {
    fn min_size(&self) -> (u16, u16) {
        let lines = self.wrapped_lines();
        let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
        (width as u16, lines.len() as u16)
    }

    fn render(&self, width: u16, height: u16) -> Canvas {
        let mut canvas = create_canvas(width, height);
        let lines = self.wrapped_lines();
        for (y, line) in lines.iter().enumerate().take(height as usize) {
            let chars: Vec<char> = line.chars().collect();
            let len = chars.len().min(width as usize);
            for x in 0..len {
                canvas[y][x] = chars[x];
            }
        }
        canvas
    }
}

/// Creates a Text widget for displaying a string with optional word wrapping.
///
/// # Example
///
/// ```rust
/// use widgets::text::text;
///
/// let txt = text("Hello, world!").max_width(20);
/// ```
pub fn text<S: Into<String>>(s: S) -> Text {
    Text::new(s)
}
