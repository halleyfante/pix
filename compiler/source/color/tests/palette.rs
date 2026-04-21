use std::collections::HashMap;

use crate::color::palette::{build_palette, parse_hexadecimal, resolve_color, Color};
use crate::evaluator::instruction::Instruction;
use crate::parser::ast::{ColorEntry, ColorValue};

#[test]
fn parse_six_digit_hexadecimal() {
    let color = parse_hexadecimal("FF0000").unwrap();
    assert_eq!(color, Color { red: 255, green: 0, blue: 0, alpha: 255 });
}

#[test]
fn parse_three_digit_hexadecimal() {
    let color = parse_hexadecimal("F00").unwrap();
    assert_eq!(color, Color { red: 255, green: 0, blue: 0, alpha: 255 });
}

#[test]
fn parse_eight_digit_hexadecimal() {
    let color = parse_hexadecimal("FF000080").unwrap();
    assert_eq!(color, Color { red: 255, green: 0, blue: 0, alpha: 128 });
}

#[test]
fn parse_hexadecimal_lowercase() {
    let color = parse_hexadecimal("ff8800").unwrap();
    assert_eq!(color, Color { red: 255, green: 136, blue: 0, alpha: 255 });
}

#[test]
fn parse_hexadecimal_invalid_length() {
    let result = parse_hexadecimal("FFFF");
    assert!(result.is_err());
}

#[test]
fn build_palette_from_color_block() {
    let instructions = vec![Instruction::ColorBlock {
        entries: vec![
            ColorEntry {
                name: "red".to_string(),
                color: "FF0000".to_string(),
            },
            ColorEntry {
                name: "blue".to_string(),
                color: "0000FF".to_string(),
            },
        ],
    }];
    let palette = build_palette(&instructions).unwrap();
    assert_eq!(palette.get("red").unwrap(), &Color { red: 255, green: 0, blue: 0, alpha: 255 });
    assert_eq!(palette.get("blue").unwrap(), &Color { red: 0, green: 0, blue: 255, alpha: 255 });
}

#[test]
fn resolve_hexadecimal_color() {
    let palette = HashMap::new();
    let color = resolve_color(&ColorValue::Hexadecimal("00FF00".to_string()), &palette).unwrap();
    assert_eq!(color, Color { red: 0, green: 255, blue: 0, alpha: 255 });
}

#[test]
fn resolve_named_color() {
    let mut palette = HashMap::new();
    palette.insert("skin".to_string(), Color { red: 255, green: 203, blue: 150, alpha: 255 });
    let color = resolve_color(&ColorValue::Named("skin".to_string()), &palette).unwrap();
    assert_eq!(color, Color { red: 255, green: 203, blue: 150, alpha: 255 });
}

#[test]
fn resolve_undefined_named_color() {
    let palette = HashMap::new();
    let result = resolve_color(&ColorValue::Named("missing".to_string()), &palette);
    assert!(result.is_err());
}
