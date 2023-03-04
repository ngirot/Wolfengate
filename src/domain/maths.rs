use std::f32::consts::PI;

use crate::domain::coord::Position;

pub const ANGLE_RIGHT: Angle = Angle::init(0.0);
pub const ANGLE_UP: Angle = Angle::init(PI / 2.0);
pub const ANGLE_DOWN: Angle = Angle::init(3.0 * PI / 2.0);
pub const ANGLE_LEFT: Angle = Angle::init(PI);

#[derive(Copy, Clone)]
pub struct Vector {
    start: Position,
    end: Position,
}

#[derive(Copy, Clone, Debug)]
pub struct Angle {
    radiant: f32,
}

pub fn signed_angle(p1: Position, p2: Position) -> Option<Angle> {
    let points_vector = Vector::new(p1, p2);
    let abscissa_vector = Vector::new(Position::new(0.0, 0.0), Position::new(1.0, 0.0));

    points_vector
        .angle(abscissa_vector)
        .map(|angle| angle.sign(points_vector.angle_sign_is_negative(abscissa_vector)))
}

impl Vector {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn angle(&self, vector: Vector) -> Option<Angle> {
        let len = self.length() * vector.length();
        if len == 0.0 {
            return None;
        }
        let radiant = (self.scalar(vector) / len).acos();
        Some(Angle::new(radiant))
    }

    pub fn scalar(&self, vector: Vector) -> f32 {
        let self_origin = self.to_origin();
        let other_origin = vector.to_origin();

        self_origin.end.x() * other_origin.end.x() + self_origin.end.y() * other_origin.end.y()
    }

    pub fn angle_sign_is_negative(&self, vector: Vector) -> bool {
        let self_origin = self.to_origin().end;
        let other_origin = vector.to_origin().end;

        let z_in_cross_product =
            self_origin.x() * other_origin.y() - self_origin.y() * other_origin.x();
        z_in_cross_product >= 0.0
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

pub struct Move {
    orientation: Angle,
    distance: f32,
}

impl Move {
    pub fn new(orientation: Angle, distance: f32) -> Self {
        Self {
            orientation,
            distance,
        }
    }

    pub fn orientation(&self) -> Angle {
        self.orientation
    }
    pub fn distance(&self) -> f32 {
        self.distance
    }
}

impl Angle {
    const fn init(radiant: f32) -> Self {
        Self { radiant }
    }
    pub fn new(radiant: f32) -> Self {
        Self {
            radiant: radiant % (2.0 * PI),
        }
    }

    pub fn add(&self, other: Angle) -> Self {
        Self {
            radiant: self.radiant + other.radiant,
        }
    }

    pub fn cos(&self) -> f32 {
        self.radiant.cos()
    }

    pub fn sin(&self) -> f32 {
        self.radiant.sin()
    }

    pub fn tan(&self) -> f32 {
        self.radiant.tan()
    }

    pub fn discreet_cone(&self, cone_angle: Angle, number_of_angle: i32) -> Vec<Self> {
        let min = self.radiant + (cone_angle.radiant / 2.0);
        let step = cone_angle.radiant / number_of_angle as f32;

        let mut result = vec![];

        for i in 0..number_of_angle {
            let current_angle = min - step * i as f32;
            result.push(Angle::new(current_angle));
        }

        result
    }

    pub fn position_in_discreet_cone(
        &self,
        cone_angle: Angle,
        number_of_angle: i32,
        angle_negative: bool,
    ) -> f32 {
        let step = number_of_angle as f32 / cone_angle.radiant as f32;
        let angle_sign = if angle_negative { -1.0 } else { 1.0 };
        number_of_angle as f32 / 2.0 + self.radiant * step * angle_sign
    }

    pub fn align_to_x(&self) -> Self {
        if self.cos() >= 0.0 {
            ANGLE_RIGHT
        } else {
            ANGLE_LEFT
        }
    }

    pub fn align_to_y(&self) -> Self {
        if self.sin() >= 0.0 {
            ANGLE_UP
        } else {
            ANGLE_DOWN
        }
    }

    pub fn sign(&self, negative_angle: bool) -> Self {
        if negative_angle {
            Angle::new(-self.radiant)
        } else {
            Angle::new(self.radiant)
        }
    }

    pub fn to_radiant(&self) -> f32 {
        self.radiant
    }
}

#[cfg(test)]
mod fn_test {
    use std::f32::consts::PI;

    use spectral::prelude::*;

    use crate::domain::coord::Position;
    use crate::domain::maths::signed_angle;

    #[test]
    fn should_compute_positive_angle_with_two_points() {
        let start = Position::new(0.0, 0.0);
        let end = Position::new(1.0, 1.0);
        let angle = signed_angle(start, end).map(|angle| angle.to_radiant());

        assert_that(&angle).is_some().is_equal_to(PI / 4.0);
    }

    #[test]
    fn should_compute_positive_negative_angle_with_two_points() {
        let start = Position::new(0.0, 0.0);
        let end = Position::new(1.0, -1.0);
        let angle = signed_angle(start, end).map(|angle| angle.to_radiant());

        assert_that(&angle).is_some().is_equal_to(&(-PI / 4.0));
    }

    #[test]
    fn should_not_compute_angle_of_a_point() {
        let start = Position::new(-1.0, 7.0);
        let end = Position::new(-1.0, 7.0);
        let angle = signed_angle(start, end);

        assert_that(&angle).is_none();
    }
}

#[cfg(test)]
mod vector_test {
    use spectral::prelude::*;

    use crate::domain::coord::Position;
    use crate::domain::maths::Vector;

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

        let angle = vector1.angle(vector2).map(|angle| angle.to_radiant());

        assert_that(&angle).is_some().is_close_to(0.068, 0.001);
    }

    #[test]
    fn should_calculate_angle_on_vector_not_from_origin() {
        let vector1 = Vector::new(Position::new(2.2, 4.3), Position::new(5.7, 2.5));
        let vector2 = Vector::new(Position::new(2.2, 4.3), Position::new(8.4, 1.9));

        let angle = vector1.angle(vector2).map(|angle| angle.to_radiant());

        assert_that(&angle).is_some().is_close_to(0.106, 0.001);
    }

    #[test]
    fn should_calculate_angle_on_vector_with_different_origin() {
        let vector1 = Vector::new(Position::new(1.2, 2.3), Position::new(4.8, 7.1));
        let vector2 = Vector::new(Position::new(4.3, 6.4), Position::new(9.3, 8.7));

        let angle = vector1.angle(vector2).map(|angle| angle.to_radiant());

        assert_that(&angle).is_some().is_close_to(0.496, 0.001);
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

        let sign1 = vector1.angle_sign_is_negative(vector2);
        let sign2 = vector2.angle_sign_is_negative(vector1);

        assert_that(&sign1).is_equal_to(!sign2);
    }

    #[test]
    fn should_calculate_angle_sign() {
        let vector1 = Vector::new(Position::new(1.0, 1.0), Position::new(2.0, 2.0));
        let vector2 = Vector::new(Position::new(-1.0, 4.0), Position::new(6.0, 3.0));

        let sign = vector1.angle_sign_is_negative(vector2);

        assert_that(&sign).is_false();
    }
}
