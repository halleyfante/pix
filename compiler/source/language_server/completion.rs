use serde::Serialize;

use crate::lexer::token::{Token, TokenKind};
use crate::lexer::Lexer;

#[derive(Debug, PartialEq, Serialize)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum CompletionKind {
    Keyword,
    Color,
    Variable,
    Format,
    Snippet,
}

pub fn complete(source: &str, line: usize, column: usize) -> Vec<CompletionItem> {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(_) => {
            return statement_keywords();
        }
    };

    if is_inside_color_block(&tokens, line, column) {
        return Vec::new();
    }

    let previous_token = find_previous_token(&tokens, line, column);
    let second_previous_token = find_second_previous_token(&tokens, line, column);

    match previous_token {
        Some(token) => match &token.kind {
            TokenKind::With => with_completions(),

            TokenKind::Color => {
                if is_after_with(&tokens, token) {
                    collect_color_names(source)
                } else {
                    statement_keywords()
                }
            }

            TokenKind::In => format_completions(),

            TokenKind::Png | TokenKind::Svg | TokenKind::Webp | TokenKind::Gif => vec![keyword("scale")],

            TokenKind::Grid => Vec::new(),

            TokenKind::Number(_) => {
                match second_previous_token {
                    Some(second) => match &second.kind {
                        TokenKind::Grid => vec![keyword("by")],
                        TokenKind::Radius => vec![keyword("with")],
                        TokenKind::Scale => Vec::new(),
                        TokenKind::By => Vec::new(),
                        _ => statement_keywords(),
                    },
                    None => statement_keywords(),
                }
            }

            TokenKind::Export => Vec::new(),

            TokenKind::StringLiteral(_) => {
                if is_after_export(&tokens, token) {
                    vec![keyword("in")]
                } else {
                    statement_keywords()
                }
            }

            TokenKind::Pixel
            | TokenKind::Circle
            | TokenKind::Line
            | TokenKind::Rectangle
            | TokenKind::Triangle => vec![point_snippet()],

            TokenKind::To => vec![point_snippet()],

            TokenKind::RightParenthesis => {
                completions_after_point(&tokens, token)
            }

            TokenKind::Draw | TokenKind::Erase => expression_starters(),

            TokenKind::And | TokenKind::Or | TokenKind::Not => expression_starters(),

            TokenKind::X | TokenKind::Y => expression_continuations(token),

            _ => statement_keywords(),
        },
        None => statement_keywords(),
    }
}

fn completions_after_point(tokens: &[Token], right_paren: &Token) -> Vec<CompletionItem> {
    let statement_start = find_statement_start(tokens, right_paren);
    match statement_start {
        Some(token) => match &token.kind {
            TokenKind::Pixel => vec![keyword("with")],
            TokenKind::Circle => {
                if has_token_before(tokens, right_paren, TokenKind::Radius) {
                    vec![keyword("with")]
                } else {
                    vec![keyword("radius")]
                }
            }
            TokenKind::Line | TokenKind::Rectangle => {
                if has_token_before(tokens, right_paren, TokenKind::To) {
                    vec![keyword("with")]
                } else {
                    vec![keyword("to")]
                }
            }
            TokenKind::Triangle => {
                let to_count = count_token_before(tokens, right_paren, TokenKind::To);
                if to_count >= 2 {
                    vec![keyword("with")]
                } else {
                    vec![keyword("to")]
                }
            }
            _ => statement_keywords(),
        },
        None => statement_keywords(),
    }
}

fn find_previous_token(tokens: &[Token], line: usize, column: usize) -> Option<&Token> {
    let mut previous = None;
    for token in tokens {
        if token.position.line > line {
            break;
        }
        if token.position.line == line && token.position.column >= column {
            break;
        }
        previous = Some(token);
    }
    previous
}

fn find_second_previous_token(tokens: &[Token], line: usize, column: usize) -> Option<&Token> {
    let mut previous = None;
    let mut second_previous = None;
    for token in tokens {
        if token.position.line > line {
            break;
        }
        if token.position.line == line && token.position.column >= column {
            break;
        }
        second_previous = previous;
        previous = Some(token);
    }
    second_previous
}

fn find_statement_start<'a>(tokens: &'a [Token], before: &Token) -> Option<&'a Token> {
    let mut result = None;
    for token in tokens {
        if std::ptr::eq(token, before) {
            break;
        }
        match &token.kind {
            TokenKind::Pixel
            | TokenKind::Circle
            | TokenKind::Line
            | TokenKind::Rectangle
            | TokenKind::Triangle
            | TokenKind::Draw
            | TokenKind::Erase
            | TokenKind::Clear
            | TokenKind::Grid
            | TokenKind::Export => {
                result = Some(token);
            }
            _ => {}
        }
    }
    result
}

fn has_token_before(tokens: &[Token], before: &Token, kind: TokenKind) -> bool {
    for token in tokens {
        if std::ptr::eq(token, before) {
            break;
        }
        if token.kind == kind {
            return true;
        }
    }
    false
}

fn count_token_before(tokens: &[Token], before: &Token, kind: TokenKind) -> usize {
    let mut count = 0;
    for token in tokens {
        if std::ptr::eq(token, before) {
            break;
        }
        if token.kind == kind {
            count += 1;
        }
    }
    count
}

fn is_after_with(tokens: &[Token], color_token: &Token) -> bool {
    for (index, token) in tokens.iter().enumerate() {
        if std::ptr::eq(token, color_token) && index > 0 {
            return tokens[index - 1].kind == TokenKind::With;
        }
    }
    false
}

fn is_after_export(tokens: &[Token], string_token: &Token) -> bool {
    for (index, token) in tokens.iter().enumerate() {
        if std::ptr::eq(token, string_token) && index > 0 {
            return tokens[index - 1].kind == TokenKind::Export;
        }
    }
    false
}

fn is_inside_color_block(tokens: &[Token], line: usize, column: usize) -> bool {
    let mut inside = false;
    for (index, token) in tokens.iter().enumerate() {
        if token.position.line > line {
            break;
        }
        if token.position.line == line && token.position.column >= column {
            break;
        }
        if token.kind == TokenKind::Color
            && let Some(next) = tokens.get(index + 1)
            && next.kind == TokenKind::LeftBrace
        {
            inside = true;
        }
        if token.kind == TokenKind::RightBrace {
            inside = false;
        }
    }
    inside
}

fn collect_color_names(source: &str) -> Vec<CompletionItem> {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(_) => {
            return Vec::new();
        }
    };

    let mut names = Vec::new();
    let mut inside_color_block = false;

    for (index, token) in tokens.iter().enumerate() {
        if token.kind == TokenKind::Color
            && let Some(next) = tokens.get(index + 1)
            && next.kind == TokenKind::LeftBrace
        {
            inside_color_block = true;
            continue;
        }
        if inside_color_block {
            if token.kind == TokenKind::RightBrace {
                inside_color_block = false;
                continue;
            }
            if let TokenKind::Identifier(name) = &token.kind
                && let Some(next) = tokens.get(index + 1)
                && next.kind == TokenKind::Colon
            {
                names.push(CompletionItem {
                    label: name.clone(),
                    kind: CompletionKind::Color,
                    snippet: None,
                });
            }
        }
    }

    names
}

fn keyword(name: &str) -> CompletionItem {
    CompletionItem {
        label: name.to_string(),
        kind: CompletionKind::Keyword,
        snippet: None,
    }
}

fn point_snippet() -> CompletionItem {
    CompletionItem {
        label: "(x, y)".to_string(),
        kind: CompletionKind::Snippet,
        snippet: Some("(${1}, ${2})".to_string()),
    }
}

fn statement_keywords() -> Vec<CompletionItem> {
    let keywords = [
        "grid", "draw", "erase", "clear", "export", "color",
        "pixel", "line", "rectangle", "triangle", "circle",
        "frame", "copy", "move", "layer", "mirror",
    ];
    keywords.iter().map(|name| keyword(name)).collect()
}

fn with_completions() -> Vec<CompletionItem> {
    vec![
        keyword("color"),
        CompletionItem {
            label: "#".to_string(),
            kind: CompletionKind::Color,
            snippet: None,
        },
    ]
}

fn format_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "png".to_string(),
            kind: CompletionKind::Format,
            snippet: None,
        },
        CompletionItem {
            label: "svg".to_string(),
            kind: CompletionKind::Format,
            snippet: None,
        },
        CompletionItem {
            label: "webp".to_string(),
            kind: CompletionKind::Format,
            snippet: None,
        },
        CompletionItem {
            label: "gif".to_string(),
            kind: CompletionKind::Format,
            snippet: None,
        },
    ]
}

fn expression_starters() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "x".to_string(),
            kind: CompletionKind::Variable,
            snippet: None,
        },
        CompletionItem {
            label: "y".to_string(),
            kind: CompletionKind::Variable,
            snippet: None,
        },
        keyword("not"),
    ]
}

fn expression_continuations(previous: &Token) -> Vec<CompletionItem> {
    let mut items = vec![
        keyword("and"),
        keyword("or"),
    ];
    // Only suggest "with" if we might be in a draw statement context
    match &previous.kind {
        TokenKind::X | TokenKind::Y => {
            items.push(keyword("with"));
        }
        _ => {}
    }
    items
}
