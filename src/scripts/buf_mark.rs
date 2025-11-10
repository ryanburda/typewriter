use crate::typewriter::{sleep, type_chord, type_string};
use rdev::Key;

#[allow(dead_code)]
const TYPING_DELAY_MS: u64 = 40;

#[allow(dead_code)]
pub fn script() {
    // Move to the above split
    type_chord(vec![Key::ControlLeft, Key::KeyK]);

    // Open `lua/buf-mark/init.lua`
    type_string(":e lua/buf-mark/init.lua".to_string(), TYPING_DELAY_MS);
    sleep(300);
    type_chord(vec![Key::Return]);

    // Add buf-mark
    sleep(500);
    type_string(":BufMarkSet i".to_string(), TYPING_DELAY_MS);
    sleep(700);
    type_chord(vec![Key::Return]);

    // Open and buf-mark README.md
    sleep(500);
    type_string(":e README.md".to_string(), TYPING_DELAY_MS);
    sleep(700);
    type_chord(vec![Key::Return]);

    sleep(1000);
    type_string(":BufMarkSet r".to_string(), TYPING_DELAY_MS);
    sleep(700);
    type_chord(vec![Key::Return]);

    // Move down 16 lines in README.md
    sleep(1000);
    for _ in 0..16 {
        type_chord(vec![Key::KeyJ]);
    }

    // Switch back to buf-mark i
    sleep(1000);
    type_string(":BufMarkGoto i".to_string(), TYPING_DELAY_MS);
    sleep(700);
    type_chord(vec![Key::Return]);

    // Switch back to buf-mark r
    sleep(1000);
    type_string(":BufMarkGoto r".to_string(), TYPING_DELAY_MS);
    sleep(700);
    type_chord(vec![Key::Return]);
}
