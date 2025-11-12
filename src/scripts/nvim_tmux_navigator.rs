use crate::typewriter::{sleep, type_chord, type_string};
use rdev::Key;

#[allow(dead_code)]
const TYPING_DELAY_MS: u64 = 40;

#[allow(dead_code)]
pub fn script() {
    // Move to the below split
    type_chord(vec![Key::ControlLeft, Key::KeyJ]);

    // Open nvim
    type_string("nvim".to_string(), TYPING_DELAY_MS);
    sleep(300);
    type_chord(vec![Key::Return]);

    // Open tmux split below nvim
    sleep(1000);
    type_chord(vec![Key::ControlLeft, Key::Space]);
    type_chord(vec![Key::KeyJ]);
    sleep(1000);

    // run ls
    type_string("l".to_string(), TYPING_DELAY_MS);
    type_chord(vec![Key::Return]);

    // go back into nvim
    type_chord(vec![Key::ControlLeft, Key::KeyK]);
    sleep(500);
    // create vertical split
    type_chord(vec![Key::Space, Key::KeyL]);
    sleep(500);
    // create horizontal split
    type_chord(vec![Key::Space, Key::KeyJ]);
    // resize vertical split
    type_chord(vec![Key::ControlLeft, Key::Space]);
    for _ in 0..5 {
        type_chord(vec![Key::ControlLeft, Key::KeyH]);
        sleep(40);
    }

    // go back to terminal split
    sleep(500);
    type_chord(vec![Key::ControlLeft, Key::KeyJ]);
    sleep(500);

    // resize split up
    type_chord(vec![Key::ControlLeft, Key::Space]);
    for _ in 0..5 {
        type_chord(vec![Key::ControlLeft, Key::KeyK]);
        sleep(40);
    }
}
