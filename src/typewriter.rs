use rdev::{EventType, Key, SimulateError, simulate};
use std::{thread, time};

pub fn sleep(millis: u64) {
    thread::sleep(time::Duration::from_millis(millis));
}

fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    sleep(50)
}

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

pub fn type_string(s: String) {
    for char in s.chars() {
        type_char(char);
    }
}
