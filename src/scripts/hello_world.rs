use crate::script;
use rdev::Key;

#[allow(dead_code)]
const TYPING_DELAY_MS: u64 = 40;

#[allow(dead_code)]
pub fn script() {
    // ===== NEW SIMPLIFIED SYNTAX USING THE MACRO =====
    script! {
        // Move to below tmux split
        [Key::ControlLeft, Key::KeyJ];

        // hello world
        ("echo 'Hello, World!'", TYPING_DELAY_MS);
        300;
        [Key::Return];

        // goodbye moon
        ("echo 'Goodbye Moon?'", TYPING_DELAY_MS);
        300;
        [Key::Return];
    }
}

// ===== OLD VERBOSE SYNTAX (for comparison) =====
// use crate::typewriter::{sleep, type_chord, type_string};
// use rdev::Key;
//
// #[allow(dead_code)]
// const TYPING_DELAY_MS: u64 = 40;
//
// #[allow(dead_code)]
// pub fn script() {
//     // Move to below tmux split
//     type_chord(vec![Key::ControlLeft, Key::KeyJ]);
//
//     // hello world
//     type_string("echo 'Hello, World!'".to_string(), TYPING_DELAY_MS);
//     sleep(300);
//     type_chord(vec![Key::Return]);
//
//     // goodbye moon
//     type_string("echo 'Goodbye Moon?'".to_string(), TYPING_DELAY_MS);
//     sleep(300);
//     type_chord(vec![Key::Return]);
// }
