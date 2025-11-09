use crate::typewriter::{type_chord, type_string};
use rdev::Key;
use std::{thread, time};

pub fn script() {
    // Move to the above split
    type_chord(vec![Key::ControlLeft, Key::KeyK]);

    // Add new buf-mark
    type_string(":BufMarkSet b".to_string());
    thread::sleep(time::Duration::from_millis(400));
    type_chord(vec![Key::Return]);

    // Insert at the bottom of the buffer
    type_string("GA".to_string());
    type_chord(vec![Key::Return]);
    type_chord(vec![Key::Return]);
    type_string(",<.>/?;:'\"[{]}\\|-_=+`~".to_string());
    type_string("1234567890!@#$%^&*()".to_string());
    type_chord(vec![Key::Escape]);
}
