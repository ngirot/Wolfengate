#[derive(Copy, Clone)]
pub struct ScreenPoint {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
pub struct MapPoint {
    x: u8,
    y: u8,
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
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> u8 {
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
}

#[cfg(test)]
mod coord_test {
    use super::Position;
    use spectral::prelude::*;

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
    fn distance_shoul_dwork_with_nefative_numbers() {
        let position1 = Position::new(-1.0, 3.0);
        let position2 = Position::new(8.1, -7.4);

        let distance = position1.distance(&position2);

        assert_that(&distance).is_close_to(13.819, 0.001);
    }
}
