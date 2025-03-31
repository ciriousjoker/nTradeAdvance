use crate::prelude::*;

pub struct SplashScreen;

impl Screen for SplashScreen {
    fn init(&mut self) -> Result<NavAction> {
        console::set_color(COLOR_MAGENTA);
        Ok(NavAction::None)
    }

    fn build(&mut self) {
        console::print(&render_ui(image(MEW)));
    }

    fn handle_input(&mut self) -> Result<NavAction> {
        sleep(650);

        self.display_frame(50, MEW_FLASH, None);
        self.display_frame(300, MEW, None);
        self.display_frame(50, MEW_FLASH, None);
        self.display_frame(100, MEW, None);
        self.display_frame(50, MEW_FLASH, None);
        self.display_frame(400, MEW, None);
        self.display_frame(100, MEW_FLASH_SPARKLE, Some(COLOR_LIGHTMAGENTA as u8));
        self.display_frame(500, MEW, None);
        self.display_frame(50, MEW, None);
        self.display_frame(50, MEW_FLASH_SPARKLE, Some(COLOR_LIGHTMAGENTA as u8));
        self.display_frame(300, MEW_FLASH_FULL, Some(COLOR_LIGHTMAGENTA as u8));
        self.display_frame(50, MEW, None);

        Ok(NavAction::Go(Box::new(MenuScreen::new())))
    }
}

impl SplashScreen {
    /// Helper function to display a single frame of the splash animation.
    fn display_frame(&self, sleep_duration: u32, image_data: &str, color: Option<u8>) {
        console::clear_screen();
        if let Some(c) = color {
            console::set_color(c);
        }
        console::print(&render_ui(image(image_data)));
        console::flush();
        sleep(sleep_duration);
        if color.is_some() {
            // Reset the color back to default after a colored frame.
            console::set_color(COLOR_MAGENTA);
        }
    }
}
