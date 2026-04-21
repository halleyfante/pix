use crate::evaluator::evaluate::Evaluator;
use crate::evaluator::instruction::Value;
use crate::evaluator::shapes;
use crate::parser::ast::Point;

fn is_inside(expression: &crate::parser::ast::Expression, x: i64, y: i64) -> bool {
    match Evaluator::evaluate_expression(expression, x, y).unwrap() {
        Value::Boolean(result) => result,
        Value::Number(_) => panic!("expected boolean"),
    }
}

#[test]
fn pixel_matches_exact_point() {
    let expression = shapes::pixel(&Point { x: 3, y: 5 });
    assert!(is_inside(&expression, 3, 5));
    assert!(!is_inside(&expression, 3, 4));
    assert!(!is_inside(&expression, 2, 5));
    assert!(!is_inside(&expression, 0, 0));
}

#[test]
fn rectangle_matches_interior() {
    let expression = shapes::rectangle(&Point { x: 1, y: 1 }, &Point { x: 5, y: 5 });
    assert!(is_inside(&expression, 2, 2));
    assert!(is_inside(&expression, 3, 4));
    assert!(!is_inside(&expression, 1, 1));
    assert!(!is_inside(&expression, 5, 5));
    assert!(!is_inside(&expression, 0, 3));
    assert!(!is_inside(&expression, 3, 6));
}

#[test]
fn rectangle_swapped_points() {
    let expression = shapes::rectangle(&Point { x: 5, y: 5 }, &Point { x: 1, y: 1 });
    assert!(is_inside(&expression, 2, 2));
    assert!(is_inside(&expression, 3, 4));
    assert!(!is_inside(&expression, 1, 1));
    assert!(!is_inside(&expression, 5, 5));
}

#[test]
fn circle_matches_interior() {
    let expression = shapes::circle(&Point { x: 8, y: 8 }, 4);
    assert!(is_inside(&expression, 8, 8));
    assert!(is_inside(&expression, 9, 8));
    assert!(is_inside(&expression, 8, 10));
    assert!(!is_inside(&expression, 8, 12));
    assert!(!is_inside(&expression, 12, 8));
    assert!(!is_inside(&expression, 0, 0));
}

#[test]
fn circle_large_radius_does_not_panic() {
    let expression = shapes::circle(&Point { x: 5, y: 5 }, 65536);
    assert!(is_inside(&expression, 5, 5));
}

#[test]
fn line_horizontal() {
    let expression = shapes::line(&Point { x: 2, y: 5 }, &Point { x: 7, y: 5 });
    assert!(is_inside(&expression, 2, 5));
    assert!(is_inside(&expression, 4, 5));
    assert!(is_inside(&expression, 7, 5));
    assert!(!is_inside(&expression, 1, 5));
    assert!(!is_inside(&expression, 8, 5));
    assert!(!is_inside(&expression, 4, 4));
}

#[test]
fn line_vertical() {
    let expression = shapes::line(&Point { x: 3, y: 1 }, &Point { x: 3, y: 6 });
    assert!(is_inside(&expression, 3, 1));
    assert!(is_inside(&expression, 3, 4));
    assert!(is_inside(&expression, 3, 6));
    assert!(!is_inside(&expression, 3, 0));
    assert!(!is_inside(&expression, 3, 7));
    assert!(!is_inside(&expression, 2, 4));
}

#[test]
fn line_diagonal() {
    let expression = shapes::line(&Point { x: 0, y: 0 }, &Point { x: 8, y: 8 });
    assert!(is_inside(&expression, 0, 0));
    assert!(is_inside(&expression, 4, 4));
    assert!(is_inside(&expression, 8, 8));
    assert!(!is_inside(&expression, 1, 2));
    assert!(!is_inside(&expression, 9, 9));
}

#[test]
fn line_single_point() {
    let expression = shapes::line(&Point { x: 3, y: 5 }, &Point { x: 3, y: 5 });
    assert!(is_inside(&expression, 3, 5));
    assert!(!is_inside(&expression, 3, 4));
    assert!(!is_inside(&expression, 2, 5));
}

#[test]
fn triangle_matches_interior() {
    let expression = shapes::triangle(
        &Point { x: 0, y: 0 },
        &Point { x: 10, y: 0 },
        &Point { x: 5, y: 10 },
    );
    assert!(is_inside(&expression, 5, 3));
    assert!(is_inside(&expression, 5, 1));
    assert!(!is_inside(&expression, 0, 0));
    assert!(!is_inside(&expression, 0, 10));
    assert!(!is_inside(&expression, 10, 10));
}
