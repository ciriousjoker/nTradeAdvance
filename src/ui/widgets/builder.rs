#[cfg(feature = "calculator-build")]
use crate::prelude::*;
use crate::ui::rendering::{Canvas, IntoWidget, Widget};

pub struct BuilderWidget<F>
where
    F: Fn() -> Box<dyn Widget>,
{
    pub builder: F,
}

impl<F> BuilderWidget<F>
where
    F: Fn() -> Box<dyn Widget>,
{
    pub fn new(builder: F) -> Self {
        Self { builder }
    }
}

impl<F> Widget for BuilderWidget<F>
where
    F: Fn() -> Box<dyn Widget>,
{
    fn min_size(&self) -> (u16, u16) {
        (self.builder)().min_size()
    }
    fn render(&self, width: u16, height: u16) -> Canvas {
        (self.builder)().render(width, height)
    }
}

/// Creates a BuilderWidget that produces a child widget on demand using the provided closure.
///
/// # Example
///
/// ```rust
/// use widgets::builder::builder;
/// use widgets::text::text;
///
/// // Create a builder widget that always returns a new Text widget.
/// let dynamic = builder(|| {
///     // Dynamic logic can be inserted here.
///     text("Dynamic content")
/// });
/// ```
pub fn builder<F, W>(builder: F) -> BuilderWidget<impl Fn() -> Box<dyn Widget>>
where
    F: Fn() -> W,
    W: IntoWidget,
{
    BuilderWidget::new(move || builder().into_widget())
}
