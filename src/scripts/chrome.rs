use crate::typewriter::{sleep, type_chord, type_string};
use rdev::Key;

#[allow(dead_code)]
const TYPING_DELAY_MS: u64 = 40;

#[allow(dead_code)]
pub fn script() {
    // Launch Spotlight
    type_chord(vec![Key::MetaLeft, Key::Space]);
    sleep(300);

    // Open Chrome
    type_string("Chro".to_string(), TYPING_DELAY_MS);
    sleep(80);
    type_chord(vec![Key::Return]);
    sleep(2000);

    // Create 5 Chrome tabs
    for _ in 0..5 {
        type_chord(vec![Key::MetaLeft, Key::KeyT]);
        sleep(500);
    }
    sleep(1000);

    // Go back into search bar and search for something
    type_chord(vec![Key::MetaLeft, Key::KeyL]);
    sleep(500);
    type_string("cavapoos for sale".to_string(), TYPING_DELAY_MS);
    type_chord(vec![Key::Return]);
    sleep(3000);

    // Quit Chrome
    type_chord(vec![Key::MetaLeft, Key::KeyQ]);
}
