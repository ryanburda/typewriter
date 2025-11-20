# typewriter

Have a computer type for you

Typewriter is a keyboard automation tool with a simple DSL

![](./docs/typewriter.gif)

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/typewriter`.

## Quick Start

Create a script file (e.g., `hello.tw`):

```
// Simple automation script
[MetaLeft, Space];        // Open Spotlight/launcher
300;                      // Wait 300ms
"Terminal";               // Type "Terminal"
[Return];                 // Press Enter
```

Run it:

```bash
typewriter --file hello.tw
```

## Usage

```bash
# Run from file
typewriter --file script.tw

# Run directly from command line
typewriter --script '"Hello"; 100; [Return];'

# Disable mouse click interrupt
typewriter --file script.tw --no-interrupt
```

## Script Syntax

Four simple statement types:

```
"text";           // Type text (default 50ms delay between characters)
("text", 100);    // Type with custom delay
500;              // Sleep 500ms
[Ctrl, C];        // Key chord (keyboard shortcut)
```

## Pattern Reference

### 1. String Literals (Default Delay)
```rust
"Hello";  // Expands to: type_string("Hello".to_string(), 50)
```

### 2. String with Custom Delay
```rust
("Hello", 100);  // Expands to: type_string("Hello".to_string(), 100)
```

### 3. Integers (Sleep)
```rust
500;  // Expands to: sleep(500)
```

### 4. Key Chords
```rust
[Key::ControlLeft, Key::KeyC];  // Expands to: type_chord(vec![Key::ControlLeft, Key::KeyC])
```

Example scripts are in the [`examples/`](./examples/) directory.

## Reference

[OS Caveats](https://docs.rs/rdev/latest/rdev/index.html#os-caveats)

[Key Enum](https://docs.rs/rdev/latest/rdev/enum.Key.html)
