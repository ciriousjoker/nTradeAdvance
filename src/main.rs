#![cfg_attr(feature = "calculator-build", no_std)]

// Use ndless for calculator builds.
#[cfg(feature = "calculator-build")]
extern crate ndless;

// Register the global allocator for calculator builds.
#[cfg(feature = "calculator-build")]
extern crate ndless_handler;

mod assets;
mod bindings;
mod errors;
mod navigator;
mod platform;
mod prelude;
mod saves;
mod ui;

use crate::prelude::*;

fn main() {
    console::init_console();

    // Create the navigator with the splash screen as the root screen.
    let mut navigator = Navigator::new(Box::new(SplashScreen));
    navigator.run();

    // Before closing, show the exit screen.
    let mut navigator = Navigator::new(Box::new(ExitScreen));
    navigator.run();

    console::dispose();
}
