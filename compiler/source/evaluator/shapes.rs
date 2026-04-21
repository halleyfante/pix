use crate::parser::ast::{Expression, Operator, Point};

fn number(value: u32) -> Expression {
    Expression::Number(value)
}

fn coordinate_x() -> Expression {
    Expression::CoordinateX
}

fn coordinate_y() -> Expression {
    Expression::CoordinateY
}

fn binary(left: Expression, operator: Operator, right: Expression) -> Expression {
    Expression::BinaryOperation {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }
}

fn and(left: Expression, right: Expression) -> Expression {
    binary(left, Operator::And, right)
}

fn equal(left: Expression, right: Expression) -> Expression {
    binary(left, Operator::Equal, right)
}

fn less_than(left: Expression, right: Expression) -> Expression {
    binary(left, Operator::LessThan, right)
}

fn greater_than(left: Expression, right: Expression) -> Expression {
    binary(left, Operator::GreaterThan, right)
}

fn greater_than_or_equal(left: Expression, right: Expression) -> Expression {
    binary(left, Operator::GreaterThanOrEqual, right)
}

fn less_than_or_equal(left: Expression, right: Expression) -> Expression {
    binary(left, Operator::LessThanOrEqual, right)
}

fn subtract(left: Expression, right: Expression) -> Expression {
    binary(left, Operator::Subtract, right)
}

fn multiply(left: Expression, right: Expression) -> Expression {
    binary(left, Operator::Multiply, right)
}

fn add(left: Expression, right: Expression) -> Expression {
    binary(left, Operator::Add, right)
}

fn power(base: Expression, exponent: Expression) -> Expression {
    binary(base, Operator::Power, exponent)
}

/// `x = px and y = py`
pub fn pixel(point: &Point) -> Expression {
    and(
        equal(coordinate_x(), number(point.x)),
        equal(coordinate_y(), number(point.y)),
    )
}

/// `x > x1 and x < x2 and y > y1 and y < y2`
pub fn rectangle(from: &Point, to: &Point) -> Expression {
    let x_min = from.x.min(to.x);
    let x_max = from.x.max(to.x);
    let y_min = from.y.min(to.y);
    let y_max = from.y.max(to.y);

    and(
        and(
            greater_than(coordinate_x(), number(x_min)),
            less_than(coordinate_x(), number(x_max)),
        ),
        and(
            greater_than(coordinate_y(), number(y_min)),
            less_than(coordinate_y(), number(y_max)),
        ),
    )
}

/// `(x - cx)^2 + (y - cy)^2 < r^2`
pub fn circle(center: &Point, radius: u32) -> Expression {
    let horizontal_distance = power(subtract(coordinate_x(), number(center.x)), number(2));
    let vertical_distance = power(subtract(coordinate_y(), number(center.y)), number(2));
    less_than(add(horizontal_distance, vertical_distance), power(number(radius), number(2)))
}

/// Bresenham-style: for each edge, check which side the point is on.
/// A point is inside the triangle if it's on the same side of all three edges.
pub fn triangle(first: &Point, second: &Point, third: &Point) -> Expression {
    and(
        and(
            edge_condition(first, second, third),
            edge_condition(second, third, first),
        ),
        edge_condition(third, first, second),
    )
}

fn edge_condition(start: &Point, end: &Point, reference: &Point) -> Expression {
    let cross_for_point = cross_product_expression(start, end);
    let reference_value = cross_product_value(start, end, reference);

    if reference_value > 0 {
        greater_than(cross_for_point, number(0))
    } else if reference_value < 0 {
        less_than(cross_for_point, number(0))
    } else {
        // Reference point is on the edge — degenerate triangle.
        // Use equal to zero so points on the edge are included.
        equal(cross_for_point, number(0))
    }
}

fn cross_product_expression(start: &Point, end: &Point) -> Expression {
    subtract(
        multiply(
            subtract(number(end.x), number(start.x)),
            subtract(coordinate_y(), number(start.y)),
        ),
        multiply(
            subtract(number(end.y), number(start.y)),
            subtract(coordinate_x(), number(start.x)),
        ),
    )
}

fn cross_product_value(start: &Point, end: &Point, point: &Point) -> i64 {
    let edge_horizontal = end.x as i64 - start.x as i64;
    let edge_vertical = end.y as i64 - start.y as i64;
    let point_horizontal = point.x as i64 - start.x as i64;
    let point_vertical = point.y as i64 - start.y as i64;
    edge_horizontal * point_vertical - edge_vertical * point_horizontal
}

/// Produces a condition for a line from `from` to `to`.
/// Handles horizontal, vertical, and diagonal cases.
pub fn line(from: &Point, to: &Point) -> Expression {
    let horizontal_delta = to.x as i64 - from.x as i64;
    let vertical_delta = to.y as i64 - from.y as i64;

    if horizontal_delta == 0 && vertical_delta == 0 {
        // Single point, same as pixel.
        and(
            equal(coordinate_x(), number(from.x)),
            equal(coordinate_y(), number(from.y)),
        )
    } else if vertical_delta == 0 {
        let x_min = from.x.min(to.x);
        let x_max = from.x.max(to.x);
        and(
            equal(coordinate_y(), number(from.y)),
            and(
                greater_than_or_equal(coordinate_x(), number(x_min)),
                less_than_or_equal(coordinate_x(), number(x_max)),
            ),
        )
    } else if horizontal_delta == 0 {
        let y_min = from.y.min(to.y);
        let y_max = from.y.max(to.y);
        and(
            equal(coordinate_x(), number(from.x)),
            and(
                greater_than_or_equal(coordinate_y(), number(y_min)),
                less_than_or_equal(coordinate_y(), number(y_max)),
            ),
        )
    } else {
        let cross = cross_product_expression(from, to);
        let x_min = from.x.min(to.x);
        let x_max = from.x.max(to.x);
        let y_min = from.y.min(to.y);
        let y_max = from.y.max(to.y);

        and(
            equal(cross, number(0)),
            and(
                and(
                    greater_than_or_equal(coordinate_x(), number(x_min)),
                    less_than_or_equal(coordinate_x(), number(x_max)),
                ),
                and(
                    greater_than_or_equal(coordinate_y(), number(y_min)),
                    less_than_or_equal(coordinate_y(), number(y_max)),
                ),
            ),
        )
    }
}
