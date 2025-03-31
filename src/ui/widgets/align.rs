use crate::prelude::*;
use crate::ui::rendering::{create_canvas, overlay, Canvas, IntoWidget, Widget};

/// Options for horizontal alignment.
#[derive(PartialEq)]
#[allow(dead_code)]
pub enum AlignHorizontal {
    Start,
    Center,
    End,
    Stretch,
}

/// Options for vertical alignment.
#[derive(PartialEq)]
#[allow(dead_code)]
pub enum AlignVertical {
    Start,
    Center,
    End,
    Stretch,
}

/// Represents the alignment configuration.
pub struct Alignment {
    pub horizontal: AlignHorizontal,
    pub vertical: AlignVertical,
}

pub struct Align {
    pub child: Box<dyn Widget>,
    pub alignment: Alignment,
}

impl Align {
    pub fn new<T: IntoWidget>(child: T) -> Self {
        Self {
            child: child.into_widget(),
            alignment: Alignment {
                horizontal: AlignHorizontal::Center,
                vertical: AlignVertical::Center,
            },
        }
    }

    /// Sets the horizontal alignment.
    pub fn horizontal(mut self, h: AlignHorizontal) -> Self {
        self.alignment.horizontal = h;
        self
    }

    /// Sets the vertical alignment.
    pub fn vertical(mut self, v: AlignVertical) -> Self {
        self.alignment.vertical = v;
        self
    }
}

impl Widget for Align {
    fn min_size(&self) -> (u16, u16) {
        self.child.min_size()
    }
    fn render(&self, width: u16, height: u16) -> Canvas {
        let mut canvas = create_canvas(width, height);
        let (child_w, child_h) = self.child.min_size();
        let render_w = match self.alignment.horizontal {
            AlignHorizontal::Stretch => width,
            _ => child_w.min(width),
        };
        let render_h = match self.alignment.vertical {
            AlignVertical::Stretch => height,
            _ => child_h.min(height),
        };
        let offset_x = match self.alignment.horizontal {
            AlignHorizontal::Start | AlignHorizontal::Stretch => 0,
            AlignHorizontal::Center => (width - render_w) / 2,
            AlignHorizontal::End => width - render_w,
        };
        let offset_y = match self.alignment.vertical {
            AlignVertical::Start | AlignVertical::Stretch => 0,
            AlignVertical::Center => (height - render_h) / 2,
            AlignVertical::End => height - render_h,
        };
        let child_canvas = self.child.render(render_w, render_h);
        overlay(&mut canvas, &child_canvas, offset_x, offset_y);
        canvas
    }
}

/// Creates an Align widget that positions its child according to the specified alignment options.
///
/// # Example
///
/// ```rust
/// use widgets::align::{align, AlignHorizontal, AlignVertical};
/// use widgets::text::text;
///
/// let aligned = align(text("Centered"))
///     .horizontal(AlignHorizontal::Center)
///     .vertical(AlignVertical::Center);
/// ```
pub fn align<T: IntoWidget>(child: T) -> Align {
    Align::new(child)
}
