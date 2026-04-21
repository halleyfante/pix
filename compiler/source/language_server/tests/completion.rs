use crate::language_server::completion::{complete, CompletionItem, CompletionKind};

fn labels(items: &[CompletionItem]) -> Vec<&str> {
    items.iter().map(|item| item.label.as_str()).collect()
}

fn has_label(items: &[CompletionItem], label: &str) -> bool {
    items.iter().any(|item| item.label == label)
}

#[test]
fn empty_source_returns_statement_keywords() {
    let items = complete("", 1, 1);
    assert!(has_label(&items, "grid"));
    assert!(has_label(&items, "draw"));
    assert!(has_label(&items, "circle"));
    assert!(has_label(&items, "export"));
}

#[test]
fn after_grid_number_suggests_by() {
    let items = complete("grid 17 ", 1, 9);
    assert_eq!(labels(&items), vec!["by"]);
}

#[test]
fn after_grid_suggests_nothing() {
    let items = complete("grid ", 1, 6);
    assert!(items.is_empty());
}

#[test]
fn after_in_suggests_formats() {
    let items = complete("export \"test\" in ", 1, 18);
    assert_eq!(items.len(), 4);
    assert!(has_label(&items, "png"));
    assert!(has_label(&items, "svg"));
    assert!(has_label(&items, "webp"));
    assert!(has_label(&items, "gif"));
    assert_eq!(items[0].kind, CompletionKind::Format);
}

#[test]
fn after_png_suggests_scale() {
    let items = complete("export \"test\" in png ", 1, 22);
    assert_eq!(labels(&items), vec!["scale"]);
}

#[test]
fn after_svg_suggests_scale() {
    let items = complete("export \"test\" in svg ", 1, 22);
    assert_eq!(labels(&items), vec!["scale"]);
}

#[test]
fn after_webp_suggests_scale() {
    let items = complete("export \"test\" in webp ", 1, 23);
    assert_eq!(labels(&items), vec!["scale"]);
}

#[test]
fn after_export_suggests_nothing() {
    let items = complete("export ", 1, 8);
    assert!(items.is_empty());
}

#[test]
fn after_export_filename_suggests_in() {
    let items = complete("export \"test\" ", 1, 15);
    assert_eq!(labels(&items), vec!["in"]);
}

#[test]
fn after_with_suggests_color_and_hex() {
    let items = complete("grid 5 by 5\npixel (2, 2) with ", 2, 19);
    assert!(has_label(&items, "color"));
    assert!(has_label(&items, "#"));
}

#[test]
fn after_with_color_suggests_defined_color_names() {
    let source = "grid 5 by 5\ncolor {\n    warm: #e84a00\n    cold: #0044e8\n}\npixel (2, 2) with color ";
    let items = complete(source, 6, 24);
    let names = labels(&items);
    assert!(names.contains(&"warm"));
    assert!(names.contains(&"cold"));
}

#[test]
fn after_shape_keyword_suggests_point_snippet() {
    let items = complete("grid 5 by 5\ncircle ", 2, 8);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].label, "(x, y)");
    assert_eq!(items[0].kind, CompletionKind::Snippet);
    assert_eq!(items[0].snippet, Some("(${1}, ${2})".to_string()));
}

#[test]
fn after_pixel_point_suggests_with() {
    let items = complete("grid 5 by 5\npixel (2, 2) ", 2, 14);
    assert_eq!(labels(&items), vec!["with"]);
}

#[test]
fn after_circle_point_suggests_radius() {
    let items = complete("grid 5 by 5\ncircle (2, 2) ", 2, 15);
    assert_eq!(labels(&items), vec!["radius"]);
}

#[test]
fn after_circle_radius_number_suggests_with() {
    let items = complete("grid 5 by 5\ncircle (2, 2) radius 3 ", 2, 24);
    assert_eq!(labels(&items), vec!["with"]);
}

#[test]
fn after_line_point_suggests_to() {
    let items = complete("grid 5 by 5\nline (1, 1) ", 2, 13);
    assert_eq!(labels(&items), vec!["to"]);
}

#[test]
fn after_line_to_point_suggests_with() {
    let items = complete("grid 5 by 5\nline (1, 1) to (3, 3) ", 2, 23);
    assert_eq!(labels(&items), vec!["with"]);
}

#[test]
fn after_rectangle_point_suggests_to() {
    let items = complete("grid 5 by 5\nrectangle (0, 0) ", 2, 18);
    assert_eq!(labels(&items), vec!["to"]);
}

#[test]
fn after_rectangle_to_point_suggests_with() {
    let items = complete("grid 5 by 5\nrectangle (0, 0) to (4, 4) ", 2, 28);
    assert_eq!(labels(&items), vec!["with"]);
}

#[test]
fn after_triangle_first_point_suggests_to() {
    let items = complete("grid 5 by 5\ntriangle (0, 0) ", 2, 17);
    assert_eq!(labels(&items), vec!["to"]);
}

#[test]
fn after_triangle_second_point_suggests_to() {
    let items = complete("grid 5 by 5\ntriangle (0, 0) to (4, 0) ", 2, 27);
    assert_eq!(labels(&items), vec!["to"]);
}

#[test]
fn after_triangle_third_point_suggests_with() {
    let items = complete("grid 5 by 5\ntriangle (0, 0) to (4, 0) to (2, 4) ", 2, 39);
    assert_eq!(labels(&items), vec!["with"]);
}

#[test]
fn after_to_suggests_point_snippet() {
    let items = complete("grid 5 by 5\nline (1, 1) to ", 2, 16);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].label, "(x, y)");
    assert_eq!(items[0].kind, CompletionKind::Snippet);
}

#[test]
fn after_draw_suggests_expression_starters() {
    let items = complete("grid 5 by 5\ndraw ", 2, 6);
    assert!(has_label(&items, "x"));
    assert!(has_label(&items, "y"));
    assert!(has_label(&items, "not"));
}

#[test]
fn after_erase_suggests_expression_starters() {
    let items = complete("grid 5 by 5\nerase ", 2, 7);
    assert!(has_label(&items, "x"));
    assert!(has_label(&items, "y"));
    assert!(has_label(&items, "not"));
}

#[test]
fn after_and_suggests_expression_starters() {
    let items = complete("grid 5 by 5\ndraw x = 3 and ", 2, 16);
    assert!(has_label(&items, "x"));
    assert!(has_label(&items, "y"));
}

#[test]
fn after_x_suggests_continuations_with_with() {
    let items = complete("grid 5 by 5\ndraw x ", 2, 8);
    assert!(has_label(&items, "and"));
    assert!(has_label(&items, "or"));
    assert!(has_label(&items, "with"));
}

#[test]
fn inside_color_block_suggests_nothing() {
    let items = complete("grid 5 by 5\ncolor {\n    ", 3, 5);
    assert!(items.is_empty());
}

#[test]
fn after_color_block_suggests_statement_keywords() {
    let items = complete("grid 5 by 5\ncolor {\n    warm: #e84a00\n}\n", 5, 1);
    assert!(has_label(&items, "draw"));
    assert!(has_label(&items, "circle"));
}

#[test]
fn invalid_source_returns_statement_keywords() {
    let items = complete("@@@ invalid !!!", 1, 16);
    assert!(has_label(&items, "grid"));
}

#[test]
fn after_scale_number_suggests_nothing() {
    let items = complete("export \"test\" in png scale 8 ", 1, 29);
    assert!(items.is_empty());
}

#[test]
fn after_by_number_suggests_nothing() {
    let items = complete("grid 17 by 17 ", 1, 15);
    assert!(items.is_empty());
}

#[test]
fn color_names_not_suggested_when_color_is_statement() {
    let source = "grid 5 by 5\ncolor ";
    let items = complete(source, 2, 7);
    assert!(has_label(&items, "grid"));
}
