use crate::prelude::*;

pub enum NavAction {
    Push(Box<dyn Screen>),
    Pop,
    Exit,
    Go(Box<dyn Screen>),
    None,
}

/// The Screen trait defines the interface for all UI screens.
pub trait Screen {
    fn init(&mut self) -> Result<NavAction>;
    fn build(&mut self);
    fn handle_input(&mut self) -> Result<NavAction>;
}

/// The Navigator struct maintains a stack of screens and handles transitions.
pub struct Navigator {
    stack: Vec<Box<dyn Screen>>,
}

impl Navigator {
    /// Creates a new Navigator with the provided root screen.
    pub fn new(mut root: Box<dyn Screen>) -> Self {
        let mut nav = Self { stack: Vec::new() };
        // Initialize root and handle result
        match root.init() {
            Ok(action) => {
                nav.stack.push(root);
                nav.process_action(action);
            }
            Err(e) => {
                nav.stack.push(Box::new(ErrorScreen { error: e }));
            }
        }
        nav
    }

    /// Runs the UI loop until there are no more screens.
    pub fn run(&mut self) {
        while let Some(screen) = self.stack.last_mut() {
            console::clear_screen();
            screen.build();
            console::flush();

            match screen.handle_input() {
                Ok(action) => {
                    self.process_action(action);
                }
                Err(e) => {
                    self.stack.push(Box::new(ErrorScreen { error: e }));
                }
            }
        }
    }

    fn process_action(&mut self, action: NavAction) {
        match action {
            NavAction::Push(mut screen) => {
                if let Err(e) = screen.init() {
                    self.stack.push(Box::new(ErrorScreen { error: e }));
                } else {
                    self.stack.push(screen);
                }
            }
            NavAction::Pop => {
                self.stack.pop();
            }
            NavAction::Exit => {
                self.stack.clear();
            }
            NavAction::Go(mut screen) => {
                if let Err(e) = screen.init() {
                    self.stack.push(Box::new(ErrorScreen { error: e }));
                } else {
                    self.stack.pop();
                    self.stack.push(screen);
                }
            }
            NavAction::None => {}
        }
    }
}
