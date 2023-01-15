use crate::domain::force::Force;

use super::coord::Position;

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
        let relative_force = force.for_relative_view(self.orientation);
        let new_position = self.position.apply_force(relative_force);
        Self {
            position: new_position,
            orientation: self.orientation,
        }
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
        assert_that(&after_move.position().x()).is_close_to(-0.814, 0.001);
        assert_that(&after_move.position().y()).is_close_to(3.413, 0.001);
        assert_that(&after_move.orientation()).is_close_to(1.293, 0.001);
    }
}