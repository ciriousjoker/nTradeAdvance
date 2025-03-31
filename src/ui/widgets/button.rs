use crate::prelude::*;
use crate::ui::rendering::{Canvas, IntoWidget, Widget};

pub struct Button {
    pub text: String,
    pub is_selected: bool,
    pub borders: Option<Borders>,
    pub corners: Option<Corners>,
    pub borders_selected: Option<Borders>,
    pub corners_selected: Option<Corners>,
}

const BORDERS_BTN_SELECTED: Borders = Borders {
    left: Some('|'),
    right: Some('|'),
    top: Some('='),
    bottom: Some('='),
};

// On the calculator, this corner style looks nicer.
#[cfg(feature = "calculator-build")]
const CORNERS_BTN_SELECTED: Corners = Corners {
    top_left: Some('/'),
    top_right: Some('\\'),
    bottom_left: Some('\\'),
    bottom_right: Some('/'),
};

#[cfg(not(feature = "calculator-build"))]
const CORNERS_BTN_SELECTED: Corners = CORNERS_ROUND;

impl Button {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self {
            text: s.into(),
            is_selected: false,
            borders: None,
            corners: None,
            borders_selected: None,
            corners_selected: None,
        }
    }

    /// Sets the selection state of the button.
    pub fn selected(mut self, sel: bool) -> Self {
        self.is_selected = sel;
        self
    }

    /// Helper method that builds the inner widget (Text wrapped in Border) with the
    /// appropriate border and corner style, depending on selection state.
    fn build_widget(&self) -> Box<dyn Widget> {
        // Center the text.
        let text_widget = align(text(&self.text))
            .horizontal(AlignHorizontal::Center)
            .vertical(AlignVertical::Center);

        // Define default styles.
        let normal_borders = self.borders.unwrap_or(Borders {
            left: Some('|'),
            right: Some('|'),
            top: Some('-'),
            bottom: Some('-'),
        });
        let normal_corners = self.corners.unwrap_or(CORNERS_ROUND);
        let selected_borders = self.borders_selected.unwrap_or(BORDERS_BTN_SELECTED);
        let selected_corners = self.corners_selected.unwrap_or(CORNERS_BTN_SELECTED);
        // Choose style based on selection state.
        let (borders, corners) = if self.is_selected {
            (selected_borders, selected_corners)
        } else {
            (normal_borders, normal_corners)
        };

        border(align(text_widget))
            .borders(borders)
            .corners(corners)
            .into_widget()
    }
}

impl Widget for Button {
    fn min_size(&self) -> (u16, u16) {
        // Delegate size calculation to the constructed widget.
        self.build_widget().min_size()
    }

    fn render(&self, width: u16, height: u16) -> Canvas {
        // Delegate rendering to the constructed widget.
        self.build_widget().render(width, height)
    }
}

/// Creates a Button widget with the given label text.
///
/// # Example
///
/// ```rust
/// use widgets::button::button;
///
/// let btn = button("Click Me").selected(true);
/// ```
pub fn button<S: Into<String>>(s: S) -> Button {
    Button::new(s)
}
