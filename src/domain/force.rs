use crate::domain::maths::{Angle, ANGLE_DOWN, ANGLE_LEFT, ANGLE_RIGHT, ANGLE_UP};

#[derive(Copy, Clone)]
pub struct Force {
    orientation: Angle,
    power: f32,
    rotation: Angle,
}

#[derive(Copy, Clone)]
pub struct InputForce {
    movement_speed: f32,
    rotation_speed: f32,
}

impl Force {
    pub fn new(orientation: Angle, power: f32, rotation: Angle) -> Self {
        Self {
            orientation,
            power,
            rotation,
        }
    }

    pub fn orientation(&self) -> Angle {
        self.orientation
    }

    pub fn power(&self) -> f32 {
        self.power
    }

    pub fn rotation(&self) -> Angle {
        self.rotation
    }

    pub fn power_multiplier(&self, factor: f32) -> Self {
        Self {
            power: self.power * factor,
            rotation: self.rotation,
            orientation: self.orientation,
        }
    }

    pub fn add(&self, force: Force) -> Force {
        let x1 = self.power() * self.orientation().cos();
        let y1 = self.power() * self.orientation().sin();

        let x2 = force.power() * force.orientation().cos();
        let y2 = force.power() * force.orientation().sin();

        let x3 = x1 + x2;
        let y3 = y1 + y2;

        Self {
            orientation: Angle::new(y3.atan2(x3)),
            power: ((x3 * x3) + (y3 * y3)).sqrt(),
            rotation: self.rotation.add(force.rotation),
        }
    }

    pub fn for_relative_view(&self, orientation: Angle) -> Self {
        Force {
            orientation: self.orientation.add(orientation),
            power: self.power,
            rotation: self.rotation,
        }
    }
}

impl InputForce {
    pub fn new(movement_speed: f32, rotation_speed: f32) -> Self {
        Self {
            movement_speed,
            rotation_speed,
        }
    }

    pub fn forward(&self) -> Force {
        self.movement_to_force(ANGLE_RIGHT)
    }

    pub fn backward(&self) -> Force {
        self.movement_to_force(ANGLE_LEFT)
    }

    pub fn strafe_left(&self) -> Force {
        self.movement_to_force(ANGLE_UP)
    }

    pub fn state_right(&self) -> Force {
        self.movement_to_force(ANGLE_DOWN)
    }

    pub fn rotate(&self, amplitude: i32) -> Force {
        self.rotation_to_force(amplitude)
    }

    fn movement_to_force(&self, orientation: Angle) -> Force {
        Force::new(orientation, self.movement_speed, ANGLE_RIGHT)
    }

    fn rotation_to_force(&self, amplitude: i32) -> Force {
        let angle = Angle::new(self.rotation_speed * -amplitude as f32);
        Force::new(ANGLE_RIGHT, 0.0, angle)
    }
}

#[cfg(test)]
mod input_force_test {
    use std::f32::consts::PI;

    use spectral::prelude::*;

    use crate::domain::force::{Force, InputForce};
    use crate::domain::maths::{Angle, ANGLE_RIGHT};

    #[test]
    fn move_force_should_not_have_a_rotation() {
        let stats = InputForce::new(1.2, 2.3);
        let force = stats.movement_to_force(Angle::new(1.2));
        assert_that!(force.rotation().to_radiant()).is_equal_to(0.0);
    }

    #[test]
    fn move_force_should_have_the_move_orientation() {
        let stats = InputForce::new(1.2, 2.3);
        let force = stats.movement_to_force(Angle::new(1.2));

        assert_that!(force.orientation().to_radiant()).is_equal_to(1.2);
    }

    #[test]
    fn move_force_should_have_the_a_power_calculated_from_speed() {
        let stats = InputForce::new(1.2, 2.3);
        let force = stats.movement_to_force(Angle::new(1.2));

        assert_that!(force.power()).is_equal_to(1.2);
    }

    #[test]
    fn rotation_force_should_have_a_rotation_calculated_from_rotation_speed() {
        let stats = InputForce::new(2.4, 2.7);
        let force = stats.rotation_to_force(4);

        assert_that!(force.rotation().to_radiant()).is_close_to(-4.516, 0.001);
    }

    #[test]
    fn rotation_force_should_have_no_orientation() {
        let stats = InputForce::new(2.4, 6.3);
        let force = stats.rotation_to_force(12);

        assert_that!(force.orientation().to_radiant()).is_equal_to(0.0);
    }

    #[test]
    fn rotation_force_should_have_no_power() {
        let stats = InputForce::new(2.4, 6.3);
        let force = stats.rotation_to_force(12);

        assert_that!(force.power()).is_equal_to(0.0);
    }

    #[test]
    fn force_should_add_power_and_orientation() {
        let force1 = Force::new(ANGLE_RIGHT, 1.0, ANGLE_RIGHT);
        let force2 = Force::new(Angle::new(PI / 2.0), 1.5, ANGLE_RIGHT);

        let added = force1.add(force2);

        assert_that!(added.orientation().to_radiant()).is_close_to(0.982, 0.001);
        assert_that!(added.power()).is_close_to(1.803, 0.001);
    }

    #[test]
    fn force_should_add_rotation() {
        let force1 = Force::new(ANGLE_RIGHT, 0.0, Angle::new(1.0));
        let force2 = Force::new(ANGLE_RIGHT, 0.0, Angle::new(-3.0));

        let added = force1.add(force2);

        assert_that!(added.rotation().to_radiant()).is_equal_to(-2.0);
    }

    #[test]
    fn force_should_add_zero() {
        let force = Force::new(ANGLE_RIGHT, 0.0, ANGLE_RIGHT);

        let added = force.add(force);

        assert_that!(added.power()).is_equal_to(0.0);
        assert_that!(added.orientation().to_radiant()).is_equal_to(0.0);
        assert_that!(added.rotation().to_radiant()).is_equal_to(0.0);
    }

    #[test]
    fn should_increase_force_by_factor() {
        let force = Force::new(ANGLE_RIGHT, 10.0, ANGLE_RIGHT);

        let reduced = force.power_multiplier(2.0);

        assert_that!(reduced.power()).is_equal_to(20.0);
    }

    #[test]
    fn should_reduce_force_by_factor() {
        let force = Force::new(ANGLE_RIGHT, 10.0, ANGLE_RIGHT);

        let reduced = force.power_multiplier(0.5);

        assert_that!(reduced.power()).is_equal_to(5.0);
    }
}
