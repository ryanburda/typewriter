//! A keyboard simulation library for typing text programmatically.
//!
//! This module provides functions to simulate keyboard input, including
//! individual characters, strings, and key chords (multiple keys pressed
//! simultaneously). It uses the `rdev` library to send keyboard events
//! to the operating system.
//!
//! # Examples
//!
//! ```
//! use typewriter::*;
//!
//! // Type a simple string
//! type_string("Hello, World!".to_string());
//!
//! // Type a keyboard shortcut (Ctrl+C)
//! type_chord(vec![Key::ControlLeft, Key::KeyC]);
//! ```

use rdev::{EventType, Key, SimulateError, simulate};
use std::{thread, time};

/// Pauses execution for the specified number of milliseconds.
///
/// # Arguments
///
/// * `millis` - The number of milliseconds to sleep
///
/// # Examples
///
/// ```
/// sleep(100); // Pauses for 100 milliseconds
/// ```
pub fn sleep(millis: u64) {
    thread::sleep(time::Duration::from_millis(millis));
}

/// Sends a keyboard event and waits 50ms.
///
/// This is an internal helper function that simulates a keyboard event
/// (such as a key press or release) and includes a 50ms delay to ensure
/// proper event processing.
///
/// # Arguments
///
/// * `event_type` - The keyboard event to simulate (KeyPress or KeyRelease)
///
/// # Errors
///
/// Prints an error message to stdout if the event simulation fails.
fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    sleep(50)
}

/// Types a chord by pressing multiple keys simultaneously.
///
/// This function simulates pressing multiple keys at once (like keyboard shortcuts).
/// All keys are pressed in sequence, then all keys are released in the same order.
///
/// # Arguments
///
/// * `keys` - A vector of keys to press together
///
/// # Examples
///
/// ```
/// // Type Ctrl+C
/// type_chord(vec![Key::ControlLeft, Key::KeyC]);
///
/// // Type Shift+A (capital A)
/// type_chord(vec![Key::ShiftLeft, Key::KeyA]);
/// ```
pub fn type_chord(keys: Vec<Key>) {
    // press
    for key in &keys {
        send(&EventType::KeyPress(*key));
    }
    //release
    for key in &keys {
        send(&EventType::KeyRelease(*key));
    }
}

/// Types a single character by simulating the appropriate key combination.
///
/// Supports lowercase letters, uppercase letters, digits, and common symbols.
/// For uppercase letters and symbols requiring Shift, the function automatically
/// includes the Shift key in the chord.
///
/// # Arguments
///
/// * `c` - The character to type
///
/// # Supported Characters
///
/// - Lowercase letters (a-z)
/// - Uppercase letters (A-Z)
/// - Digits (0-9)
/// - Symbols: `! @ # $ % ^ & * ( ) - _ = + [ ] { } \ | ; : ' " , < . > / ? ` ~`
/// - Space
///
/// # Examples
///
/// ```
/// type_char('a');  // Types lowercase 'a'
/// type_char('A');  // Types uppercase 'A' (with Shift)
/// type_char('!');  // Types '!' (Shift+1)
/// ```
///
/// # Note
///
/// Unsupported characters are silently ignored.
pub fn type_char(c: char) {
    match c {
        'a' => type_chord(vec![Key::KeyA]),
        'b' => type_chord(vec![Key::KeyB]),
        'c' => type_chord(vec![Key::KeyC]),
        'd' => type_chord(vec![Key::KeyD]),
        'e' => type_chord(vec![Key::KeyE]),
        'f' => type_chord(vec![Key::KeyF]),
        'g' => type_chord(vec![Key::KeyG]),
        'h' => type_chord(vec![Key::KeyH]),
        'i' => type_chord(vec![Key::KeyI]),
        'j' => type_chord(vec![Key::KeyJ]),
        'k' => type_chord(vec![Key::KeyK]),
        'l' => type_chord(vec![Key::KeyL]),
        'm' => type_chord(vec![Key::KeyM]),
        'n' => type_chord(vec![Key::KeyN]),
        'o' => type_chord(vec![Key::KeyO]),
        'p' => type_chord(vec![Key::KeyP]),
        'q' => type_chord(vec![Key::KeyQ]),
        'r' => type_chord(vec![Key::KeyR]),
        's' => type_chord(vec![Key::KeyS]),
        't' => type_chord(vec![Key::KeyT]),
        'u' => type_chord(vec![Key::KeyU]),
        'v' => type_chord(vec![Key::KeyV]),
        'w' => type_chord(vec![Key::KeyW]),
        'x' => type_chord(vec![Key::KeyX]),
        'y' => type_chord(vec![Key::KeyY]),
        'z' => type_chord(vec![Key::KeyZ]),
        'A' => type_chord(vec![Key::ShiftLeft, Key::KeyA]),
        'B' => type_chord(vec![Key::ShiftLeft, Key::KeyB]),
        'C' => type_chord(vec![Key::ShiftLeft, Key::KeyC]),
        'D' => type_chord(vec![Key::ShiftLeft, Key::KeyD]),
        'E' => type_chord(vec![Key::ShiftLeft, Key::KeyE]),
        'F' => type_chord(vec![Key::ShiftLeft, Key::KeyF]),
        'G' => type_chord(vec![Key::ShiftLeft, Key::KeyG]),
        'H' => type_chord(vec![Key::ShiftLeft, Key::KeyH]),
        'I' => type_chord(vec![Key::ShiftLeft, Key::KeyI]),
        'J' => type_chord(vec![Key::ShiftLeft, Key::KeyJ]),
        'K' => type_chord(vec![Key::ShiftLeft, Key::KeyK]),
        'L' => type_chord(vec![Key::ShiftLeft, Key::KeyL]),
        'M' => type_chord(vec![Key::ShiftLeft, Key::KeyM]),
        'N' => type_chord(vec![Key::ShiftLeft, Key::KeyN]),
        'O' => type_chord(vec![Key::ShiftLeft, Key::KeyO]),
        'P' => type_chord(vec![Key::ShiftLeft, Key::KeyP]),
        'Q' => type_chord(vec![Key::ShiftLeft, Key::KeyQ]),
        'R' => type_chord(vec![Key::ShiftLeft, Key::KeyR]),
        'S' => type_chord(vec![Key::ShiftLeft, Key::KeyS]),
        'T' => type_chord(vec![Key::ShiftLeft, Key::KeyT]),
        'U' => type_chord(vec![Key::ShiftLeft, Key::KeyU]),
        'V' => type_chord(vec![Key::ShiftLeft, Key::KeyV]),
        'W' => type_chord(vec![Key::ShiftLeft, Key::KeyW]),
        'X' => type_chord(vec![Key::ShiftLeft, Key::KeyX]),
        'Y' => type_chord(vec![Key::ShiftLeft, Key::KeyY]),
        'Z' => type_chord(vec![Key::ShiftLeft, Key::KeyZ]),
        '0' => type_chord(vec![Key::Num0]),
        '1' => type_chord(vec![Key::Num1]),
        '2' => type_chord(vec![Key::Num2]),
        '3' => type_chord(vec![Key::Num3]),
        '4' => type_chord(vec![Key::Num4]),
        '5' => type_chord(vec![Key::Num5]),
        '6' => type_chord(vec![Key::Num6]),
        '7' => type_chord(vec![Key::Num7]),
        '8' => type_chord(vec![Key::Num8]),
        '9' => type_chord(vec![Key::Num9]),
        '!' => type_chord(vec![Key::ShiftLeft, Key::Num1]),
        '@' => type_chord(vec![Key::ShiftLeft, Key::Num2]),
        '#' => type_chord(vec![Key::ShiftLeft, Key::Num3]),
        '$' => type_chord(vec![Key::ShiftLeft, Key::Num4]),
        '%' => type_chord(vec![Key::ShiftLeft, Key::Num5]),
        '^' => type_chord(vec![Key::ShiftLeft, Key::Num6]),
        '&' => type_chord(vec![Key::ShiftLeft, Key::Num7]),
        '*' => type_chord(vec![Key::ShiftLeft, Key::Num8]),
        '(' => type_chord(vec![Key::ShiftLeft, Key::Num9]),
        ')' => type_chord(vec![Key::ShiftLeft, Key::Num0]),
        ' ' => type_chord(vec![Key::Space]),
        ',' => type_chord(vec![Key::Comma]),
        '<' => type_chord(vec![Key::ShiftLeft, Key::Comma]),
        '.' => type_chord(vec![Key::Dot]),
        '>' => type_chord(vec![Key::ShiftLeft, Key::Dot]),
        '/' => type_chord(vec![Key::Slash]),
        '?' => type_chord(vec![Key::ShiftLeft, Key::Slash]),
        ';' => type_chord(vec![Key::SemiColon]),
        ':' => type_chord(vec![Key::ShiftLeft, Key::SemiColon]),
        '\'' => type_chord(vec![Key::Quote]),
        '"' => type_chord(vec![Key::ShiftLeft, Key::Quote]),
        '[' => type_chord(vec![Key::LeftBracket]),
        '{' => type_chord(vec![Key::ShiftLeft, Key::LeftBracket]),
        ']' => type_chord(vec![Key::RightBracket]),
        '}' => type_chord(vec![Key::ShiftLeft, Key::RightBracket]),
        '\\' => type_chord(vec![Key::BackSlash]),
        '|' => type_chord(vec![Key::ShiftLeft, Key::BackSlash]),
        '-' => type_chord(vec![Key::Minus]),
        '_' => type_chord(vec![Key::ShiftLeft, Key::Minus]),
        '=' => type_chord(vec![Key::Equal]),
        '+' => type_chord(vec![Key::ShiftLeft, Key::Equal]),
        '`' => type_chord(vec![Key::BackQuote]),
        '~' => type_chord(vec![Key::ShiftLeft, Key::BackQuote]),
        _ => {}
    }
}

/// Types an entire string by simulating keystrokes for each character.
///
/// This function iterates through each character in the string and calls
/// `type_char` to simulate the appropriate key combination. Characters that
/// are not supported by `type_char` will be silently skipped.
///
/// # Arguments
///
/// * `s` - The string to type
///
/// # Examples
///
/// ```
/// type_string("Hello, World!".to_string());
/// type_string("user@example.com".to_string());
/// ```
///
/// # See Also
///
/// - [`type_char`] for details on supported characters
pub fn type_string(s: String) {
    for char in s.chars() {
        type_char(char);
    }
}
