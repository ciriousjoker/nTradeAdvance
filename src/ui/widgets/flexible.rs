#[cfg(feature = "calculator-build")]
use crate::prelude::*;
use crate::ui::rendering::{Canvas, IntoWidget, Widget};

pub struct Flexible {
    pub flex: i32,
    pub child: Box<dyn Widget>,
}

impl Flexible {
    /// Creates a new Flexible widget with the given flex factor and child.
    pub fn new<T: IntoWidget>(flex: i32, child: T) -> Self {
        Self {
            flex,
            child: child.into_widget(),
        }
    }
}

impl Widget for Flexible {
    fn min_size(&self) -> (u16, u16) {
        self.child.min_size()
    }
    fn render(&self, width: u16, height: u16) -> Canvas {
        self.child.render(width, height)
    }
    fn flex_factor(&self) -> Option<i32> {
        Some(self.flex)
    }
}

/// Creates a Flexible widget that wraps a child widget and assigns a flex factor for layout.
///
/// # Example
///
/// ```rust
/// use widgets::flexible::flexible;
/// use widgets::text::text;
///
/// let flex = flexible(1, text("Flexible content"));
/// ```
pub fn flexible<T: IntoWidget>(flex: i32, child: T) -> Flexible {
    Flexible::new(flex, child)
}
