use crate::prelude::*;

#[derive(Debug)]
pub struct ErrorScreen {
    pub error: AppError,
}

impl Screen for ErrorScreen {
    fn init(&mut self) -> Result<NavAction> {
        Ok(NavAction::None)
    }

    fn build(&mut self) {
        let ui = border(column(widget_vec![
            align(padding(text("ERROR")).vertical(1)),
            divider('-'),
            text(""),
            flexible(
                1,
                align(
                    border(padding(text(&self.error.to_string()).max_width(40)))
                        .corners(CORNERS_ROUND),
                ),
            ),
            align(sizedbox(button("OK").selected(true)).width(8).height(3)),
            text(""),
            text(""),
        ]))
        .borders(Borders {
            left: Some('|'),
            right: Some('|'),
            top: Some('-'),
            bottom: Some('-'),
        })
        .corners(Corners {
            top_left: Some('+'),
            top_right: Some('+'),
            bottom_left: Some('+'),
            bottom_right: Some('+'),
        });

        let output = render_ui(ui);
        console::print(&output);
        console::flush();
    }

    fn handle_input(&mut self) -> Result<NavAction> {
        let input = wait_input();
        match input {
            InputKey::Enter => Ok(NavAction::Pop),
            _ => Ok(NavAction::None),
        }
    }
}
