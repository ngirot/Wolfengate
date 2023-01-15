use crate::domain::force::Force;

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
        ((self.x - position.x).abs().powi(2) + (self.y - position.y).abs().powi(2)).sqrt()
    }

    pub fn apply_force(&self, force: Force) -> Position {
        let factor = force.power();
        let new_angle = force.orientation();

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

#[cfg(test)]
mod coord_test {
    use std::f32::consts::PI;

    use spectral::prelude::*;
    use crate::domain::force::Force;

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
        let force = Force::new(0.0, 4.0, 0.0);

        let applied = position.apply_force(force);

        assert_that(&applied.x()).is_close_to(9.0, 0.0);
        assert_that(&applied.y()).is_close_to(10.0, 0.0);
    }

    #[test]
    fn apply_simple_y_force() {
        let position = Position::new(5.0, 10.0);
        let force = Force::new(PI / 2.0, 4.0, 0.0);

        let applied = position.apply_force(force);

        assert_that(&applied.x()).is_close_to(5.0, 0.0);
        assert_that(&applied.y()).is_close_to(14.0, 0.0);
    }

    #[test]
    fn apply_force_with_angle() {
        let position = Position::new(3.1, 6.4);
        let force = Force::new(1.2, 3.2, 0.0);

        let applied = position.apply_force(force);

        assert_that(&applied.x()).is_close_to(4.259, 0.001);
        assert_that(&applied.y()).is_close_to(9.382, 0.001);
    }
}
