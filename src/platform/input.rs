pub enum InputKey {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Escape,
}

#[cfg(feature = "calculator-build")]
mod calculator {
    use super::InputKey;
    use ndless::input::get_keys;
    use ndless::input::touchpad::touchpad_scan;
    use ndless::input::wait_key_pressed;
    use ndless::input::Key;

    /// Waits for and returns an input key on the calculator.
    pub fn wait_input() -> InputKey {
        use ndless::input::wait_no_key_pressed;
        loop {
            wait_key_pressed();
            let touchpad = touchpad_scan();
            if touchpad.is_ok() {
                let touchpad = touchpad.unwrap();
                if touchpad.pressed {
                    let input = match touchpad.arrow {
                        Some(Key::Up) => Some(InputKey::Up),
                        Some(Key::Down) => Some(InputKey::Down),
                        Some(Key::Left) => Some(InputKey::Left),
                        Some(Key::Right) => Some(InputKey::Right),
                        _ => None,
                    };
                    if let Some(key) = input {
                        return key;
                    }
                }
            }
            let keys = get_keys();
            if keys.contains(&Key::Esc) {
                return InputKey::Escape;
            } else if keys.contains(&Key::Enter) {
                return InputKey::Enter;
            }
            wait_no_key_pressed();
        }
    }
}

#[cfg(feature = "desktop")]
mod desktop {
    use super::InputKey;
    use core::time::Duration;
    use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind};
    use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

    /// Waits for and returns an input key from the terminal,
    /// reacting only to keydown (press) events.
    pub fn wait_input() -> InputKey {
        enable_raw_mode().expect("Failed to enable raw mode");

        // Discards any buffered input events.
        while poll(Duration::from_millis(0)).unwrap() {
            let _ = read();
        }

        let input = loop {
            if let Ok(Event::Key(KeyEvent { code, kind, .. })) = read() {
                // Only react if this is a key press event.
                if kind == KeyEventKind::Press {
                    match code {
                        KeyCode::Up => break InputKey::Up,
                        KeyCode::Down => break InputKey::Down,
                        KeyCode::Left => break InputKey::Left,
                        KeyCode::Right => break InputKey::Right,
                        KeyCode::Enter => break InputKey::Enter,
                        KeyCode::Esc => break InputKey::Escape,
                        _ => {}
                    }
                }
            }
        };

        disable_raw_mode().expect("Failed to disable raw mode");
        input
    }
}

#[cfg(feature = "calculator-build")]
pub use calculator::*;

#[cfg(feature = "desktop")]
pub use desktop::*;
