use crate::prelude::*;

pub struct MenuScreen {
    selected: usize,
}

impl MenuScreen {
    pub fn new() -> Self {
        MenuScreen { selected: 0 }
    }
}

impl Screen for MenuScreen {
    fn init(&mut self) -> Result<NavAction> {
        console::set_color(COLOR_WHITE);
        Ok(NavAction::None)
    }

    fn build(&mut self) {
        let ui = border(column(widget_vec![
            align(image(LOGO)),
            align(text("Advance")),
            text(""),
            text(""),
            align(
                padding(column(widget_vec![
                    text("Trade Pokemon between Ruby, Sapphire,"),
                    align(text("Emerald, FireRed & LeafGreen!")),
                    text(""),
                    text(""),
                    align(sizedbox(button("Trade").selected(self.selected == 0)).width(16)),
                    text(" "),
                    align(sizedbox(button("About").selected(self.selected == 1)).width(16)),
                    text(" "),
                    align(sizedbox(button("Exit").selected(self.selected == 2)).width(16)),
                ]))
                .left(1)
                .right(2)
            ),
            text(""),
        ]));

        let output = render_ui(ui);
        console::print(&output);
        console::flush();
    }

    fn handle_input(&mut self) -> Result<NavAction> {
        let input = wait_input();
        match input {
            InputKey::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
                Ok(NavAction::None)
            }
            InputKey::Down => {
                if self.selected < 2 {
                    self.selected += 1;
                }
                Ok(NavAction::None)
            }
            InputKey::Enter => {
                return match self.selected {
                    0 => Ok(NavAction::Push(Box::new(TradeScreen::new()))),
                    1 => Ok(NavAction::Push(Box::new(AboutScreen))),
                    2 => Ok(NavAction::Exit),
                    _ => panic!("Invalid action."),
                }
            }
            InputKey::Escape => Ok(NavAction::Pop),
            _ => Ok(NavAction::None),
        }
    }
}
