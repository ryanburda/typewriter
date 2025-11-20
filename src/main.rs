mod macros;
mod parser;
mod typewriter;

use clap::Parser as ClapParser;
use rdev::{Button, Event, EventType, listen};
use std::fs;
use std::path::PathBuf;
use std::process;
use std::thread;

#[derive(ClapParser, Debug)]
#[command(name = "typewriter")]
#[command(about = "A keyboard automation tool with a simple DSL", long_about = None)]
struct Args {
    /// Script file to execute
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// Script content to execute directly
    #[arg(short, long, value_name = "SCRIPT")]
    script: Option<String>,

    /// Disable mouse click interrupt
    #[arg(long)]
    no_interrupt: bool,
}

fn main() {
    let args = Args::parse();

    // Get script content from either file or direct input
    let script_content = if let Some(file_path) = args.file {
        match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file {:?}: {}", file_path, e);
                process::exit(1);
            }
        }
    } else if let Some(script) = args.script {
        script
    } else {
        eprintln!("Error: Either --file or --script must be provided");
        eprintln!("Use --help for more information");
        process::exit(1);
    };

    // Spawn a thread to run the script
    let worker_handle = thread::spawn(move || {
        match parser::run_script(&script_content) {
            Ok(()) => {
                println!("Script completed successfully.");
            }
            Err(e) => {
                eprintln!("Script error: {}", e);
                process::exit(1);
            }
        }
    });

    if !args.no_interrupt {
        // Main thread listens for mouse clicks
        let callback = move |event: Event| {
            if let EventType::ButtonPress(Button::Left) = event.event_type {
                println!("\nMouse click detected! Interrupting script...");
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
    }

    // Wait for the worker to finish
    worker_handle.join().unwrap();

    // Worker completed successfully, exit cleanly
    process::exit(0);
}
