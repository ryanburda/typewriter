use crate::script;
use rdev::Key;

#[allow(dead_code)]
pub fn script() {
    script! {
        // Launch Spotlight
        [Key::MetaLeft, Key::Space];
        300;

        // Type "Terminal"
        "Terminal";
        200;
        [Key::Return];
        1000;

        // Type some commands with the simplified syntax
        "echo 'Hello from the macro!'";
        100;
        [Key::Return];
        500;

        // Example with custom typing delay (100ms per character)
        ("This text types slowly...", 100);
        300;
        [Key::Return];
        500;

        // Navigate with key chords
        [Key::ControlLeft, Key::KeyA];
        200;
        [Key::ControlLeft, Key::KeyK];
        500;

        // Type another command
        "ls -la";
        100;
        [Key::Return];
    }
}
