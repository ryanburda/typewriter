use crate::typewriter::{sleep, type_chord, type_string};
use rdev::Key;

#[allow(dead_code)]
const TYPING_DELAY_MS: u64 = 40;

#[allow(dead_code)]
pub fn script() {
    // Move to the split to the right
    type_chord(vec![Key::ControlLeft, Key::KeyL]);

    // echo
    type_string("echo 'Hello, World!'".to_string(), TYPING_DELAY_MS);
    sleep(300);
    type_chord(vec![Key::Return]);
    type_string("echo 'Goodbye Moon?'".to_string(), TYPING_DELAY_MS);
    sleep(300);
    type_chord(vec![Key::Return]);
}
