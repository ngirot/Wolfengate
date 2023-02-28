use crate::domain::actor::SpeedStats;

#[derive(Copy, Clone)]
pub struct ScreenPoint {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
pub struct MapPoint {
    x: i16,
    y: i16,
}

#[derive(Copy, Clone)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone)]
pub struct Vector {
    start: Position,
    end: Position,
}

#[derive(Copy, Clone)]
pub struct Acceleration {
    orientation: f32,
    units_per_seconds_square: f32,
}

#[derive(Copy, Clone)]
pub struct Speed {
    orientation: f32,
    units_per_seconds: f32,
}

pub struct Move {
    orientation: f32,
    distance: f32,
}

pub fn signed_angle(p1: Position, p2: Position) -> Option<f32> {
    let points_vector = Vector::new(p1, p2);
    let abscissa_vector = Vector::new(Position::new(0.0, 0.0), Position::new(1.0, 0.0));

    points_vector.angle(abscissa_vector)
        .map(|angle| angle * points_vector.angle_sign(abscissa_vector))
}

impl ScreenPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

impl MapPoint {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i16 {
        self.x
    }

    pub fn y(&self) -> i16 {
        self.y
    }
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn with_x(&self, x: f32) -> Self {
        Position { x, y: self.y }
    }

    pub fn with_y(&self, y: f32) -> Self {
        Position { x: self.x, y }
    }

    pub fn distance(&self, position: &Position) -> f32 {
        Vector::new(*self, *position).length()
    }

    pub fn apply_force(&self, moves: Move) -> Position {
        let factor = moves.distance();
        let new_angle = moves.orientation();

        Position::new(
            self.x + new_angle.cos() * factor,
            self.y + new_angle.sin() * factor,
        )
    }

    pub fn to_map_point(&self, direction_x: f32, direction_y: f32) -> MapPoint {
        let offset_x: i16 = if direction_x >= 0.0 { 0 } else { 1 };
        let offset_y: i16 = if direction_y >= 0.0 { 0 } else { 1 };

        let x = self.x().floor() as i16 - offset_x;
        let y = self.y().floor() as i16 - offset_y;

        MapPoint::new(x, y)
    }

    pub fn projection_x(&self, angle: f32) -> Position {
        let direction = angle.cos().signum();
        let next_x = Self::round(self.x(), direction);

        self.with_x(next_x)
            .with_y(self.y() + angle.tan() * (next_x - self.x()))
    }

    pub fn projection_y(&self, angle: f32) -> Position {
        let direction = angle.sin().signum();
        let next_y = Self::round(self.y(), direction);

        self.with_x(self.x() + (next_y - self.y()) / angle.tan())
            .with_y(next_y)
    }

    fn round(number: f32, sign: f32) -> f32 {
        let rounded = if sign > 0.0 {
            number.ceil()
        } else {
            number.floor()
        };

        if rounded == number {
            number + sign
        } else {
            rounded
        }
    }
}

impl Vector {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn angle(&self, vector: Vector) -> Option<f32> {
        let len = self.length() * vector.length();
        if len == 0.0 {
            return None;
        }
        Some((self.scalar(vector) / len).acos())
    }

    pub fn scalar(&self, vector: Vector) -> f32 {
        let self_origin = self.to_origin();
        let other_origin = vector.to_origin();

        self_origin.end.x() * other_origin.end.x() + self_origin.end.y() * other_origin.end.y()
    }

    pub fn angle_sign(&self, vector: Vector) -> f32 {
        let self_origin = self.to_origin().end;
        let other_origin = vector.to_origin().end;

        let z_in_cross_product = self_origin.x() * other_origin.y() - self_origin.y() * other_origin.x();
        if z_in_cross_product < 0.0 {
            1.0
        } else {
            -1.0
        }
    }

    pub fn length(&self) -> f32 {
        let x = self.end.x() - self.start.x();
        let y = self.end.y() - self.start.y();

        (x * x + y * y).sqrt()
    }

    fn to_origin(self) -> Self {
        Vector {
            start: Position::new(0.0, 0.0),
            end: Position::new(self.end.x() - self.start.x(), self.end.y() - self.start.y()),
        }
    }
}

impl Speed {
    pub fn new(orientation: f32, units_per_seconds: f32) -> Self {
        Self {
            orientation,
            units_per_seconds,
        }
    }

    pub fn to_move(&self, microseconds_elapsed: u128) -> Move {
        Move::new(self.orientation, microseconds_elapsed as f32 / 1000000.0 * self.units_per_seconds as f32)
    }

    pub fn reduce(&self, reduction: SpeedStats) -> Self {
        Self {
            orientation: self.orientation,
            units_per_seconds: self.units_per_seconds - reduction.units_per_seconds(),
        }
    }

    pub fn add(&self, speed: Speed) -> Speed {
        let x1 = self.units_per_seconds() * self.orientation().cos();
        let y1 = self.units_per_seconds() * self.orientation().sin();

        let x2 = speed.units_per_seconds() * speed.orientation().cos();
        let y2 = speed.units_per_seconds() * speed.orientation().sin();

        let x3 = x1 + x2;
        let y3 = y1 + y2;

        Self {
            orientation: y3.atan2(x3),
            units_per_seconds: ((x3 * x3) + (y3 * y3)).sqrt(),
        }
    }

    pub fn orientation(&self) -> f32 {
        self.orientation
    }

    pub fn units_per_seconds(&self) -> f32 {
        self.units_per_seconds
    }
}

impl Acceleration {
    pub fn new(orientation: f32, units_per_seconds_square: f32) -> Self {
        Self {
            orientation,
            units_per_seconds_square,
        }
    }

    pub fn to_speed(&self, microseconds_elapsed: u128) -> Speed {
        Speed::new(self.orientation, microseconds_elapsed as f32 / 1000000.0 * self.units_per_seconds_square)
    }
}

impl Move {
    pub fn new(orientation: f32, distance: f32) -> Self {
        Self {
            orientation,
            distance,
        }
    }

    pub fn orientation(&self) -> f32 {
        self.orientation
    }
    pub fn distance(&self) -> f32 {
        self.distance
    }
}

#[cfg(test)]
mod fn_test {
    use std::f32::consts::PI;

    use spectral::prelude::*;

    use crate::domain::coord::{Position, signed_angle};

    #[test]
    fn should_compute_positive_angle_with_two_points() {
        let start = Position::new(0.0, 0.0);
        let end = Position::new(1.0, 1.0);
        let angle = signed_angle(start, end);

        assert_that(&angle)
            .is_some()
            .is_equal_to(PI / 4.0);
    }

    #[test]
    fn should_compute_positive_negative_angle_with_two_points() {
        let start = Position::new(0.0, 0.0);
        let end = Position::new(1.0, -1.0);
        let angle = signed_angle(start, end);

        assert_that(&angle)
            .is_some()
            .is_equal_to(-PI / 4.0);
    }

    #[test]
    fn should_not_compute_angle_of_a_point() {
        let start = Position::new(-1.0, 7.0);
        let end = Position::new(-1.0, 7.0);
        let angle = signed_angle(start, end);

        assert_that(&angle)
            .is_none();
    }
}

#[cfg(test)]
mod coord_test {
    use std::f32::consts::PI;

    use spectral::prelude::*;

    use crate::domain::coord::Move;

    use super::Position;

    #[test]
    fn should_have_no_distance_between_the_same_point() {
        let position = Position::new(1.0, 3.0);

        let distance = position.distance(&position);

        assert_that(&distance).is_close_to(0.0, 0.00001);
    }

    #[test]
    fn should_have_distance_between_two_points() {
        let position1 = Position::new(1.0, 3.0);
        let position2 = Position::new(8.1, 7.4);

        let distance = position1.distance(&position2);

        assert_that(&distance).is_close_to(8.352, 0.001);
    }

    #[test]
    fn distance_should_be_bijective() {
        let position1 = Position::new(1.0, 3.0);
        let position2 = Position::new(8.1, 7.4);

        let distance1 = position1.distance(&position2);
        let distance2 = position2.distance(&position1);

        assert_that(&distance1).is_close_to(distance2, 0.0000001);
    }

    #[test]
    fn distance_should_work_with_negative_numbers() {
        let position1 = Position::new(-1.0, 3.0);
        let position2 = Position::new(8.1, -7.4);

        let distance = position1.distance(&position2);

        assert_that(&distance).is_close_to(13.819, 0.001);
    }

    #[test]
    fn position_already_on_x_should_be_projected_to_the_next_int_on_the_right() {
        let position = Position::new(3.0, 2.5);
        let projected = position.projection_x(0.01);

        assert_that(&projected.x()).is_equal_to(4.0);
    }

    #[test]
    fn projected_position_on_x_should_round_on_the_right() {
        let position = Position::new(3.2, 2.5);
        let projected = position.projection_x(0.0);

        assert_that(&projected.x()).is_equal_to(4.0);
    }

    #[test]
    fn projected_position_on_x_should_compute_y_on_the_right() {
        let position = Position::new(3.2, 2.5);
        let projected = position.projection_x(0.15);

        assert_that(&projected.y()).is_close_to(2.620, 0.001);
    }

    #[test]
    fn projected_position_on_x_should_compute_y_on_the_left() {
        let position = Position::new(3.2, 2.5);
        let projected = position.projection_x(PI + 0.15);

        assert_that(&projected.y()).is_close_to(2.469, 0.001);
    }

    #[test]
    fn position_already_on_x_should_be_projected_to_the_next_on_int_the_left() {
        let position = Position::new(3.0, 2.5);
        let projected = position.projection_x(PI);

        assert_that(&projected.x()).is_equal_to(2.0);
    }

    #[test]
    fn projected_position_on_x_should_round_on_the_left() {
        let position = Position::new(3.2, 2.5);
        let projected = position.projection_x(PI - 0.01);

        assert_that(&projected.x()).is_equal_to(3.0);
    }

    #[test]
    fn position_already_on_y_should_be_projected_to_the_next_int_on_the_top() {
        let position = Position::new(3.2, 2.0);
        let projected = position.projection_y(PI / 2.0);

        assert_that(&projected.y()).is_equal_to(3.0);
    }

    #[test]
    fn projected_position_on_y_should_round_on_the_top() {
        let position = Position::new(3.5, 2.6);
        let projected = position.projection_y(PI / 2.0);

        assert_that(&projected.y()).is_equal_to(3.0);
    }

    #[test]
    fn position_already_on_y_should_be_projected_to_the_next_on_int_the_bottom() {
        let position = Position::new(3.5, 2.0);
        let projected = position.projection_y(-PI / 2.0);

        assert_that(&projected.y()).is_equal_to(1.0);
    }

    #[test]
    fn projected_position_on_y_should_round_on_the_bottom() {
        let position = Position::new(3.5, 2.6);
        let projected = position.projection_y(-PI / 2.0);

        assert_that(&projected.y()).is_equal_to(2.0);
    }

    #[test]
    fn projected_position_on_y_should_compute_x_on_the_right() {
        let position = Position::new(3.2, 2.5);
        let projected = position.projection_y(PI / 2.0 - 0.15);

        assert_that(&projected.x()).is_close_to(3.275, 0.001);
    }

    #[test]
    fn projected_position_on_y_should_compute_x_on_the_left() {
        let position = Position::new(3.2, 2.5);
        let projected = position.projection_y((-PI / 2.0) - 0.15);

        assert_that(&projected.x()).is_close_to(3.124, 0.001);
    }

    #[test]
    fn apply_simple_x_force() {
        let position = Position::new(5.0, 10.0);
        let force = Move::new(0.0, 4.0);

        let applied = position.apply_force(force);

        assert_that(&applied.x()).is_close_to(9.0, 0.0);
        assert_that(&applied.y()).is_close_to(10.0, 0.0);
    }

    #[test]
    fn apply_simple_y_force() {
        let position = Position::new(5.0, 10.0);
        let force = Move::new(PI / 2.0, 4.0);

        let applied = position.apply_force(force);

        assert_that(&applied.x()).is_close_to(5.0, 0.0);
        assert_that(&applied.y()).is_close_to(14.0, 0.0);
    }

    #[test]
    fn apply_force_with_angle() {
        let position = Position::new(3.1, 6.4);
        let force = Move::new(1.2, 3.2);

        let applied = position.apply_force(force);

        assert_that(&applied.x()).is_close_to(4.259, 0.001);
        assert_that(&applied.y()).is_close_to(9.382, 0.001);
    }
}

#[cfg(test)]
mod vector_test {
    use spectral::prelude::*;

    use crate::domain::coord::{Position, Vector};

    #[test]
    fn vector_length_of_two_points() {
        let vector = Vector::new(Position::new(2.0, 3.4), Position::new(4.2, 5.3));

        let length = vector.length();

        assert_that(&length).is_close_to(2.907, 0.001);
    }

    #[test]
    fn vector_length_is_between_two_point_is_the_same_in_any_order() {
        let point_1 = Position::new(2.3, 4.5);
        let point_2 = Position::new(2.6, 6.5);

        let length_1 = Vector::new(point_1, point_2).length();
        let length_2 = Vector::new(point_2, point_1).length();

        assert_that(&length_1).is_equal_to(length_2);
    }

    #[test]
    fn vector_length_of_same_point_is_0() {
        let point = Position::new(3.0, 4.0);
        let vector = Vector::new(point, point);

        let length = vector.length();

        assert_that(&length).is_equal_to(0.0);
    }

    #[test]
    fn should_calculate_scalar_on_vector_from_origin() {
        let vector1 = Vector::new(Position::new(0.0, 0.0), Position::new(3.2, 4.2));
        let vector2 = Vector::new(Position::new(0.0, 0.0), Position::new(5.6, 6.4));

        let scalar = vector1.scalar(vector2);

        assert_that(&scalar).is_close_to(44.8, 0.001);
    }

    #[test]
    fn should_calculate_scalar_on_vector_not_from_origin() {
        let vector1 = Vector::new(Position::new(2.2, 4.3), Position::new(5.7, 2.5));
        let vector2 = Vector::new(Position::new(2.2, 4.3), Position::new(8.4, 1.9));

        let scalar = vector1.scalar(vector2);

        assert_that(&scalar).is_close_to(26.02, 0.001);
    }

    #[test]
    fn should_calculate_scalar_on_vector_with_different_origin() {
        let vector1 = Vector::new(Position::new(1.2, 2.3), Position::new(4.8, 7.1));
        let vector2 = Vector::new(Position::new(4.3, 6.4), Position::new(9.3, 8.7));

        let scalar = vector1.scalar(vector2);

        assert_that(&scalar).is_close_to(29.04, 0.001);
    }

    #[test]
    fn should_calculate_angle_on_vector_from_origin() {
        let vector1 = Vector::new(Position::new(0.0, 0.0), Position::new(3.2, 4.2));
        let vector2 = Vector::new(Position::new(0.0, 0.0), Position::new(5.6, 6.4));

        let angle = vector1.angle(vector2);

        assert_that(&angle)
            .is_some()
            .is_close_to(0.068, 0.001);
    }

    #[test]
    fn should_calculate_angle_on_vector_not_from_origin() {
        let vector1 = Vector::new(Position::new(2.2, 4.3), Position::new(5.7, 2.5));
        let vector2 = Vector::new(Position::new(2.2, 4.3), Position::new(8.4, 1.9));

        let angle = vector1.angle(vector2);

        assert_that(&angle)
            .is_some()
            .is_close_to(0.106, 0.001);
    }

    #[test]
    fn should_calculate_angle_on_vector_with_different_origin() {
        let vector1 = Vector::new(Position::new(1.2, 2.3), Position::new(4.8, 7.1));
        let vector2 = Vector::new(Position::new(4.3, 6.4), Position::new(9.3, 8.7));

        let angle = vector1.angle(vector2);

        assert_that(&angle)
            .is_some()
            .is_close_to(0.496, 0.001);
    }

    #[test]
    fn should_not_have_angle_if_first_vector_is_a_point() {
        let position = Position::new(1.2, 1.3);
        let vector1 = Vector::new(position, position);
        let vector2 = Vector::new(Position::new(1.0, 1.0), Position::new(2.0, 2.0));

        let angle = vector1.angle(vector2);

        assert_that(&angle).is_none();
    }

    #[test]
    fn should_not_have_angle_if_second_vector_is_a_point() {
        let vector1 = Vector::new(Position::new(1.0, 1.0), Position::new(2.0, 2.0));
        let position = Position::new(1.2, 1.3);
        let vector2 = Vector::new(position, position);

        let angle = vector1.angle(vector2);

        assert_that(&angle).is_none();
    }

    #[test]
    fn angle_sign_should_be_opposite() {
        let vector1 = Vector::new(Position::new(1.0, 1.0), Position::new(2.0, 2.0));
        let vector2 = Vector::new(Position::new(-1.0, 4.0), Position::new(6.0, 3.0));

        let sign1 = vector1.angle_sign(vector2);
        let sign2 = vector2.angle_sign(vector1);

        assert_that(&sign1).is_equal_to(sign2 * -1.0);
    }

    #[test]
    fn should_calculate_angle_sign() {
        let vector1 = Vector::new(Position::new(1.0, 1.0), Position::new(2.0, 2.0));
        let vector2 = Vector::new(Position::new(-1.0, 4.0), Position::new(6.0, 3.0));

        let sign = vector1.angle_sign(vector2);

        assert_that(&sign).is_equal_to(1.0);
    }
}
