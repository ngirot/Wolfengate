use crate::domain::actor::SpeedStats;
use crate::domain::maths::{Angle, Move};

#[derive(Copy, Clone)]
pub struct Acceleration {
    orientation: Angle,
    units_per_seconds_square: f32,
}

#[derive(Copy, Clone)]
pub struct Speed {
    orientation: Angle,
    units_per_seconds: f32,
}

impl Speed {
    pub fn new(orientation: Angle, units_per_seconds: f32) -> Self {
        Self {
            orientation,
            units_per_seconds,
        }
    }

    pub fn to_move(&self, microseconds_elapsed: u128) -> Move {
        Move::new(
            self.orientation,
            microseconds_elapsed as f32 / 1000000.0 * self.units_per_seconds as f32,
        )
    }

    pub fn reduce(&self, reduction: SpeedStats) -> Self {
        Self {
            orientation: self.orientation,
            units_per_seconds: self.units_per_seconds - reduction.units_per_seconds(),
        }
    }

    pub fn with_min_speed(&self, min_speed: SpeedStats) -> Self {
        if self.units_per_seconds <= min_speed.units_per_seconds() {
            Self {
                units_per_seconds: min_speed.units_per_seconds(),
                orientation: self.orientation,
            }
        } else {
            *self
        }
    }

    pub fn with_max_speed(&self, max_speed: SpeedStats) -> Self {
        if self.units_per_seconds >= max_speed.units_per_seconds() {
            Self {
                units_per_seconds: max_speed.units_per_seconds(),
                orientation: self.orientation,
            }
        } else {
            *self
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
            orientation: Angle::new(y3.atan2(x3)),
            units_per_seconds: ((x3 * x3) + (y3 * y3)).sqrt(),
        }
    }

    pub fn rotate(&self, rotation: Angle) -> Self {
        Self {
            orientation: self.orientation.add(rotation),
            units_per_seconds: self.units_per_seconds,
        }
    }

    pub fn orientation(&self) -> Angle {
        self.orientation
    }

    pub fn units_per_seconds(&self) -> f32 {
        self.units_per_seconds
    }
}

impl Acceleration {
    pub fn new(orientation: Angle, units_per_seconds_square: f32) -> Self {
        Self {
            orientation,
            units_per_seconds_square,
        }
    }

    pub fn to_speed(&self, microseconds_elapsed: u128) -> Speed {
        Speed::new(
            self.orientation,
            microseconds_elapsed as f32 / 1000000.0 * self.units_per_seconds_square,
        )
    }
}

#[cfg(test)]
mod fn_speed_stats {
    use spectral::prelude::*;

    use crate::domain::actor::SpeedStats;
    use crate::domain::maths::ANGLE_RIGHT;
    use crate::domain::physics::Speed;

    #[test]
    fn should_reduce_speed_if_bigger_than_max() {
        let current = Speed::new(ANGLE_RIGHT, 100.0);
        let max = SpeedStats::new(50.0);

        let result = current.with_max_speed(max);

        assert_that!(result.units_per_seconds()).is_equal_to(50.0);
    }

    #[test]
    fn should_keep_speed_if_lower_than_max() {
        let current = Speed::new(ANGLE_RIGHT, 100.0);
        let max = SpeedStats::new(150.0);

        let result = current.with_max_speed(max);

        assert_that!(result.units_per_seconds()).is_equal_to(100.0);
    }

    #[test]
    fn should_increase_speed_if_lower_than_min() {
        let current = Speed::new(ANGLE_RIGHT, -10.0);
        let max = SpeedStats::new(0.0);

        let result = current.with_min_speed(max);

        assert_that!(result.units_per_seconds()).is_equal_to(0.0);
    }

    #[test]
    fn should_keep_speed_if_bigger_than_min() {
        let current = Speed::new(ANGLE_RIGHT, 10.0);
        let max = SpeedStats::new(5.0);

        let result = current.with_min_speed(max);

        assert_that!(result.units_per_seconds()).is_equal_to(10.0);
    }
}