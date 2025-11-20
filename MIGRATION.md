# Migration Guide: From Embedded to External Scripts

This guide explains how to migrate from the old embedded script system to the new external script files.

## What Changed

**Before:** Scripts were Rust modules compiled into the binary
**After:** Scripts are external `.tw` files parsed at runtime

## Benefits of External Scripts

1. **No recompilation** - Edit scripts without rebuilding the binary
2. **Portable** - Share scripts between machines without the codebase
3. **Simpler syntax** - No Rust boilerplate required
4. **Language agnostic** - Scripts are just text files

## Migration Steps

### Step 1: Convert Script Syntax

The DSL syntax remains the same, but you no longer need Rust code around it.

**Old (Rust module):**
```rust
use crate::script;
use rdev::Key;

#[allow(dead_code)]
pub fn script() {
    script! {
        [Key::ControlLeft, Key::KeyJ];
        ("echo 'Hello, World!'", 40);
        300;
        [Key::Return];
    }
}
```

**New (.tw file):**
```
// hello.tw
[ControlLeft, J];
("echo 'Hello, World!'", 40);
300;
[Return];
```

Changes:
- Remove all Rust imports and function wrapper
- Remove `Key::` prefix from key names (e.g., `Key::ControlLeft` → `ControlLeft`)
- Keep everything inside the `script! { }` block

### Step 2: Save as .tw Files

1. Create a directory for your scripts (e.g., `~/typewriter-scripts/`)
2. Extract the script content (everything inside `script! { }`)
3. Remove Rust-specific syntax (`use`, `pub fn`, `Key::` prefix)
4. Save with `.tw` extension

### Step 3: Run Scripts

**Old way:**
```bash
# Had to modify main.rs and recompile
cargo run
```

**New way:**
```bash
# Just run any script file
typewriter --file ~/typewriter-scripts/hello.tw
```

## Example Migrations

### Example 1: hello_world.rs → hello.tw

**src/scripts/hello_world.rs:**
```rust
use crate::script;
use rdev::Key;

const TYPING_DELAY_MS: u64 = 40;

pub fn script() {
    script! {
        [Key::ControlLeft, Key::KeyJ];
        ("echo 'Hello, World!'", TYPING_DELAY_MS);
        300;
        [Key::Return];
    }
}
```

**hello.tw:**
```
// Move to terminal
[ControlLeft, J];

// Type command
("echo 'Hello, World!'", 40);
300;
[Return];
```

### Example 2: chrome.rs → chrome.tw

**src/scripts/chrome.rs:**
```rust
use crate::typewriter::{sleep, type_chord, type_string};
use rdev::Key;

const TYPING_DELAY_MS: u64 = 40;

pub fn script() {
    type_chord(vec![Key::MetaLeft, Key::Space]);
    sleep(300);
    type_string("Chro".to_string(), TYPING_DELAY_MS);
    // ... etc
}
```

**chrome.tw:**
```
// Launch Spotlight
[MetaLeft, Space];
300;

// Open Chrome
("Chro", 40);
// ... etc
```

## Key Name Changes

| Old (Rust) | New (Script) |
|------------|--------------|
| `Key::ControlLeft` | `ControlLeft` or `Ctrl` |
| `Key::ShiftLeft` | `ShiftLeft` or `Shift` |
| `Key::MetaLeft` | `MetaLeft`, `Meta`, or `Cmd` |
| `Key::Return` | `Return` or `Enter` |
| `Key::KeyA` | `A` or `KeyA` |
| `Key::Space` | `Space` |

The new parser accepts multiple aliases for convenience.

## Removing Old Script Files

After migrating, you can remove the `src/scripts/` directory:

```bash
# Optional: back up first
mv src/scripts ~/typewriter-scripts-backup

# Remove from repo
rm -rf src/scripts
```

Update `src/main.rs` to remove the `mod scripts;` line if you haven't already.

## Still Want Compiled Scripts?

The `script!` macro still works if you want to embed scripts in Rust code:

```rust
use crate::script;
use rdev::Key;

fn my_script() {
    script! {
        "Hello";
        [Return];
    }
}
```

This can be useful for:
- Distribution (single binary with scripts built-in)
- Complex scripts that need Rust logic (loops, conditionals)
- Type safety at compile time

## Questions?

Check [MACRO_GUIDE.md](MACRO_GUIDE.md) for full syntax reference and examples.
