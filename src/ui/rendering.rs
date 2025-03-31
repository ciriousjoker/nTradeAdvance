#[cfg(feature = "calculator-build")]
use crate::prelude::*;

/// Terminal dimensions for rendering.
pub const NIO_MAX_ROWS: u16 = 30; // NOTE: nspireio.h says 27, real device says 30
pub const NIO_MAX_COLS: u16 = 53; // NOTE: nspireio.h says 64, real device says 53.5

/// If this character would be printed into the screen, it is replaced with a space.
/// This allows us to overlay widgets that don't fill the entire space (rest is filled with spaces).
/// However, for the coloredbox, we need to wipe the background, so this is printed instead.
pub const WIPE_CHAR: char = '\u{E000}';

/// The `Widget` trait defines the interface for all UI widgets.
pub trait Widget {
    /// Returns the minimum (width, height) required by the widget.
    fn min_size(&self) -> (u16, u16);

    /// Renders the widget into a canvas (a 2D vector of characters) of given width and height.
    fn render(&self, width: u16, height: u16) -> Canvas;

    /// Returns an optional flex factor for layout. Default is `None`.
    fn flex_factor(&self) -> Option<i32> {
        None
    }
}

/// A 2D canvas is represented as a vector of character rows.
pub type Canvas = Vec<Vec<char>>;

/// Creates a blank canvas filled with spaces.
pub fn create_canvas(width: u16, height: u16) -> Canvas {
    let mut canvas = Vec::new();
    for _ in 0..height {
        canvas.push(vec![' '; width as usize]);
    }
    canvas
}

/// Converts a canvas into a single string with newline separators.
/// If a cell contains WIPE_CHAR, it is printed as a space.
pub fn canvas_to_string(canvas: &Canvas) -> String {
    canvas
        .iter()
        .map(|row| {
            row.iter()
                .map(|&ch| if ch == WIPE_CHAR { ' ' } else { ch })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Overlays a child canvas onto a parent canvas at (offset_x, offset_y).
/// If a cell in the child canvas equals `WIPE_CHAR`, the parent's cell is replaced with a space.
/// Otherwise, non-space characters in the child override the parent's content.
pub fn overlay(canvas: &mut Canvas, child: &Canvas, offset_x: u16, offset_y: u16) {
    let parent_height = canvas.len();
    let parent_width = if parent_height > 0 {
        canvas[0].len()
    } else {
        0
    };
    for (j, child_row) in child.iter().enumerate() {
        let y = j as u16 + offset_y;
        if y >= parent_height as u16 {
            break;
        }
        for (i, &ch) in child_row.iter().enumerate() {
            let x = i as u16 + offset_x;
            if x >= parent_width as u16 {
                break;
            }
            if ch != ' ' {
                canvas[y as usize][x as usize] = ch;
            }
        }
    }
}

/// Renders a ui into a string using the default terminal dimensions.
pub fn render_ui<W: Widget>(widget: W) -> String {
    let canvas = widget.render(NIO_MAX_COLS, NIO_MAX_ROWS);
    canvas_to_string(&canvas)
}

/// The `IntoWidget` trait allows conversion into a boxed widget.
pub trait IntoWidget {
    fn into_widget(self) -> Box<dyn Widget>;
}

impl<T: Widget + 'static> IntoWidget for T {
    fn into_widget(self) -> Box<dyn Widget> {
        Box::new(self)
    }
}

impl IntoWidget for Box<dyn Widget> {
    fn into_widget(self) -> Box<dyn Widget> {
        self
    }
}

/// Macro to easily build a vector of boxed widgets from items implementing IntoWidget.
///
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate your_crate;
/// use widgets::text::text;
///
/// let children = widget_vec![
///     text("Line 1"),
///     text("Line 2"),
/// ];
/// ```
///
/// This is necessary, because the widgets passed to a column or row must be boxed,
/// but we don't want to box them manually.
/// This macro does the boxing for us.
#[macro_export]
macro_rules! widget_vec {
    ($($w:expr),* $(,)?) => {{
        #[cfg(feature = "calculator-build")]
        {ndless::alloc::vec![$($crate::ui::rendering::IntoWidget::into_widget($w)),*]}

        #[cfg(feature = "desktop")]
        {vec![$($crate::ui::rendering::IntoWidget::into_widget($w)),*]}
    }};
}
