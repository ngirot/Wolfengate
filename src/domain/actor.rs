use crate::domain::force::Force;

use super::coord::Position;

#[derive(Copy, Clone)]
pub struct ActorStats {
    movement_speed: f32,
    rotation_speed: f32,
}

#[derive(Copy, Clone)]
pub struct Player {
    position: Position,
    orientation: f32,
}

impl Player {
    pub fn new(position: Position, orientation: f32) -> Self {
        Self {
            position,
            orientation,
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn orientation(&self) -> f32 {
        self.orientation
    }

    pub fn apply_force(&self, force: Force) -> Self {
        self.move_direction(force)
            .rotate(force.rotation())
    }

    fn rotate(&self, angle: f32) -> Self {
        let new_orientation = self.orientation + angle;
        Self {
            position: self.position,
            orientation: new_orientation,
        }
    }


    fn move_direction(&self, force: Force) -> Player {
        let new_position = self.position.apply_force(force);
        Self {
            position: new_position,
            orientation: self.orientation,
        }
    }
}

impl ActorStats {
    pub fn new(movement_speed: f32, rotation_speed: f32) -> Self {
        Self {
            movement_speed,
            rotation_speed,
        }
    }

    pub fn movement_to_force(&self, orientation: f32, milliseconds_elapsed: u128) -> Force {
        let factor = self.movement_speed * milliseconds_elapsed as f32;
        Force::new(orientation, factor, 0.0)
    }

    pub fn rotation_to_force(&self, amplitude: i32) -> Force {
        let angle = self.rotation_speed * -amplitude as f32;
        Force::new(0.0, 0.0, angle)
    }
}

#[cfg(test)]
mod actor_test {
    use spectral::prelude::*;

    use crate::domain::coord::Position;
    use crate::domain::force::Force;

    use super::Player;

    #[test]
    fn should_apply_force() {
        let player = Player::new(
            Position::new(1.0, 2.0),
            1.05,
        );
        let after_move = player.apply_force(Force::new(1.43, 2.3, 0.243));
        assert_that(&after_move.position().x()).is_close_to(1.322, 0.001);
        assert_that(&after_move.position().y()).is_close_to(4.277, 0.001);
        assert_that(&after_move.orientation()).is_close_to(1.293, 0.001);
    }
}

#[cfg(test)]
mod actor_stats_test {
    use spectral::prelude::*;

    use crate::domain::actor::ActorStats;

    #[test]
    fn move_force_should_not_have_a_rotation() {
        let stats = ActorStats::new(1.2, 2.3);
        let force = stats.movement_to_force(1.2, 120);
        assert_that(&force.rotation()).is_equal_to(0.0);
    }

    #[test]
    fn move_force_should_have_the_move_orientation() {
        let stats = ActorStats::new(1.2, 2.3);
        let force = stats.movement_to_force(1.2, 120);

        assert_that(&force.orientation()).is_equal_to(1.2);
    }

    #[test]
    fn move_force_should_have_the_a_power_calculated_from_speed() {
        let stats = ActorStats::new(1.2, 2.3);
        let force = stats.movement_to_force(1.2, 120);

        assert_that(&force.power()).is_equal_to(144.0);
    }

    #[test]
    fn rotation_force_should_have_a_rotation_calculated_from_rotation_speed() {
        let stats = ActorStats::new(2.4, 2.7);
        let force = stats.rotation_to_force(4);

        assert_that(&force.rotation()).is_equal_to(-10.8);
    }

    #[test]
    fn rotation_force_should_have_no_orientation() {
        let stats = ActorStats::new(2.4, 6.3);
        let force = stats.rotation_to_force(12);

        assert_that(&force.orientation()).is_equal_to(0.0);
    }

    #[test]
    fn rotation_force_should_have_no_power() {
        let stats = ActorStats::new(2.4, 6.3);
        let force = stats.rotation_to_force(12);

        assert_that(&force.power()).is_equal_to(0.0);
    }
}
