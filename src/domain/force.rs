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

    pub fn forward(&self, milliseconds_elapsed: u128) -> Force {
        self.movement_to_force(0.0, milliseconds_elapsed)
    }

    pub fn backward(&self, milliseconds_elapsed: u128) -> Force {
        self.movement_to_force(PI, milliseconds_elapsed)
    }

    pub fn strafe_left(&self, milliseconds_elapsed: u128) -> Force {
        self.movement_to_force(PI / 2.0, milliseconds_elapsed)
    }

    pub fn state_right(&self, milliseconds_elapsed: u128) -> Force {
        self.movement_to_force(-PI / 2.0, milliseconds_elapsed)
    }

    pub fn rotate(&self, amplitude: i32) -> Force {
        self.rotation_to_force(amplitude)
    }

    fn movement_to_force(&self, orientation: f32, milliseconds_elapsed: u128) -> Force {
        let factor = self.movement_speed * milliseconds_elapsed as f32;
        Force::new(orientation, factor, 0.0)
    }

    fn rotation_to_force(&self, amplitude: i32) -> Force {
        let angle = self.rotation_speed * -amplitude as f32;
        Force::new(0.0, 0.0, angle)
    }
}


#[cfg(test)]
mod input_force_test {
    use spectral::prelude::*;

    use crate::domain::force::InputForce;

    #[test]
    fn move_force_should_not_have_a_rotation() {
        let stats = InputForce::new(1.2, 2.3);
        let force = stats.movement_to_force(1.2, 120);
        assert_that(&force.rotation()).is_equal_to(0.0);
    }

    #[test]
    fn move_force_should_have_the_move_orientation() {
        let stats = InputForce::new(1.2, 2.3);
        let force = stats.movement_to_force(1.2, 120);

        assert_that(&force.orientation()).is_equal_to(1.2);
    }

    #[test]
    fn move_force_should_have_the_a_power_calculated_from_speed() {
        let stats = InputForce::new(1.2, 2.3);
        let force = stats.movement_to_force(1.2, 120);

        assert_that(&force.power()).is_equal_to(144.0);
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
}
