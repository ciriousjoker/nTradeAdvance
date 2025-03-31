#[cfg(feature = "calculator-build")]
mod calculator {
    use crate::bindings::nspireio::*;
    use ndless::ffi::CString;

    /// Prints a string to the calculator's screen.
    pub fn print(s: &str) {
        if let Ok(cstr) = CString::new(s) {
            unsafe { nio_fputs(cstr.as_ptr(), nio_get_default()) };
        }
    }

    /// Clears the calculator's screen.
    pub fn clear_screen() {
        unsafe { nio_clear(nio_get_default()) }
    }

    /// Sets the text color on the calculator.
    pub fn set_color(foreground: u8) {
        unsafe { nio_color(nio_get_default(), NIO_COLOR_BLACK as u8, foreground) }
    }

    /// Initializes the console on the calculator.
    pub fn init_console() {
        unsafe {
            static mut CONSOLE: Option<nio_console> = None;
            CONSOLE = Some(core::mem::zeroed());
            #[allow(static_mut_refs)]
            let csl = CONSOLE.as_mut().unwrap();
            nio_init(
                csl,
                54,
                30,
                0,
                0,
                NIO_COLOR_BLACK as u8,
                NIO_COLOR_WHITE as u8,
                false,
            );
            nio_set_default(csl);
            nio_cursor_type(csl, NIO_CURSOR_UNDERSCORE as i32);
        }
    }

    /// Flushes the console output on the calculator.
    pub fn flush() {
        unsafe {
            nio_fflush(nio_get_default());
        }
    }

    /// Disposes of the console resources on the calculator.
    pub fn dispose() {
        unsafe {
            nio_free(nio_get_default());
        }
    }
}
#[cfg(not(feature = "calculator-build"))]
mod desktop {
    use crate::ui::rendering::{NIO_MAX_COLS, NIO_MAX_ROWS};
    use crossterm::{
        cursor::MoveTo,
        execute, queue,
        style::{Color, Print, SetForegroundColor},
        terminal::{Clear, ClearType, SetSize},
    };
    use std::io::{stdout, Write};

    /// Prints a string to the desktop terminal.
    pub fn print(s: &str) {
        let mut stdout = stdout();
        queue!(stdout, Print(s)).unwrap();
    }

    /// Clears the terminal screen using crossterm commands.
    pub fn clear_screen() {
        let mut stdout = stdout();
        // Clear the entire screen and move the cursor to the top-left corner.
        queue!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        // Purge clears the history.
        // NOTE: It MUST come after ClearType::All, otherwise the history will be cleared,
        // then the screen will be cleared which adds the current screen to the history.
        // -> clear first, then erase history
        queue!(stdout, Clear(ClearType::Purge), MoveTo(0, 0)).unwrap();
    }

    /// Sets the terminal text color based on the provided color code.
    ///
    /// This uses the ANSI value directly via crossterm's `Color::AnsiValue`.
    pub fn set_color(foreground: u8) {
        let mut stdout = stdout();
        queue!(stdout, SetForegroundColor(Color::AnsiValue(foreground))).unwrap();
    }

    /// Initializes the console on the desktop.
    pub fn init_console() {
        let mut stdout = stdout();
        // If possible, set the terminal size to the appropriate dimensions.
        execute!(stdout, SetSize(NIO_MAX_COLS, NIO_MAX_ROWS)).unwrap();
    }

    /// Flushes the terminal output.
    pub fn flush() {
        let mut stdout = stdout();
        stdout.flush().unwrap();
    }

    /// Disposes of the console (no-op for desktop).
    pub fn dispose() {}
}

#[cfg(feature = "calculator-build")]
pub use calculator::*;

#[cfg(not(feature = "calculator-build"))]
pub use desktop::*;
