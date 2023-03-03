use crate::domain::coord::{Acceleration, Speed};
use crate::domain::force::Force;

use super::coord::Position;

#[derive(Copy, Clone)]
pub struct Player {
    position: Position,
    inertia: Speed,
    orientation: f32,
    stats: PlayerStats,
}

#[derive(Copy, Clone)]
pub struct PlayerStats {
    acceleration: AccelerationStats,
    deceleration: AccelerationStats,
    max_speed: SpeedStats,
}

#[derive(Copy, Clone)]
pub struct AccelerationStats {
    units_per_seconds_square: f32,
}

#[derive(Copy, Clone)]
pub struct SpeedStats {
    units_per_seconds: f32,
}

pub struct Enemy {
    position: Position,
}

impl Player {
    pub fn new(position: Position, orientation: f32, stats: PlayerStats) -> Self {
        Self {
            position,
            inertia: Speed::new(0.0, 0.0),
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

    pub fn inertia(&self) -> Speed {
        self.inertia
    }

    pub fn with_inertia(&self, inertia: Speed) -> Self {
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
        let acceleration = self.stats.acceleration().to_acceleration(force.orientation());
        let reduction = self.stats.deceleration.to_speed_stats(microseconds_elapsed);
        let maximum_speed = self.stats.max_speed;

        let mut full_inertia = self.inertia()
            .rotate(force.rotation());

        if force.power() > 0.0 {
            full_inertia = full_inertia.add(acceleration.to_speed(microseconds_elapsed));
        } else {
            full_inertia = full_inertia.reduce(reduction);
        }

        if full_inertia.units_per_seconds() > maximum_speed.units_per_seconds() {
            full_inertia = Speed::new(full_inertia.orientation(), maximum_speed.units_per_seconds);
        }

        if full_inertia.units_per_seconds() < 0.0 {
            full_inertia = Speed::new(full_inertia.orientation(), 0.0);
        }

        let moves = full_inertia
            .to_move(microseconds_elapsed);
        let new_position = self.position.apply_force(moves);
        Self {
            position: new_position,
            inertia: full_inertia,
            orientation: self.orientation,
            stats: self.stats,
        }
    }
}

impl PlayerStats {
    pub fn new(acceleration: AccelerationStats, deceleration: AccelerationStats, max_speed: SpeedStats) -> Self {
        Self {
            acceleration,
            deceleration,
            max_speed,
        }
    }

    pub fn acceleration(&self) -> AccelerationStats {
        self.acceleration
    }

    pub fn deceleration(&self) -> AccelerationStats {
        self.deceleration
    }

    pub fn max_speed(&self) -> SpeedStats {
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

impl AccelerationStats {
    pub fn new(units_per_seconds_square: f32) -> Self {
        Self {
            units_per_seconds_square,
        }
    }

    pub fn to_acceleration(&self, orientation: f32) -> Acceleration {
        Acceleration::new(orientation, self.units_per_seconds_square)
    }

    pub fn to_speed_stats(&self, microseconds_elapsed: u128) -> SpeedStats {
        SpeedStats::new(self.units_per_seconds_square / 1000000.0 * microseconds_elapsed as f32)
    }
}

impl SpeedStats {
    pub fn new(units_per_seconds: f32) -> Self {
        Self {
            units_per_seconds
        }
    }

    pub fn units_per_seconds(&self) -> f32 {
        self.units_per_seconds
    }
}

#[cfg(test)]
mod actor_test {
    use std::f32::consts::PI;
    use spectral::prelude::*;

    use crate::domain::actor::{AccelerationStats, PlayerStats, SpeedStats};
    use crate::domain::coord::{Position, Speed};
    use crate::domain::force::Force;

    use super::Player;

    #[test]
    fn should_apply_straight_move() {
        let acceleration = AccelerationStats::new(2.0);
        let deceleration = AccelerationStats::new(1.0);
        let max_speed = SpeedStats::new(5.0);

        let player = Player::new(
            Position::new(1.0, 2.0),
            0.0,
            PlayerStats::new(acceleration, deceleration, max_speed),
        );
        let after_move = player.apply_force(Force::new(0.0, 1.0, 0.0), 1000000);
        assert_that(&after_move.position().x()).is_equal_to(3.0);
        assert_that(&after_move.position().y()).is_equal_to(2.0);
    }

    #[test]
    fn should_have_inertia_when_there_is_no_move() {
        let acceleration = AccelerationStats::new(2.0);
        let deceleration = AccelerationStats::new(1.0);
        let max_speed = SpeedStats::new(5.0);

        let player = Player::new(
            Position::new(1.0, 2.0),
            0.0,
            PlayerStats::new(acceleration, deceleration, max_speed),
        ).with_inertia(Speed::new(0.0, 3.0));
        let after_move = player.apply_force(Force::new(0.0, 0.0, 0.0), 1000000);
        assert_that(&after_move.position().x()).is_equal_to(3.0);
        assert_that(&after_move.position().y()).is_equal_to(2.0);
    }

    #[test]
    fn should_go_against_inertia() {
        let acceleration = AccelerationStats::new(2.0);
        let deceleration = AccelerationStats::new(1.0);
        let max_speed = SpeedStats::new(5.0);

        let player = Player::new(
            Position::new(1.0, 5.0),
            0.0,
            PlayerStats::new(acceleration, deceleration, max_speed),
        ).with_inertia(Speed::new(0.0, 3.0));
        let after_move = player.apply_force(Force::new(PI, 1.0, 0.0), 1000000);
        assert_that(&after_move.position().x()).is_equal_to(2.0);
        assert_that(&after_move.position().y()).is_equal_to(5.0);
    }

    #[test]
    fn should_apply_rotation_move() {
        let acceleration = AccelerationStats::new(2.0);
        let deceleration = AccelerationStats::new(1.0);
        let max_speed = SpeedStats::new(5.0);

        let player = Player::new(
            Position::new(1.0, 2.0),
            1.3,
            PlayerStats::new(acceleration, deceleration, max_speed),
        );
        let after_move = player.apply_force(Force::new(0.0, 0.0, 1.2), 1000000);
        assert_that(&after_move.orientation).is_equal_to(2.5);
    }

    #[test]
    fn should_keep_inertia_in_the_same_direction_as_player_orientation() {
        let acceleration = AccelerationStats::new(2.0);
        let deceleration = AccelerationStats::new(1.0);
        let max_speed = SpeedStats::new(5.0);

        let player = Player::new(
            Position::new(1.0, 2.0),
            0.0,
            PlayerStats::new(acceleration, deceleration, max_speed),
        ).with_inertia(Speed::new(0.0, 3.0));
        let after_move = player.apply_force(Force::new(0.0, 0.0, PI / 2.0), 1000000);
        println!("({},{})", player.position.x(), player.position().y());
        println!("({},{})", after_move.position.x(), after_move.position().y());
        assert_that(&after_move.position.x()).is_close_to(1.0, 0.001);
        assert_that(&after_move.position.y()).is_equal_to(4.0);
    }
}