use crate::typewriter::{sleep, type_chord};
use rdev::Key;

#[allow(dead_code)]
const TYPING_DELAY_MS: u64 = 40;

#[allow(dead_code)]
pub fn script() {
    // Move to the first tmux tab
    type_chord(vec![Key::Alt, Key::Num1]);
    sleep(3000);

    // Navigate
    type_chord(vec![Key::ControlLeft, Key::KeyJ]);
    sleep(700);
    type_chord(vec![Key::ControlLeft, Key::KeyL]);
    sleep(700);
    type_chord(vec![Key::ControlLeft, Key::KeyL]);
    sleep(700);
    type_chord(vec![Key::ControlLeft, Key::KeyK]);
    sleep(700);
    type_chord(vec![Key::ControlLeft, Key::KeyL]);
    sleep(700);
    type_chord(vec![Key::ControlLeft, Key::KeyJ]);
    sleep(700);

    // Resize
    for _ in 0..3 {
        type_chord(vec![Key::Alt, Key::KeyK]);
        sleep(80);
    }
    sleep(700);

    type_chord(vec![Key::ControlLeft, Key::KeyK]);
    sleep(700);
    for _ in 0..3 {
        type_chord(vec![Key::Alt, Key::KeyJ]);
        sleep(80);
    }
    sleep(500);

    type_chord(vec![Key::ControlLeft, Key::KeyJ]);
    sleep(700);
    type_chord(vec![Key::ControlLeft, Key::KeyJ]);
    sleep(700);
    type_chord(vec![Key::ControlLeft, Key::KeyH]);
    sleep(700);

    for _ in 0..3 {
        type_chord(vec![Key::Alt, Key::KeyL]);
        sleep(80);
    }
    sleep(700);
    for _ in 0..3 {
        type_chord(vec![Key::Alt, Key::KeyH]);
        sleep(80);
    }
}
