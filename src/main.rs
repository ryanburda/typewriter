mod scripts;
mod typewriter;

use rdev::{Button, Event, EventType, listen};
use scripts::*;
use std::process;
use std::thread;

fn main() {
    // Spawn a thread to run the script
    let worker_handle = thread::spawn(move || {
        nvim_tmux_navigator::script();
        println!("Keystrokes completed.");
    });

    // Main thread listens for mouse clicks
    let callback = move |event: Event| {
        if let EventType::ButtonPress(Button::Left) = event.event_type {
            println!("\nMouse click detected! Interrupting keystrokes...");
            process::exit(0);
        }
    };

    // Start listening - this blocks until an event triggers exit
    // or the worker thread completes and we exit below
    thread::spawn(move || {
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error);
        }
    });

    // Wait for the worker to finish
    worker_handle.join().unwrap();

    // Worker completed successfully, exit cleanly
    process::exit(0);
}
