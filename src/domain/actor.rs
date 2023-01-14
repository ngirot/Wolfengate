use std::f32::consts::PI;
use crate::domain::coord::Force;

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
    stats: ActorStats,
}

impl Player {
    pub fn new(position: Position, orientation: f32, stats: ActorStats) -> Self {
        Self {
            position,
            orientation,
            stats,
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn orientation(&self) -> f32 {
        self.orientation
    }

    pub fn move_forward(&self, milliseconds_elapsed: u128) -> Self {
        self.move_direction(self.orientation, milliseconds_elapsed)
    }

    pub fn move_backward(&self, milliseconds_elapsed: u128) -> Self {
        let direction = self.orientation + PI;
        self.move_direction(direction, milliseconds_elapsed)
    }

    pub fn move_left(&self, milliseconds_elapsed: u128) -> Self {
        let direction = self.orientation + (PI / 2.0);
        self.move_direction(direction, milliseconds_elapsed)
    }

    pub fn move_right(&self, milliseconds_elapsed: u128) -> Self {
        let direction = self.orientation - (PI / 2.0);
        self.move_direction(direction, milliseconds_elapsed)
    }

    pub fn rotate(&self, amplitude: i32) -> Self {
        let angle = self.stats.rotation_speed * -amplitude as f32;
        let new_orientation = self.orientation + angle;
        Self {
            position: self.position,
            orientation: new_orientation,
            stats: self.stats,
        }
    }

    fn get_speed_factor(&self, milliseconds_elapsed: u128) -> f32 {
        self.stats.movement_speed * milliseconds_elapsed as f32
    }

    fn move_direction(&self, orientation: f32, milliseconds_elapsed: u128) -> Player {
        let factor = self.get_speed_factor(milliseconds_elapsed);
        let force = Force::new(orientation, factor);

        let new_position = self.position.apply_force(force);
        Self {
            position: new_position,
            orientation: self.orientation,
            stats: self.stats,
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
}

#[cfg(test)]
mod actor_test {
    use spectral::prelude::*;

    use crate::domain::coord::Position;

    use super::Player;

    #[test]
    fn should_move_forward() {
        let player = Player::new(
            Position::new(1.0, 2.0),
            1.05,
            super::ActorStats::new(1.2, 1.0),
        );
        let after_move = player.move_forward(20);
        assert_that(&after_move.position().x()).is_close_to(12.941, 0.001);
        assert_that(&after_move.position().y()).is_close_to(22.818, 0.001);
    }

    #[test]
    fn should_move_backward() {
        let player = Player::new(
            Position::new(1.0, 2.0),
            1.05,
            super::ActorStats::new(1.0, 1.0),
        );
        let after_move = player.move_backward(27);
        assert_that(&after_move.position().x()).is_close_to(-12.434, 0.001);
        assert_that(&after_move.position().y()).is_close_to(-21.420, 0.001);
    }

    #[test]
    fn should_rotate_left() {
        let player = Player::new(
            Position::new(1.0, 2.0),
            1.05,
            super::ActorStats::new(1.0, 0.3),
        );
        let after_move = player.rotate(12);
        assert_that(&after_move.orientation).is_close_to(-2.55, 0.001);
    }

    #[test]
    fn should_rotate_right() {
        let player = Player::new(
            Position::new(1.0, 2.0),
            1.05,
            super::ActorStats::new(1.0, 0.3),
        );
        let after_move = player.rotate(-12);
        assert_that(&after_move.orientation).is_close_to(4.65, 0.001);
    }

    #[test]
    fn should_strafe_left() {
        let player = Player::new(
            Position::new(1.0, 2.0),
            1.05,
            super::ActorStats::new(1.0, 0.3),
        );
        let after_move = player.move_left(12);
        assert_that(&after_move.position().x()).is_close_to(-9.409, 0.001);
        assert_that(&after_move.position().y()).is_close_to(7.970, 0.001);
    }

    #[test]
    fn should_strafe_right() {
        let player = Player::new(
            Position::new(1.0, 2.0),
            1.05,
            super::ActorStats::new(1.0, 0.3),
        );
        let after_move = player.move_right(34);
        assert_that(&after_move.position().x()).is_close_to(30.492, 0.001);
        assert_that(&after_move.position().y()).is_close_to(-14.917, 0.001);
    }
}
