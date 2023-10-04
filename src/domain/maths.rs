use std::f32::consts::PI;

use crate::domain::topology::coord::{Position, ProjectedSprite};
use crate::domain::ui::view::ViewScreen;

pub const ANGLE_RIGHT: Angle = Angle::init(0.0);
pub const ANGLE_UP: Angle = Angle::init(PI / 2.0);
pub const ANGLE_DOWN: Angle = Angle::init(3.0 * PI / 2.0);
pub const ANGLE_LEFT: Angle = Angle::init(PI);

pub const ANGLE_0: Angle = ANGLE_RIGHT;
pub const ANGLE_90: Angle = ANGLE_UP;
pub const ANGLE_180: Angle = ANGLE_LEFT;
pub const ANGLE_240: Angle = ANGLE_DOWN;

#[derive(Copy, Clone)]
pub struct Vector {
    start: Position,
    end: Position,
}

#[derive(Copy, Clone, Debug)]
pub struct Angle {
    radiant: f32,
}

pub fn decimal_part(number: f32) -> f32 {
    let n = number.abs();
    n - n.floor()
}

pub fn between(min: f32, value: f32, max: f32) -> f32 {
    if value < min {
        return min;
    }

    if value > max {
        return max;
    }

    value
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

    pub fn from_angle(angle: Angle) -> Self {
        Self {
            start: Position::new(0.0, 0.0),
            end: Position::new(angle.cos(), angle.sin()),
        }
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

    pub fn to_position(&self, force: f32) -> Position {
        Position::new(self.cos() * force, self.sin() * force)
    }

    pub fn from_degree(degree: f32) -> Self {
        Angle::new(degree * PI / 180.0)
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

    pub fn discreet_cone_spherical_space(&self, cone_angle: Angle, number_of_angle: i32) -> Vec<Self> {
        let min = self.radiant + (cone_angle.radiant / 2.0);
        let step = cone_angle.radiant / (number_of_angle - 1) as f32;

        let mut result = vec![];

        for i in 0..number_of_angle {
            let current_angle = min - step * i as f32;
            result.push(Angle::new(current_angle));
        }

        result
    }

    pub fn discreet_cone_straight_space(&self, cone_angle: Angle, number_of_angle: i32) -> Vec<Self> {
        let half_angle = Angle::new(cone_angle.to_radiant() / 2.0);
        let max_x_distance = -half_angle.tan();
        let step = max_x_distance * 2.0 / (number_of_angle - 1) as f32;

        let mut result = vec![];

        let angle_reference = Vector::new(Position::new(0.0, 0.0), Position::new(0.0, 1.0));

        for i in 0..number_of_angle {
            let current_x_position = max_x_distance - (step * (i as f32));
            let current_vector = Vector::new(Position::new(0.0, 0.0), Position::new(current_x_position, 1.0));
            let current_angle = current_vector.angle(angle_reference).unwrap();
            let factor = if current_vector.angle_sign_is_negative(angle_reference) { -1.0 } else { 1.0 };
            result.push(Angle::new(self.radiant + current_angle.radiant * factor));
        }

        result
    }

    pub fn position_in_discreet_cone_straight(&self,
                                              view: &ViewScreen,
                                              view_orientation: &Angle,
                                              sprite: Position) -> Option<ProjectedSprite> {
        let camera = Position::new(view_orientation.cos(), view_orientation.sin());
        let plane = view.view_plane(view_orientation);

        let inverse_det = 1.0 / (plane.x() * camera.y() - camera.x() * plane.y());//required for correct matrix multiplication

        let x = inverse_det * (camera.y() * sprite.x() - camera.x() * sprite.y());
        let y = inverse_det * (-plane.y() * sprite.x() + plane.x() * sprite.y()); //this is actually the depth inside the screen, that what Z is in 3D

        if y <= 0.0 {
            None
        } else {
            let x = 1.0 + x / y;
            let screen_adjustment = view.width() as f32 - ((view.width() as f32/ 2.0) * x);
            Some(ProjectedSprite::new(screen_adjustment, y))
        }
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

    pub fn multiplication(&self, factor: f32) -> Self {
        Angle::new(self.radiant * factor)
    }

    pub fn addition(&self, angle_to_add: Angle) -> Self {
        Angle::new(self.radiant + angle_to_add.radiant)
    }
}


#[cfg(test)]
mod fn_decimal_part {
    use spectral::assert_that;
    use spectral::prelude::*;

    use crate::domain::maths::decimal_part;

    #[test]
    fn should_get_decimal_part_of_a_number() {
        let decimal = decimal_part(1.23);
        assert_that!(decimal).is_close_to(0.23, 0.001);
    }

    #[test]
    fn should_get_decimal_part_of_a_negative_number() {
        let decimal = decimal_part(-1.23);
        assert_that!(decimal).is_close_to(0.23, 0.001);
    }
}

#[cfg(test)]
mod fn_between {
    use spectral::prelude::*;

    use crate::domain::maths::between;

    #[test]
    fn should_keep_value_if_not_bigger_than_max_and_not_lower_than_min() {
        let value = between(1.0, 1.5, 2.0);
        assert_that!(value).is_equal_to(1.5);
    }

    #[test]
    fn should_get_min_if_value_is_lower() {
        let value = between(10.0, 5.0, 20.0);
        assert_that!(value).is_equal_to(10.0);
    }

    #[test]
    fn should_get_max_if_value_is_bigger() {
        let value = between(2.0, 5.3, 5.0);
        assert_that!(value).is_equal_to(5.0);
    }
}

#[cfg(test)]
mod angle_test {
    use std::f32::consts::PI;

    use spectral::prelude::*;

    use crate::domain::maths::Angle;

    #[test]
    fn angle_should_be_created_from_radiant() {
        let angle = Angle::new(PI / 2.0);
        assert_that!(angle.to_radiant()).is_close_to(PI / 2.0, 0.001);
    }

    #[test]
    fn angle_should_be_created_from_radiant_modulo_2_pi() {
        let angle = Angle::new(8.0 * PI);
        assert_that!(angle.to_radiant()).is_close_to(0.0, 0.001);
    }

    #[test]
    fn angle_could_be_created_from_degrees() {
        let angle = Angle::from_degree(90.0);
        assert_that!(angle.to_radiant()).is_close_to(PI / 2.0, 0.001)
    }

    #[test]
    fn angle_could_be_created_from_degrees_modulo_2_pi() {
        let angle = Angle::from_degree(540.0);
        assert_that!(angle.to_radiant()).is_close_to(PI, 0.001)
    }

    #[test]
    fn angle_should_be_multiplied_by_a_factor_lower_than_1() {
        let angle = Angle::new(1.2);
        let result = angle.multiplication(0.5);

        assert_that!(result.to_radiant()).is_close_to(0.6, 0.001);
    }

    #[test]
    fn angle_should_be_multiplied_by_a_factor_greater_than_1() {
        let angle = Angle::new(1.2);
        let result = angle.multiplication(3.2);

        assert_that!(result.to_radiant()).is_close_to(3.84, 0.001);
    }

    #[test]
    fn angle_should_be_added_with_a_positive_angle() {
        let angle = Angle::new(1.2);
        let result = angle.addition(Angle::new(2.5));

        assert_that!(result.to_radiant()).is_close_to(3.7, 0.001);
    }

    #[test]
    fn angle_should_be_added_with_a_negative_angle() {
        let angle = Angle::new(1.2);
        let result = angle.addition(Angle::new(-2.5));

        assert_that!(result.to_radiant()).is_close_to(-1.3, 0.001);
    }
}


#[cfg(test)]
mod fn_test {
    use std::f32::consts::PI;

    use spectral::prelude::*;

    use crate::domain::maths::signed_angle;
    use crate::domain::topology::coord::Position;

    #[test]
    fn should_compute_positive_angle_with_two_points() {
        let start = Position::new(0.0, 0.0);
        let end = Position::new(1.0, 1.0);
        let angle = signed_angle(start, end).map(|angle| angle.to_radiant());

        assert_that!(angle).is_some().is_equal_to(PI / 4.0);
    }

    #[test]
    fn should_compute_positive_negative_angle_with_two_points() {
        let start = Position::new(0.0, 0.0);
        let end = Position::new(1.0, -1.0);
        let angle = signed_angle(start, end).map(|angle| angle.to_radiant());

        assert_that!(angle).is_some().is_equal_to(&(-PI / 4.0));
    }

    #[test]
    fn should_not_compute_angle_of_a_point() {
        let start = Position::new(-1.0, 7.0);
        let end = Position::new(-1.0, 7.0);
        let angle = signed_angle(start, end);

        assert_that!(angle).is_none();
    }
}

#[cfg(test)]
mod vector_test {
    use spectral::prelude::*;

    use crate::domain::topology::coord::Position;
    use crate::domain::maths::{Angle, ANGLE_180, ANGLE_90, ANGLE_UP, Vector};

    #[test]
    fn vector_from_angle() {
        let vector = Vector::from_angle(ANGLE_90);

        assert_that!(vector.start.x()).is_equal_to(0.0);
        assert_that!(vector.start.y()).is_equal_to(0.0);
        assert_that!(vector.end.x()).is_close_to(0.0, 0.001);
        assert_that!(vector.end.y()).is_close_to(1.0, 0.001);
    }

    #[test]
    fn vector_length_of_two_points() {
        let vector = Vector::new(Position::new(2.0, 3.4), Position::new(4.2, 5.3));

        let length = vector.length();

        assert_that!(length).is_close_to(2.907, 0.001);
    }

    #[test]
    fn vector_length_is_between_two_point_is_the_same_in_any_order() {
        let point_1 = Position::new(2.3, 4.5);
        let point_2 = Position::new(2.6, 6.5);

        let length_1 = Vector::new(point_1, point_2).length();
        let length_2 = Vector::new(point_2, point_1).length();

        assert_that!(length_1).is_equal_to(length_2);
    }

    #[test]
    fn vector_length_of_same_point_is_0() {
        let point = Position::new(3.0, 4.0);
        let vector = Vector::new(point, point);

        let length = vector.length();

        assert_that!(length).is_equal_to(0.0);
    }

    #[test]
    fn should_calculate_scalar_on_vector_from_origin() {
        let vector1 = Vector::new(Position::new(0.0, 0.0), Position::new(3.2, 4.2));
        let vector2 = Vector::new(Position::new(0.0, 0.0), Position::new(5.6, 6.4));

        let scalar = vector1.scalar(vector2);

        assert_that!(scalar).is_close_to(44.8, 0.001);
    }

    #[test]
    fn should_calculate_scalar_on_vector_not_from_origin() {
        let vector1 = Vector::new(Position::new(2.2, 4.3), Position::new(5.7, 2.5));
        let vector2 = Vector::new(Position::new(2.2, 4.3), Position::new(8.4, 1.9));

        let scalar = vector1.scalar(vector2);

        assert_that!(scalar).is_close_to(26.02, 0.001);
    }

    #[test]
    fn should_calculate_scalar_on_vector_with_different_origin() {
        let vector1 = Vector::new(Position::new(1.2, 2.3), Position::new(4.8, 7.1));
        let vector2 = Vector::new(Position::new(4.3, 6.4), Position::new(9.3, 8.7));

        let scalar = vector1.scalar(vector2);

        assert_that!(scalar).is_close_to(29.04, 0.001);
    }

    #[test]
    fn should_calculate_angle_on_vector_from_origin() {
        let vector1 = Vector::new(Position::new(0.0, 0.0), Position::new(3.2, 4.2));
        let vector2 = Vector::new(Position::new(0.0, 0.0), Position::new(5.6, 6.4));

        let angle = vector1.angle(vector2).map(|angle| angle.to_radiant());

        assert_that!(angle).is_some().is_close_to(0.068, 0.001);
    }

    #[test]
    fn should_calculate_angle_on_vector_not_from_origin() {
        let vector1 = Vector::new(Position::new(2.2, 4.3), Position::new(5.7, 2.5));
        let vector2 = Vector::new(Position::new(2.2, 4.3), Position::new(8.4, 1.9));

        let angle = vector1.angle(vector2).map(|angle| angle.to_radiant());

        assert_that!(angle).is_some().is_close_to(0.106, 0.001);
    }

    #[test]
    fn should_calculate_angle_on_vector_with_different_origin() {
        let vector1 = Vector::new(Position::new(1.2, 2.3), Position::new(4.8, 7.1));
        let vector2 = Vector::new(Position::new(4.3, 6.4), Position::new(9.3, 8.7));

        let angle = vector1.angle(vector2).map(|angle| angle.to_radiant());

        assert_that!(angle).is_some().is_close_to(0.496, 0.001);
    }

    #[test]
    fn should_not_have_angle_if_first_vector_is_a_point() {
        let position = Position::new(1.2, 1.3);
        let vector1 = Vector::new(position, position);
        let vector2 = Vector::new(Position::new(1.0, 1.0), Position::new(2.0, 2.0));

        let angle = vector1.angle(vector2);

        assert_that!(angle).is_none();
    }

    #[test]
    fn should_not_have_angle_if_second_vector_is_a_point() {
        let vector1 = Vector::new(Position::new(1.0, 1.0), Position::new(2.0, 2.0));
        let position = Position::new(1.2, 1.3);
        let vector2 = Vector::new(position, position);

        let angle = vector1.angle(vector2);

        assert_that!(angle).is_none();
    }

    #[test]
    fn angle_sign_should_be_opposite() {
        let vector1 = Vector::new(Position::new(1.0, 1.0), Position::new(2.0, 2.0));
        let vector2 = Vector::new(Position::new(-1.0, 4.0), Position::new(6.0, 3.0));

        let sign1 = vector1.angle_sign_is_negative(vector2);
        let sign2 = vector2.angle_sign_is_negative(vector1);

        assert_that!(sign1).is_equal_to(!sign2);
    }

    #[test]
    fn should_calculate_angle_sign() {
        let vector1 = Vector::new(Position::new(1.0, 1.0), Position::new(2.0, 2.0));
        let vector2 = Vector::new(Position::new(-1.0, 4.0), Position::new(6.0, 3.0));

        let sign = vector1.angle_sign_is_negative(vector2);

        assert_that!(sign).is_false();
    }

    #[test]
    fn spherical_cone_should_create_asked_number_of_angle() {
        let angles = ANGLE_UP.discreet_cone_spherical_space(ANGLE_90, 5);
        assert_that!(angles).has_length(5);
    }

    #[test]
    fn spherical_cone_should_have_uniform_angle_repartition() {
        let angles = ANGLE_UP.discreet_cone_spherical_space(ANGLE_180, 5);

        let space_1 = angles[0].to_radiant() - angles[1].to_radiant();
        let space_2 = angles[1].to_radiant() - angles[2].to_radiant();
        let space_3 = angles[2].to_radiant() - angles[3].to_radiant();
        let space_4 = angles[3].to_radiant() - angles[4].to_radiant();

        assert_that!(space_1).is_close_to(space_2, 0.0001);
        assert_that!(space_1).is_close_to(space_3, 0.0001);
        assert_that!(space_1).is_close_to(space_4, 0.0001);
    }

    #[test]
    fn spherical_cone_should_have_center_angle_as_base_angle() {
        let base_angle = Angle::new(2.6);

        let angles = base_angle.discreet_cone_spherical_space(ANGLE_180, 5);

        assert_that!(angles[2].to_radiant()).is_close_to(base_angle.to_radiant(), 0.01);
    }

    #[test]
    fn spherical_cone_should_have_extreme_angle_difference_equals_to_cone_angle_asked() {
        let cone_angle = Angle::new(1.8);
        let angles = ANGLE_UP.discreet_cone_spherical_space(cone_angle, 5);

        let space = angles[0].to_radiant() - angles[4].to_radiant();

        assert_that!(space).is_close_to(cone_angle.to_radiant(), 0.01);
    }

    #[test]
    fn straight_cone_should_create_asked_number_of_angle() {
        let angles = ANGLE_UP.discreet_cone_straight_space(ANGLE_90, 5);
        assert_that!(angles).has_length(5);
    }

    #[test]
    fn straight_cone_should_have_center_angle_as_base_angle() {
        let base_angle = Angle::new(2.6);

        let angles = base_angle.discreet_cone_straight_space(ANGLE_180, 5);

        assert_that!(angles[2].to_radiant()).is_close_to(base_angle.to_radiant(), 0.01);
    }

    #[test]
    fn straight_cone_should_have_extreme_angle_difference_equals_to_cone_angle_asked() {
        let cone_angle = Angle::new(1.8);
        let angles = ANGLE_UP.discreet_cone_straight_space(cone_angle, 5);

        let space = angles[0].to_radiant() - angles[4].to_radiant();

        assert_that!(space).is_close_to(cone_angle.to_radiant(), 0.01);
    }

    #[test]
    fn straight_cone_should_have_uniform_distance_repartition() {
        let angles = ANGLE_UP.discreet_cone_straight_space(ANGLE_90, 5);

        let project_on_y: Vec<Position> = angles.iter()
            .map(|angle| Position::new(angle.cos() / angle.sin(), angle.sin() / angle.sin()))
            .collect();

        let distance_1 = project_on_y[0].distance(&project_on_y[1]);
        let distance_2 = project_on_y[1].distance(&project_on_y[2]);
        let distance_3 = project_on_y[2].distance(&project_on_y[3]);
        let distance_4 = project_on_y[3].distance(&project_on_y[4]);

        assert_that!(distance_1).is_close_to(distance_2, 0.0001);
        assert_that!(distance_1).is_close_to(distance_3, 0.0001);
        assert_that!(distance_1).is_close_to(distance_4, 0.0001);
    }
}
