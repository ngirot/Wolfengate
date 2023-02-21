use crate::domain::force::Force;

use super::coord::Position;

#[derive(Copy, Clone)]
pub struct Player {
    position: Position,
    inertia: Force,
    orientation: f32,
    stats: PlayerStats,
}

#[derive(Copy, Clone)]
pub struct PlayerStats {
    acceleration: f32,
    deceleration: f32,
    max_speed: f32,
}

pub struct Enemy {
    position: Position,
}

impl Player {
    pub fn new(position: Position, orientation: f32, stats: PlayerStats) -> Self {
        Self {
            position,
            inertia: Force::new(0.0, 0.0, 0.0),
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

    pub fn stats(&self) -> PlayerStats {
        self.stats
    }

    pub fn inertia(&self) -> Force {
        self.inertia
    }

    pub fn with_inertia(&self, inertia: Force) -> Self {
        Self {
            inertia,
            orientation: self.orientation,
            stats: self.stats,
            position: self.position,
        }
    }

    pub fn apply_force(&self, force: Force, microseconds_elapsed: u128) -> Self {
        self.move_direction(force, microseconds_elapsed)
            .rotate(force.rotation() + self.orientation)
    }

    fn rotate(&self, angle: f32) -> Self {
        Self {
            position: self.position,
            inertia: self.inertia,
            orientation: angle,
            stats: self.stats,
        }
    }

    fn move_direction(&self, force: Force, microseconds_elapsed: u128) -> Player {
        let acceleration = microseconds_elapsed as f32 * self.stats.acceleration;
        let reduction = microseconds_elapsed as f32 * self.stats.deceleration;
        let maximum_speed = self.stats.max_speed;

        let current_inertia_with_reduction = Force::new(self.inertia().orientation(), self.inertia().power() - reduction, self.inertia().rotation());

        let mut full_inertia = current_inertia_with_reduction.add(force.power_multiplier(acceleration));

        if full_inertia.power() > maximum_speed {
            full_inertia = Force::new(full_inertia.orientation(), maximum_speed, full_inertia.rotation());
        }

        if full_inertia.power() < 0.0 {
            full_inertia = Force::new(full_inertia.orientation(), 0.0, full_inertia.rotation());
        }

        let new_position = self.position.apply_force(full_inertia);
        Self {
            position: new_position,
            inertia: full_inertia,
            orientation: self.orientation,
            stats: self.stats,
        }
    }
}

impl PlayerStats {
    pub fn new(acceleration: f32, deceleration: f32, max_speed: f32) -> Self {
        Self {
            acceleration,
            deceleration,
            max_speed,
        }
    }

    pub fn acceleration(&self) -> f32 {
        self.acceleration
    }

    pub fn deceleration(&self) -> f32 {
        self.deceleration
    }

    pub fn max_speed(&self) -> f32 {
        self.max_speed
    }
}

impl Enemy {
    pub fn new(position: Position) -> Self {
        Self { position }
    }

    pub fn position(&self) -> Position {
        self.position
    }
}

#[cfg(test)]
mod actor_test {
    use spectral::prelude::*;

    use crate::domain::actor::PlayerStats;
    use crate::domain::coord::Position;
    use crate::domain::force::Force;

    use super::Player;

    #[test]
    fn should_apply_force() {
        let player = Player::new(
            Position::new(1.0, 2.0),
            1.05,
            PlayerStats::new(1.0 / 1000.0, 1.0 / 1000.0, 1000.0),
        );
        let after_move = player.apply_force(Force::new(1.43, 2.3, 0.243), 100);
        assert_that(&after_move.position().x()).is_close_to(0.932, 0.001);
        assert_that(&after_move.position().y()).is_close_to(2.228, 0.001);
        assert_that(&after_move.orientation()).is_close_to(1.293, 0.001);
    }
}