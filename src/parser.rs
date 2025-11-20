//! Runtime parser for the typewriter DSL.
//!
//! This module provides a parser that can interpret typewriter scripts at runtime,
//! allowing scripts to be loaded from files or command-line arguments instead of
//! being compiled into the binary.

use crate::typewriter::{sleep, type_chord, type_string};
use rdev::Key;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    UnknownKey(String),
    InvalidSyntax(String),
    UnexpectedEnd,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(t) => write!(f, "Unexpected token: {}", t),
            ParseError::UnknownKey(k) => write!(f, "Unknown key: {}", k),
            ParseError::InvalidSyntax(s) => write!(f, "Invalid syntax: {}", s),
            ParseError::UnexpectedEnd => write!(f, "Unexpected end of input"),
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone)]
enum Token {
    String(String),
    Number(u64),
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Comma,
    Semicolon,
    Identifier(String),
}

/// Tokenizes the input script into a stream of tokens
fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Skip whitespace
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            // Skip comments
            '/' if chars.clone().nth(1) == Some('/') => {
                chars.next();
                chars.next();
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c == '\n' {
                        break;
                    }
                }
            }
            // String literals
            '"' => {
                chars.next();
                let mut string = String::new();
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c == '"' {
                        break;
                    }
                    if c == '\\' {
                        if let Some(&next) = chars.peek() {
                            chars.next();
                            match next {
                                'n' => string.push('\n'),
                                't' => string.push('\t'),
                                'r' => string.push('\r'),
                                '\\' => string.push('\\'),
                                '"' => string.push('"'),
                                _ => {
                                    string.push('\\');
                                    string.push(next);
                                }
                            }
                        }
                    } else {
                        string.push(c);
                    }
                }
                tokens.push(Token::String(string));
            }
            // Numbers
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num.parse().unwrap()));
            }
            // Single character tokens
            '[' => {
                chars.next();
                tokens.push(Token::LeftBracket);
            }
            ']' => {
                chars.next();
                tokens.push(Token::RightBracket);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LeftParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RightParen);
            }
            ',' => {
                chars.next();
                tokens.push(Token::Comma);
            }
            ';' => {
                chars.next();
                tokens.push(Token::Semicolon);
            }
            // Identifiers (for Key names)
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' || c == ':' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Identifier(ident));
            }
            _ => {
                return Err(ParseError::UnexpectedToken(ch.to_string()));
            }
        }
    }

    Ok(tokens)
}

/// Parses a Key identifier into an rdev::Key
fn parse_key(name: &str) -> Result<Key, ParseError> {
    // Handle Key:: prefix
    let name = name.strip_prefix("Key::").unwrap_or(name);

    match name {
        // Letters
        "KeyA" | "A" => Ok(Key::KeyA),
        "KeyB" | "B" => Ok(Key::KeyB),
        "KeyC" | "C" => Ok(Key::KeyC),
        "KeyD" | "D" => Ok(Key::KeyD),
        "KeyE" | "E" => Ok(Key::KeyE),
        "KeyF" | "F" => Ok(Key::KeyF),
        "KeyG" | "G" => Ok(Key::KeyG),
        "KeyH" | "H" => Ok(Key::KeyH),
        "KeyI" | "I" => Ok(Key::KeyI),
        "KeyJ" | "J" => Ok(Key::KeyJ),
        "KeyK" | "K" => Ok(Key::KeyK),
        "KeyL" | "L" => Ok(Key::KeyL),
        "KeyM" | "M" => Ok(Key::KeyM),
        "KeyN" | "N" => Ok(Key::KeyN),
        "KeyO" | "O" => Ok(Key::KeyO),
        "KeyP" | "P" => Ok(Key::KeyP),
        "KeyQ" | "Q" => Ok(Key::KeyQ),
        "KeyR" | "R" => Ok(Key::KeyR),
        "KeyS" | "S" => Ok(Key::KeyS),
        "KeyT" | "T" => Ok(Key::KeyT),
        "KeyU" | "U" => Ok(Key::KeyU),
        "KeyV" | "V" => Ok(Key::KeyV),
        "KeyW" | "W" => Ok(Key::KeyW),
        "KeyX" | "X" => Ok(Key::KeyX),
        "KeyY" | "Y" => Ok(Key::KeyY),
        "KeyZ" | "Z" => Ok(Key::KeyZ),

        // Numbers
        "Num0" | "0" => Ok(Key::Num0),
        "Num1" | "1" => Ok(Key::Num1),
        "Num2" | "2" => Ok(Key::Num2),
        "Num3" | "3" => Ok(Key::Num3),
        "Num4" | "4" => Ok(Key::Num4),
        "Num5" | "5" => Ok(Key::Num5),
        "Num6" | "6" => Ok(Key::Num6),
        "Num7" | "7" => Ok(Key::Num7),
        "Num8" | "8" => Ok(Key::Num8),
        "Num9" | "9" => Ok(Key::Num9),

        // Modifiers
        "ControlLeft" | "Ctrl" | "Control" => Ok(Key::ControlLeft),
        "ControlRight" | "RightCtrl" => Ok(Key::ControlRight),
        "ShiftLeft" | "Shift" => Ok(Key::ShiftLeft),
        "ShiftRight" | "RightShift" => Ok(Key::ShiftRight),
        "Alt" | "AltLeft" => Ok(Key::Alt),
        "AltRight" | "RightAlt" => Ok(Key::AltGr),
        "MetaLeft" | "Meta" | "Cmd" | "Command" | "Super" => Ok(Key::MetaLeft),
        "MetaRight" | "RightMeta" | "RightCmd" => Ok(Key::MetaRight),

        // Special keys
        "Return" | "Enter" => Ok(Key::Return),
        "Space" => Ok(Key::Space),
        "Backspace" => Ok(Key::Backspace),
        "Tab" => Ok(Key::Tab),
        "Escape" | "Esc" => Ok(Key::Escape),
        "Delete" | "Del" => Ok(Key::Delete),
        "Home" => Ok(Key::Home),
        "End" => Ok(Key::End),
        "PageUp" => Ok(Key::PageUp),
        "PageDown" => Ok(Key::PageDown),
        "UpArrow" | "Up" => Ok(Key::UpArrow),
        "DownArrow" | "Down" => Ok(Key::DownArrow),
        "LeftArrow" | "Left" => Ok(Key::LeftArrow),
        "RightArrow" | "Right" => Ok(Key::RightArrow),

        // Function keys
        "F1" => Ok(Key::F1),
        "F2" => Ok(Key::F2),
        "F3" => Ok(Key::F3),
        "F4" => Ok(Key::F4),
        "F5" => Ok(Key::F5),
        "F6" => Ok(Key::F6),
        "F7" => Ok(Key::F7),
        "F8" => Ok(Key::F8),
        "F9" => Ok(Key::F9),
        "F10" => Ok(Key::F10),
        "F11" => Ok(Key::F11),
        "F12" => Ok(Key::F12),

        _ => Err(ParseError::UnknownKey(name.to_string())),
    }
}

#[derive(Debug)]
enum Action {
    TypeString(String, u64),
    Sleep(u64),
    TypeChord(Vec<Key>),
}

/// Parses tokens into a list of actions
fn parse_actions(tokens: &[Token]) -> Result<Vec<Action>, ParseError> {
    let mut actions = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            // String literal
            Token::String(s) => {
                i += 1;
                // Check if followed by semicolon or part of tuple
                if i < tokens.len() {
                    match &tokens[i] {
                        Token::Semicolon => {
                            actions.push(Action::TypeString(s.clone(), 50));
                            i += 1;
                        }
                        Token::Comma => {
                            return Err(ParseError::InvalidSyntax(
                                "String followed by comma - use parentheses for tuple: (\"text\", delay)".to_string()
                            ));
                        }
                        _ => {
                            return Err(ParseError::UnexpectedToken(format!("{:?}", tokens[i])));
                        }
                    }
                } else {
                    return Err(ParseError::UnexpectedEnd);
                }
            }
            // Tuple (string, delay)
            Token::LeftParen => {
                i += 1;
                if let Token::String(s) = &tokens.get(i).ok_or(ParseError::UnexpectedEnd)? {
                    let string = s.clone();
                    i += 1;
                    if let Token::Comma = tokens.get(i).ok_or(ParseError::UnexpectedEnd)? {
                        i += 1;
                        if let Token::Number(delay) = tokens.get(i).ok_or(ParseError::UnexpectedEnd)? {
                            let delay = *delay;
                            i += 1;
                            if let Token::RightParen = tokens.get(i).ok_or(ParseError::UnexpectedEnd)? {
                                i += 1;
                                if let Token::Semicolon = tokens.get(i).ok_or(ParseError::UnexpectedEnd)? {
                                    i += 1;
                                    actions.push(Action::TypeString(string, delay));
                                } else {
                                    return Err(ParseError::InvalidSyntax("Expected semicolon after tuple".to_string()));
                                }
                            } else {
                                return Err(ParseError::InvalidSyntax("Expected ) after delay".to_string()));
                            }
                        } else {
                            return Err(ParseError::InvalidSyntax("Expected number for delay".to_string()));
                        }
                    } else {
                        return Err(ParseError::InvalidSyntax("Expected comma in tuple".to_string()));
                    }
                } else {
                    return Err(ParseError::InvalidSyntax("Expected string in tuple".to_string()));
                }
            }
            // Number (sleep)
            Token::Number(n) => {
                i += 1;
                if let Some(Token::Semicolon) = tokens.get(i) {
                    i += 1;
                    actions.push(Action::Sleep(*n));
                } else {
                    return Err(ParseError::InvalidSyntax("Expected semicolon after number".to_string()));
                }
            }
            // Array (key chord)
            Token::LeftBracket => {
                i += 1;
                let mut keys = Vec::new();
                loop {
                    match tokens.get(i).ok_or(ParseError::UnexpectedEnd)? {
                        Token::Identifier(key_name) => {
                            keys.push(parse_key(key_name)?);
                            i += 1;
                            match tokens.get(i).ok_or(ParseError::UnexpectedEnd)? {
                                Token::Comma => {
                                    i += 1;
                                    continue;
                                }
                                Token::RightBracket => {
                                    i += 1;
                                    break;
                                }
                                _ => return Err(ParseError::InvalidSyntax("Expected comma or ] in key array".to_string())),
                            }
                        }
                        Token::RightBracket => {
                            i += 1;
                            break;
                        }
                        _ => return Err(ParseError::InvalidSyntax("Expected key identifier in array".to_string())),
                    }
                }
                if let Some(Token::Semicolon) = tokens.get(i) {
                    i += 1;
                    actions.push(Action::TypeChord(keys));
                } else {
                    return Err(ParseError::InvalidSyntax("Expected semicolon after key array".to_string()));
                }
            }
            Token::Semicolon => {
                // Skip extra semicolons
                i += 1;
            }
            _ => {
                return Err(ParseError::UnexpectedToken(format!("{:?}", tokens[i])));
            }
        }
    }

    Ok(actions)
}

/// Executes a list of actions
fn execute_actions(actions: &[Action]) {
    for action in actions {
        match action {
            Action::TypeString(s, delay) => type_string(s.clone(), *delay),
            Action::Sleep(ms) => sleep(*ms),
            Action::TypeChord(keys) => type_chord(keys.clone()),
        }
    }
}

/// Parses and executes a typewriter script
pub fn run_script(script: &str) -> Result<(), ParseError> {
    let tokens = tokenize(script)?;
    let actions = parse_actions(&tokens)?;
    execute_actions(&actions);
    Ok(())
}
