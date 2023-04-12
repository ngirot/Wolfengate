use dyn_clone::DynClone;

pub trait Openable: DynClone + Sync {
    fn door_column(&self, opening_percentage: f32, offset: f32) -> Option<f32>;
}
dyn_clone::clone_trait_object!(Openable);

#[derive(Clone)]
pub struct CentralOpening {}

#[derive(Clone)]
pub struct LateralOpening {}

impl CentralOpening {
    pub fn new() -> Self {
        Self {}
    }
}

impl LateralOpening {
    pub fn new() -> Self {
        Self {}
    }
}

impl Openable for CentralOpening {
    fn door_column(&self, opening_percentage: f32, offset: f32) -> Option<f32> {
        let closing_percentage = 1.0 - opening_percentage;

        let new_right = 1.0 - (closing_percentage / 2.0);
        let new_left = 0.0 + (closing_percentage / 2.0);

        if offset > new_left && offset < new_right {
            return None;
        }

        if offset <= new_left {
            Some(offset + opening_percentage / 2.0)
        } else {
            Some(offset - opening_percentage / 2.0)
        }
    }
}

impl Openable for LateralOpening {
    fn door_column(&self, opening_percentage: f32, offset: f32) -> Option<f32> {
        if offset > 1.0 - opening_percentage || offset < 0.0 {
            return None;
        }

        Some(offset + opening_percentage)
    }
}

#[cfg(test)]
mod openable_test {
    use rand::Rng;
    use spectral::prelude::*;

    use crate::domain::topology::door::{CentralOpening, LateralOpening, Openable};

    fn test_on_range(f: impl Fn(f32)) {
        let mut rng = rand::thread_rng();
        for _i in 0..10_000 {
            f(rng.gen());
        }
    }

    #[test]
    fn lateral_door_full_closed_should_return_texture_offset_equals_to_position_offset() {
        let door = LateralOpening::new();

        test_on_range(|offset| {
            let texture = door.door_column(0.0, offset);
            assert_that!(texture).is_some().is_equal_to(offset);
        });
    }

    #[test]
    fn lateral_door_full_opened_should_return_no_texture_offset() {
        let door = LateralOpening::new();


        test_on_range(|offset| {
            let texture = door.door_column(1.0, offset);
            assert_that!(texture).is_none();
        });
    }

    #[test]
    fn lateral_door_partially_opened_should_get_texture_of_visible_part() {
        let door = LateralOpening::new();

        let texture = door.door_column(0.25, 0.1);
        assert_that!(texture).is_some().is_equal_to(0.35);
    }

    #[test]
    fn lateral_door_partially_opened_should_get_not_texture_of_invisible_part() {
        let door = LateralOpening::new();

        let texture = door.door_column(0.25, 0.76);
        assert_that!(texture).is_none();
    }

    #[test]
    fn central_door_full_closed_should_return_texture_offset_equals_to_position_offset() {
        let door = CentralOpening::new();

        test_on_range(|offset| {
            let texture = door.door_column(0.0, offset);
            assert_that!(texture).is_some().is_equal_to(offset);
        });
    }

    #[test]
    fn central_door_full_opened_should_return_no_texture_offset() {
        let door = CentralOpening::new();


        test_on_range(|offset| {
            let texture = door.door_column(1.0, offset);
            assert_that!(texture).is_none();
        });
    }

    #[test]
    fn central_door_partially_opened_should_get_texture_of_visible_part_on_left() {
        let door = CentralOpening::new();

        let texture = door.door_column(0.25, 0.1);
        assert_that!(texture).is_some().is_equal_to(0.225);
    }

    #[test]
    fn central_door_partially_opened_should_get_not_texture_of_invisible_part_on_left() {
        let door = CentralOpening::new();

        let texture = door.door_column(0.25, 0.45);
        assert_that!(texture).is_none();
    }

    #[test]
    fn central_door_partially_opened_should_get_texture_of_visible_part_on_right() {
        let door = CentralOpening::new();

        let texture = door.door_column(0.25, 0.9);
        assert_that!(texture).is_some().is_equal_to(0.775);
    }

    #[test]
    fn central_door_partially_opened_should_get_not_texture_of_invisible_part_on_right() {
        let door = CentralOpening::new();

        let texture = door.door_column(0.25, 0.55);
        assert_that!(texture).is_none();
    }
}

