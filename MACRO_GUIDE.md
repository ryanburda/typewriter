# Typewriter DSL

Typewriter includes a simple DSL for writing keyboard automation scripts.

## Syntax

The macro uses semicolons as statement separators and supports these patterns:

```rust
script! {
    "text";                       // Types text with default 50ms delay
    ("text", delay);              // Types text with custom delay
    100;                          // Sleeps for 100ms
    [Key::MetaLeft, Key::Tab];    // Types a key chord
}
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

### 5. Dynamic Expressions
```rust
{
    let msg = format!("Hello {}", name);
    msg
};  // Uses ScriptAction trait to determine action at compile time
```

## How It Works

The macro uses a trait-based dispatch system:

1. **Arrays** are pattern-matched first and expand to `type_chord(vec![...])`
2. **Other expressions** use the `ScriptAction` trait which implements:
   - `&str` / `String` → calls `type_string` with 50ms default delay
   - `(&str, u64)` / `(String, u64)` → calls `type_string` with custom delay
   - Integer types → calls `sleep`

The trait dispatch happens at compile time based on the expression's type.
