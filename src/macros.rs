//! Macro system for writing scripts in a simplified DSL.
//!
//! This module provides the `script!` macro and supporting traits that allow
//! you to write keyboard automation scripts using a clean, concise syntax:
//!
//! ```rust
//! script! {
//!     "Hello, World!";          // Types the string with default 50ms delay
//!     100;                       // Sleeps for 100ms
//!     [Key::ControlLeft, Key::KeyA];  // Types a chord (Ctrl+A)
//!     ("Custom delay", 100);    // Types with custom 100ms delay per character
//! }
//! ```

use crate::typewriter::{sleep, type_string};

/// Trait for dispatching script actions based on expression type.
///
/// This trait enables the `script!` macro to automatically determine
/// what action to take based on the type of expression provided.
pub trait ScriptAction {
    fn run(self);
}

// String slice -> type_string with default 50ms delay
impl ScriptAction for &str {
    fn run(self) {
        type_string(self.to_string(), 50);
    }
}

// Owned String -> type_string with default 50ms delay
impl ScriptAction for String {
    fn run(self) {
        type_string(self, 50);
    }
}

// Tuple of (string, delay) -> type_string with custom delay
impl ScriptAction for (&str, u64) {
    fn run(self) {
        type_string(self.0.to_string(), self.1);
    }
}

impl ScriptAction for (String, u64) {
    fn run(self) {
        type_string(self.0, self.1);
    }
}

// Integer types -> sleep
impl ScriptAction for u64 {
    fn run(self) {
        sleep(self);
    }
}

impl ScriptAction for u32 {
    fn run(self) {
        sleep(self as u64);
    }
}

impl ScriptAction for usize {
    fn run(self) {
        sleep(self as u64);
    }
}

impl ScriptAction for i32 {
    fn run(self) {
        sleep(self as u64);
    }
}

impl ScriptAction for i64 {
    fn run(self) {
        sleep(self as u64);
    }
}

/// Main macro for writing scripts in a simplified DSL.
///
/// # Syntax
///
/// The macro accepts a sequence of statements separated by semicolons:
/// - **String literals** (`"text";`) - Types the text with 50ms delay per character
/// - **Tuples** (`("text", delay);`) - Types the text with custom delay per character
/// - **Integers** (`100;`) - Sleeps for that many milliseconds
/// - **Arrays of Keys** (`[Key::A, Key::B];`) - Types a key chord
///
/// # Examples
///
/// ```rust
/// use typewriter::script;
/// use rdev::Key;
///
/// script! {
///     // Type text with default 50ms delay
///     "Hello, World!";
///
///     // Sleep for 300ms
///     300;
///
///     // Type a chord (Ctrl+A)
///     [Key::ControlLeft, Key::KeyA];
///
///     // Type with custom 100ms delay per character
///     ("Slow typing", 100);
///
///     // You can also use normal expressions
///     {
///         let msg = "Dynamic".to_string();
///         msg
///     };
/// }
/// ```
///
/// # How It Works
///
/// The macro processes expressions in order:
/// 1. Arrays (`[...]`) are matched first and expand to `type_chord(vec![...])`
/// 2. All other expressions use the `ScriptAction` trait for type-based dispatch
///    - Strings become `type_string` calls
///    - Integers become `sleep` calls
///    - Tuples become `type_string` calls with custom delays
#[macro_export]
macro_rules! script {
    // Base case - no more tokens
    () => {};

    // Match array of keys -> type_chord
    // This must come before the general expression case
    ([$($key:expr),* $(,)?]; $($rest:tt)*) => {
        $crate::typewriter::type_chord(vec![$($key),*]);
        $crate::script!($($rest)*);
    };

    // Match any other expression -> use ScriptAction trait dispatch
    ($expr:expr; $($rest:tt)*) => {
        $crate::macros::ScriptAction::run($expr);
        $crate::script!($($rest)*);
    };
}
