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

    pub fn move_forward(&self) -> Self {
        let new_position = Position::new(
            self.position.x() + self.orientation.cos() * self.stats.movement_speed,
            self.position.y() + self.orientation.sin() * self.stats.movement_speed,
        );
        Self {
            position: new_position,
            orientation: self.orientation,
            stats: self.stats,
        }
    }

    pub fn move_backward(&self) -> Self {
        let new_position = Position::new(
            self.position.x() - self.orientation.cos() * self.stats.movement_speed,
            self.position.y() - self.orientation.sin() * self.stats.movement_speed,
        );
        Self {
            position: new_position,
            orientation: self.orientation,
            stats: self.stats,
        }
    }

    pub fn rotate_left(&self) -> Self {
        let new_orientation = self.orientation + self.stats.rotation_speed;
        Self {
            position: self.position,
            orientation: new_orientation,
            stats: self.stats,
        }
    }

    pub fn rotate_right(&self) -> Self {
        let new_orientation = self.orientation - self.stats.rotation_speed;
        Self {
            position: self.position,
            orientation: new_orientation,
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
            super::ActorStats::new(1.0, 1.0),
        );
        let after_move = player.move_forward();
        assert_that(&after_move.position().x()).is_close_to(1.497, 0.001);
        assert_that(&after_move.position().y()).is_close_to(2.867, 0.001);
    }

    #[test]
    fn should_move_backward() {
        let player = Player::new(
            Position::new(1.0, 2.0),
            1.05,
            super::ActorStats::new(1.0, 1.0),
        );
        let after_move = player.move_backward();
        assert_that(&after_move.position().x()).is_close_to(0.502, 0.001);
        assert_that(&after_move.position().y()).is_close_to(1.132, 0.001);
    }

    #[test]
    fn should_rotate_left() {
        let player = Player::new(
            Position::new(1.0, 2.0),
            1.05,
            super::ActorStats::new(1.0, 0.3),
        );
        let after_move = player.rotate_left();
        assert_that(&after_move.orientation).is_close_to(1.349, 0.001);
    }

    #[test]
    fn should_rotate_right() {
        let player = Player::new(
            Position::new(1.0, 2.0),
            1.05,
            super::ActorStats::new(1.0, 0.3),
        );
        let after_move = player.rotate_right();
        assert_that(&after_move.orientation).is_close_to(0.749, 0.001);
    }
}