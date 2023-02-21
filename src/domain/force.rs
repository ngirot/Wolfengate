use std::f32::consts::PI;

#[derive(Copy, Clone)]
pub struct Force {
    orientation: f32,
    power: f32,
    rotation: f32,
}

#[derive(Copy, Clone)]
pub struct InputForce {
    movement_speed: f32,
    rotation_speed: f32,
}

impl Force {
    pub fn new(orientation: f32, power: f32, rotation: f32) -> Self {
        Force { orientation, power, rotation }
    }

    pub fn orientation(&self) -> f32 {
        self.orientation
    }

    pub fn power(&self) -> f32 {
        self.power
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn power_multipyer(&self, factor: f32) -> Self {
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
            orientation: y3.atan2(x3),
            power: ((x3 * x3) + (y3 * y3)).sqrt(),
            rotation: self.rotation + force.rotation,
        }
    }

    pub fn for_relative_view(&self, orientation: f32) -> Self {
        Force {
            orientation: self.orientation + orientation,
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
        self.movement_to_force(0.0)
    }

    pub fn backward(&self) -> Force {
        self.movement_to_force(PI)
    }

    pub fn strafe_left(&self) -> Force {
        self.movement_to_force(PI / 2.0)
    }

    pub fn state_right(&self) -> Force {
        self.movement_to_force(-PI / 2.0)
    }

    pub fn rotate(&self, amplitude: i32) -> Force {
        self.rotation_to_force(amplitude)
    }

    fn movement_to_force(&self, orientation: f32) -> Force {
        Force::new(orientation, self.movement_speed, 0.0)
    }

    fn rotation_to_force(&self, amplitude: i32) -> Force {
        let angle = self.rotation_speed * -amplitude as f32;
        Force::new(0.0, 0.0, angle)
    }
}

#[cfg(test)]
mod input_force_test {
    use std::f32::consts::PI;

    use spectral::prelude::*;

    use crate::domain::force::{Force, InputForce};

    #[test]
    fn move_force_should_not_have_a_rotation() {
        let stats = InputForce::new(1.2, 2.3);
        let force = stats.movement_to_force(1.2);
        assert_that(&force.rotation()).is_equal_to(0.0);
    }

    #[test]
    fn move_force_should_have_the_move_orientation() {
        let stats = InputForce::new(1.2, 2.3);
        let force = stats.movement_to_force(1.2);

        assert_that(&force.orientation()).is_equal_to(1.2);
    }

    #[test]
    fn move_force_should_have_the_a_power_calculated_from_speed() {
        let stats = InputForce::new(1.2, 2.3);
        let force = stats.movement_to_force(1.2);

        assert_that(&force.power()).is_equal_to(1.2);
    }

    #[test]
    fn rotation_force_should_have_a_rotation_calculated_from_rotation_speed() {
        let stats = InputForce::new(2.4, 2.7);
        let force = stats.rotation_to_force(4);

        assert_that(&force.rotation()).is_equal_to(-10.8);
    }

    #[test]
    fn rotation_force_should_have_no_orientation() {
        let stats = InputForce::new(2.4, 6.3);
        let force = stats.rotation_to_force(12);

        assert_that(&force.orientation()).is_equal_to(0.0);
    }

    #[test]
    fn rotation_force_should_have_no_power() {
        let stats = InputForce::new(2.4, 6.3);
        let force = stats.rotation_to_force(12);

        assert_that(&force.power()).is_equal_to(0.0);
    }

    #[test]
    fn force_should_add_power_and_orientation() {
        let force1 = Force::new(0.0, 1.0, 0.0);
        let force2 = Force::new(PI / 2.0, 1.5, 0.0);

        let added = force1.add(force2);

        assert_that(&added.orientation()).is_close_to(0.982, 0.001);
        assert_that(&added.power()).is_close_to(1.803, 0.001);
    }

    #[test]
    fn force_should_add_rotation() {
        let force1 = Force::new(0.0, 0.0, 1.0);
        let force2 = Force::new(0.0, 0.0, -3.0);

        let added = force1.add(force2);

        assert_that(&added.rotation()).is_equal_to(-2.0);
    }

    #[test]
    fn force_should_add_zero() {
        let force = Force::new(0.0, 0.0, 0.0);

        let added = force.add(force);

        assert_that(&added.power()).is_equal_to(0.0);
        assert_that(&added.orientation()).is_equal_to(0.0);
        assert_that(&added.rotation()).is_equal_to(0.0);
    }

    #[test]
    fn should_increase_force_by_factor() {
        let force = Force::new(0.0, 10.0, 0.0);

        let reduced = force.power_multipyer(2.0);

        assert_that(&reduced.power()).is_equal_to(20.0);
    }

    #[test]
    fn should_reduce_force_by_factor() {
        let force = Force::new(0.0, 10.0, 0.0);

        let reduced = force.power_multipyer(0.5);

        assert_that(&reduced.power()).is_equal_to(5.0);
    }
}
