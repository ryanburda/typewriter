use crate::typewriter::{type_chord, type_string};
use rdev::Key;
use std::{thread, time};

pub fn script() {
    // Move to the above split
    type_chord(vec![Key::ControlLeft, Key::KeyK]);

    // Open `lua/buf-mark/init.lua`
    type_string(":e lua/buf-mark/init.lua".to_string());
    thread::sleep(time::Duration::from_millis(300));
    type_chord(vec![Key::Return]);

    // Add buf-mark
    thread::sleep(time::Duration::from_millis(500));
    type_string(":BufMarkSet i".to_string());
    thread::sleep(time::Duration::from_millis(700));
    type_chord(vec![Key::Return]);

    // Open and buf-mark README.md
    thread::sleep(time::Duration::from_millis(500));
    type_string(":e README.md".to_string());
    thread::sleep(time::Duration::from_millis(700));
    type_chord(vec![Key::Return]);

    thread::sleep(time::Duration::from_millis(1000));
    type_string(":BufMarkSet r".to_string());
    thread::sleep(time::Duration::from_millis(700));
    type_chord(vec![Key::Return]);

    // Move down 16 lines in README.md
    thread::sleep(time::Duration::from_millis(1000));
    for _ in 0..16 {
        type_chord(vec![Key::KeyJ]);
    }

    // Switch back to buf-mark i
    thread::sleep(time::Duration::from_millis(1000));
    type_string(":BufMarkGoto i".to_string());
    thread::sleep(time::Duration::from_millis(700));
    type_chord(vec![Key::Return]);

    // Switch back to buf-mark r
    thread::sleep(time::Duration::from_millis(1000));
    type_string(":BufMarkGoto r".to_string());
    thread::sleep(time::Duration::from_millis(700));
    type_chord(vec![Key::Return]);
}
