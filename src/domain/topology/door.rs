pub trait Openable {
    fn door_column(&self, offset: f32) -> Option<f32>;
}

pub struct CentralDoor {
    opening_percentage: f32,
}

pub struct LateralDoor {
    opening_percentage: f32,
}

impl CentralDoor {
    pub fn new(opening_percentage: f32) -> Self {
        Self {
            opening_percentage,
        }
    }
}

impl LateralDoor {
    pub fn new(opening_percentage: f32) -> Self {
        Self {
            opening_percentage,
        }
    }
}

impl Openable for CentralDoor {
    fn door_column(&self, offset: f32) -> Option<f32> {
        let closing_percentage = 1.0 - self.opening_percentage;

        let new_right = 1.0 - (closing_percentage / 2.0);
        let new_left = 0.0 + (closing_percentage / 2.0);

        if offset > new_left && offset < new_right {
            return None;
        }

        let position_on_texture;
        if offset <= new_left {
            position_on_texture = offset + self.opening_percentage / 2.0;
        } else {
            position_on_texture = offset - self.opening_percentage / 2.0;
        }

        Some(position_on_texture)
    }
}

impl Openable for LateralDoor {
    fn door_column(&self, offset: f32) -> Option<f32> {
        if offset > 1.0 - self.opening_percentage || offset < 0.0 {
            return None;
        }

        Some(offset + self.opening_percentage)
    }
}

#[cfg(test)]
mod openable_test {
    use rand::Rng;
    use spectral::prelude::*;

    use crate::domain::topology::door::{CentralDoor, LateralDoor, Openable};

    fn test_on_range(f: impl Fn(f32)) {
        let mut rng = rand::thread_rng();
        for _i in 0..10_000 {
            f(rng.gen());
        }
    }

    #[test]
    fn lateral_door_full_closed_should_return_texture_offset_equals_to_position_offset() {
        let door = LateralDoor::new(0.0);

        test_on_range(|offset| {
            let texture = door.door_column(offset);
            assert_that!(texture).is_some().is_equal_to(offset);
        });
    }

    #[test]
    fn lateral_door_full_opened_should_return_no_texture_offset() {
        let door = LateralDoor::new(1.0);


        test_on_range(|offset| {
            let texture = door.door_column(offset);
            assert_that!(texture).is_none();
        });
    }

    #[test]
    fn lateral_door_partially_opened_should_get_texture_of_visible_part() {
        let door = LateralDoor::new(0.25);

        let texture = door.door_column(0.1);
        assert_that!(texture).is_some().is_equal_to(0.35);
    }

    #[test]
    fn lateral_door_partially_opened_should_get_not_texture_of_invisible_part() {
        let door = LateralDoor::new(0.25);

        let texture = door.door_column(0.76);
        assert_that!(texture).is_none();
    }

    /////
    #[test]
    fn central_door_full_closed_should_return_texture_offset_equals_to_position_offset() {
        let door = CentralDoor::new(0.0);

        test_on_range(|offset| {
            let texture = door.door_column(offset);
            assert_that!(texture).is_some().is_equal_to(offset);
        });
    }

    #[test]
    fn central_door_full_opened_should_return_no_texture_offset() {
        let door = CentralDoor::new(1.0);


        test_on_range(|offset| {
            let texture = door.door_column(offset);
            assert_that!(texture).is_none();
        });
    }

    #[test]
    fn central_door_partially_opened_should_get_texture_of_visible_part_on_left() {
        let door = CentralDoor::new(0.25);

        let texture = door.door_column(0.1);
        assert_that!(texture).is_some().is_equal_to(0.225);
    }

    #[test]
    fn central_door_partially_opened_should_get_not_texture_of_invisible_part_on_left() {
        let door = CentralDoor::new(0.25);

        let texture = door.door_column(0.45);
        assert_that!(texture).is_none();
    }

    #[test]
    fn central_door_partially_opened_should_get_texture_of_visible_part_on_right() {
        let door = CentralDoor::new(0.25);

        let texture = door.door_column(0.9);
        assert_that!(texture).is_some().is_equal_to(0.775);
    }

    #[test]
    fn central_door_partially_opened_should_get_not_texture_of_invisible_part_on_right() {
        let door = CentralDoor::new(0.25);

        let texture = door.door_column(0.55);
        assert_that!(texture).is_none();
    }
}

