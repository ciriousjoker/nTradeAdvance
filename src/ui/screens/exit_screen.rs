use crate::prelude::*;

pub struct ExitScreen;

impl Screen for ExitScreen {
    fn init(&mut self) -> Result<NavAction> {
        console::set_color(COLOR_YELLOW);
        Ok(NavAction::None)
    }

    fn build(&mut self) {
        console::print(&render_ui(image(PIKACHU)));
    }

    fn handle_input(&mut self) -> Result<NavAction> {
        sleep(650);

        self.display_frame(50, PIKACHU_SPARK, Some(COLOR_LIGHTYELLOW as u8));
        self.display_frame(300, PIKACHU, None);
        self.display_frame(50, PIKACHU_SPARK, Some(COLOR_LIGHTYELLOW as u8));
        self.display_frame(100, PIKACHU, None);
        self.display_frame(50, PIKACHU_SPARK, Some(COLOR_LIGHTYELLOW as u8));
        self.display_frame(400, PIKACHU, None);
        self.display_frame(100, PIKACHU_SPARK, Some(COLOR_LIGHTYELLOW as u8));
        self.display_frame(500, PIKACHU, None);
        self.display_frame(50, PIKACHU_SPARK, Some(COLOR_LIGHTYELLOW as u8));
        self.display_frame(50, PIKACHU, None);
        self.display_frame(300, PIKACHU_SPARK, Some(COLOR_LIGHTYELLOW as u8));
        self.display_frame(50, PIKACHU, None);

        Ok(NavAction::Pop)
    }
}

impl ExitScreen {
    /// Helper function to display a single frame of the exit animation.
    fn display_frame(&self, sleep_duration: u32, image_data: &str, color: Option<u8>) {
        console::clear_screen();
        if let Some(c) = color {
            console::set_color(c);
        }
        console::print(&render_ui(image(image_data)));
        console::flush();
        sleep(sleep_duration);
        if color.is_some() {
            console::set_color(COLOR_YELLOW);
        }
    }
}
