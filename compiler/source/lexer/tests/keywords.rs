use crate::lexer::Lexer;
use crate::lexer::token::TokenKind;

#[test]
fn tokenize_and() {
    let mut lexer = Lexer::new("and");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::And);
    assert_eq!(tokens[0].text, "and");
}

#[test]
fn tokenize_or() {
    let mut lexer = Lexer::new("or");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Or);
}

#[test]
fn tokenize_not() {
    let mut lexer = Lexer::new("not");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Not);
}

#[test]
fn tokenize_logical_expression() {
    let mut lexer = Lexer::new("and or not");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].kind, TokenKind::And);
    assert_eq!(tokens[1].kind, TokenKind::Or);
    assert_eq!(tokens[2].kind, TokenKind::Not);
}

#[test]
fn tokenize_with() {
    let mut lexer = Lexer::new("with");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::With);
}

#[test]
fn tokenize_in() {
    let mut lexer = Lexer::new("in");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::In);
}

#[test]
fn tokenize_by() {
    let mut lexer = Lexer::new("by");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::By);
}

#[test]
fn tokenize_to() {
    let mut lexer = Lexer::new("to");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::To);
}

#[test]
fn tokenize_radius() {
    let mut lexer = Lexer::new("radius");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Radius);
}

#[test]
fn tokenize_scale() {
    let mut lexer = Lexer::new("scale");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Scale);
}

#[test]
fn tokenize_color() {
    let mut lexer = Lexer::new("color");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Color);
}

#[test]
fn tokenize_x() {
    let mut lexer = Lexer::new("x");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::X);
}

#[test]
fn tokenize_y() {
    let mut lexer = Lexer::new("y");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Y);
}

#[test]
fn tokenize_draw() {
    let mut lexer = Lexer::new("draw");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Draw);
}

#[test]
fn tokenize_erase() {
    let mut lexer = Lexer::new("erase");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Erase);
}

#[test]
fn tokenize_clear() {
    let mut lexer = Lexer::new("clear");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Clear);
}

#[test]
fn tokenize_grid() {
    let mut lexer = Lexer::new("grid");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Grid);
}

#[test]
fn tokenize_export() {
    let mut lexer = Lexer::new("export");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Export);
}

#[test]
fn tokenize_pixel() {
    let mut lexer = Lexer::new("pixel");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Pixel);
}

#[test]
fn tokenize_line() {
    let mut lexer = Lexer::new("line");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Line);
}

#[test]
fn tokenize_rectangle() {
    let mut lexer = Lexer::new("rectangle");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Rectangle);
}

#[test]
fn tokenize_triangle() {
    let mut lexer = Lexer::new("triangle");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Triangle);
}

#[test]
fn tokenize_circle() {
    let mut lexer = Lexer::new("circle");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Circle);
}

#[test]
fn tokenize_png() {
    let mut lexer = Lexer::new("png");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Png);
}

#[test]
fn tokenize_svg() {
    let mut lexer = Lexer::new("svg");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Svg);
}

#[test]
fn tokenize_webp() {
    let mut lexer = Lexer::new("webp");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Webp);
}

#[test]
fn tokenize_gif() {
    let mut lexer = Lexer::new("gif");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Gif);
}

#[test]
fn tokenize_layer() {
    let mut lexer = Lexer::new("layer");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Layer);
}

#[test]
fn tokenize_mirror() {
    let mut lexer = Lexer::new("mirror");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Mirror);
}

#[test]
fn tokenize_frame() {
    let mut lexer = Lexer::new("frame");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Frame);
}

#[test]
fn tokenize_copy() {
    let mut lexer = Lexer::new("copy");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Copy);
}

#[test]
fn tokenize_move() {
    let mut lexer = Lexer::new("move");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Move);
}

#[test]
fn tokenize_at() {
    let mut lexer = Lexer::new("at");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::At);
}

#[test]
fn tokenize_identifier() {
    let mut lexer = Lexer::new("skin");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Identifier("skin".to_string()));
    assert_eq!(tokens[0].text, "skin");
}
