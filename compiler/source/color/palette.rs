use std::collections::HashMap;

use crate::evaluator::instruction::Instruction;
use crate::parser::ast::ColorValue;

pub type Palette = HashMap<String, Color>;

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Debug, PartialEq)]
pub struct ColorError {
    pub message: String,
}

/// Resolves a color palette from ColorBlock instructions.
pub fn build_palette(instructions: &[Instruction]) -> Result<HashMap<String, Color>, ColorError> {
    let mut palette = HashMap::new();

    for instruction in instructions {
        if let Instruction::ColorBlock { entries } = instruction {
            for entry in entries {
                let color = parse_hexadecimal(&entry.color)?;
                palette.insert(entry.name.clone(), color);
            }
        }
    }

    Ok(palette)
}

/// Resolves a ColorValue to a Color using the palette for named colors.
pub fn resolve_color(
    color_value: &ColorValue,
    palette: &HashMap<String, Color>,
) -> Result<Color, ColorError> {
    match color_value {
        ColorValue::Hexadecimal(hexadecimal) => parse_hexadecimal(hexadecimal),
        ColorValue::Named(name) => palette
            .get(name)
            .cloned()
            .ok_or_else(|| ColorError {
                message: format!("undefined color '{}'", name),
            }),
    }
}

pub fn parse_hexadecimal(hexadecimal: &str) -> Result<Color, ColorError> {
    match hexadecimal.len() {
        3 => {
            let red = parse_hex_digit(hexadecimal.as_bytes()[0], hexadecimal)?;
            let green = parse_hex_digit(hexadecimal.as_bytes()[1], hexadecimal)?;
            let blue = parse_hex_digit(hexadecimal.as_bytes()[2], hexadecimal)?;
            Ok(Color {
                red: red * 17,
                green: green * 17,
                blue: blue * 17,
                alpha: 255,
            })
        }
        6 => {
            let red = parse_hex_pair(&hexadecimal[0..2], hexadecimal)?;
            let green = parse_hex_pair(&hexadecimal[2..4], hexadecimal)?;
            let blue = parse_hex_pair(&hexadecimal[4..6], hexadecimal)?;
            Ok(Color {
                red,
                green,
                blue,
                alpha: 255,
            })
        }
        8 => {
            let red = parse_hex_pair(&hexadecimal[0..2], hexadecimal)?;
            let green = parse_hex_pair(&hexadecimal[2..4], hexadecimal)?;
            let blue = parse_hex_pair(&hexadecimal[4..6], hexadecimal)?;
            let alpha = parse_hex_pair(&hexadecimal[6..8], hexadecimal)?;
            Ok(Color {
                red,
                green,
                blue,
                alpha,
            })
        }
        _ => Err(ColorError {
            message: format!("invalid hex color '{}', expected 3, 6, or 8 digits", hexadecimal),
        }),
    }
}

fn parse_hex_pair(pair: &str, original: &str) -> Result<u8, ColorError> {
    u8::from_str_radix(pair, 16).map_err(|_| ColorError {
        message: format!("invalid hex color '{}'", original),
    })
}

fn parse_hex_digit(digit: u8, original: &str) -> Result<u8, ColorError> {
    match digit {
        b'0'..=b'9' => Ok(digit - b'0'),
        b'a'..=b'f' => Ok(digit - b'a' + 10),
        b'A'..=b'F' => Ok(digit - b'A' + 10),
        _ => Err(ColorError {
            message: format!("invalid hex color '{}'", original),
        }),
    }
}
