use crate::prelude::*;

pub struct AboutScreen;

impl Screen for AboutScreen {
    fn init(&mut self) -> Result<NavAction> {
        Ok(NavAction::None)
    }

    fn build(&mut self) {
        let ui = border(column(widget_vec![
            text(""),
            text(""),
            align(text("Philipp Bauer")),
            align(text("Made in Germany in 2025")),
            text(""),
            align(
                border(
                    padding(
                        sizedbox(
                            text(
                                "This application is provided without any warranty. Make sure you have backups of your save files before using it!"
                            )
                            .max_width(40),
                        ).width(40),
                    )
                    .horizontal(1)
                )
                .corners(CORNERS_ROUND)
            ),
            text(""),
            align(text("The original version from 2014 was written")),
            align(text("in c, v2.0.0 onwards is written in Rust.")),
            text(""),
            text(""),
            padding(column(widget_vec![
                align(column(widget_vec![
                    text("Special thanks to:"),
                    divider('-'),
                ]))
                .horizontal(AlignHorizontal::Start),
                text("- pkmn-savedata by Zayaldrie"),
                text("- ndless-rs by Ben Schattinger (lights0123)"),
                text("- Ndless by the Ndless team"),
            ]))
            .left(3),
            text(""),
            text(""),
            flexible(
                1,
                padding(stack(widget_vec![
                    align(
                        border(text(" < esc "))
                            .borders(Borders {
                                left: None,
                                right: Some('\\'),
                                top: Some('_'),
                                bottom: None,
                            })
                            .corners(CORNERS_NONE),
                    )
                    .horizontal(AlignHorizontal::Start)
                    .vertical(AlignVertical::End),
                    align(column(widget_vec![
                        align(text("github.com/ciriousjoker/nTradeAdvance")),
                        align(text("CC BY-NC-SA 4.0")),
                    ])),
                    align(
                        border(text(format!(" v{} ", env!("CARGO_PKG_VERSION"))))
                            .borders(Borders {
                                left: Some('/'),
                                right: None,
                                top: Some('_'),
                                bottom: None,
                            })
                            .corners(CORNERS_NONE)
                    )
                    .horizontal(AlignHorizontal::End)
                    .vertical(AlignVertical::End)
                ]))
                .horizontal(0)
                .bottom(0)
            ),
        ]));

        let output = render_ui(ui);
        console::print(&output);
        console::flush();
    }

    fn handle_input(&mut self) -> Result<NavAction> {
        let input = wait_input();
        match input {
            InputKey::Escape | InputKey::Enter => Ok(NavAction::Pop),
            _ => Ok(NavAction::None),
        }
    }
}
