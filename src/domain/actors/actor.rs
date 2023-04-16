use crate::domain::actors::physics::{Acceleration, Speed};
use crate::domain::control::force::Force;
use crate::domain::maths::{Angle, ANGLE_RIGHT};
use crate::domain::topology::coord::Position;
use crate::domain::topology::index::TextureIndex;

#[derive(Copy, Clone)]
pub struct Player {
    position: Position,
    inertia: Speed,
    orientation: Angle,
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

#[derive(Debug, Copy, Clone)]
pub struct SpeedStats {
    units_per_seconds: f32,
}

#[derive(Copy, Clone)]
pub struct Enemy {
    position: Position,
    texture: TextureIndex,
}

impl Player {
    pub fn new(position: Position, orientation: Angle, stats: PlayerStats) -> Self {
        Self {
            position,
            inertia: Speed::new(ANGLE_RIGHT, 0.0),
            orientation,
            stats,
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn orientation(&self) -> Angle {
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

    pub fn with_position(&self, position: Position) -> Self {
        Self {
            inertia: self.inertia,
            orientation: self.orientation,
            stats: self.stats,
            position,
        }
    }

    pub fn apply_force(&self, force: Force, microseconds_elapsed: u128) -> Self {
        self.move_direction(force, microseconds_elapsed)
            .rotate(force.rotation().add(self.orientation))
    }

    fn rotate(&self, angle: Angle) -> Self {
        Self {
            position: self.position,
            inertia: self.inertia,
            orientation: angle,
            stats: self.stats,
        }
    }

    fn move_direction(&self, force: Force, microseconds_elapsed: u128) -> Player {
        let acceleration = self.stats
            .acceleration()
            .to_acceleration(force.orientation());

        let reduction = self.stats.deceleration.to_speed_stats(microseconds_elapsed);
        let maximum_speed = self.stats.max_speed;
        let minimum_speed = SpeedStats::new(0.0);

        let mut full_inertia = self.inertia().rotate(force.rotation());

        if force.power() > 0.0 {
            full_inertia = full_inertia.add(acceleration.to_speed(microseconds_elapsed));
        } else {
            full_inertia = full_inertia.reduce(reduction);
        }

        full_inertia = full_inertia
            .with_max_speed(maximum_speed)
            .with_min_speed(minimum_speed);

        let moves = full_inertia.to_move(microseconds_elapsed);

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
    pub fn new(
        acceleration: AccelerationStats,
        deceleration: AccelerationStats,
        max_speed: SpeedStats,
    ) -> Self {
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
    pub fn new(texture: TextureIndex, position: Position) -> Self {
        Self { position, texture }
    }

    pub fn position(&self) -> Position {
        self.position
    }


    pub fn texture(&self) -> TextureIndex {
        self.texture
    }
}

impl AccelerationStats {
    pub fn new(units_per_seconds_square: f32) -> Self {
        Self {
            units_per_seconds_square,
        }
    }

    pub fn to_acceleration(&self, orientation: Angle) -> Acceleration {
        Acceleration::new(orientation, self.units_per_seconds_square)
    }

    pub fn to_speed_stats(&self, microseconds_elapsed: u128) -> SpeedStats {
        SpeedStats::new(self.units_per_seconds_square / 1000000.0 * microseconds_elapsed as f32)
    }
}

impl SpeedStats {
    pub fn new(units_per_seconds: f32) -> Self {
        Self { units_per_seconds }
    }

    pub fn units_per_seconds(&self) -> f32 {
        self.units_per_seconds
    }

    pub fn to_units(&self, microseconds_elasped: u128) -> f32 {
        microseconds_elasped as f32 / 1000000.0 * self.units_per_seconds as f32
    }
}

#[cfg(test)]
mod actor_test {
    use spectral::prelude::*;
    use crate::domain::actors::actor::{AccelerationStats, PlayerStats, SpeedStats};
    use crate::domain::actors::physics::Speed;
    use crate::domain::control::force::Force;

    use crate::domain::maths::{Angle, ANGLE_LEFT, ANGLE_RIGHT, ANGLE_UP};
    use crate::domain::topology::coord::Position;

    use super::Player;

    #[test]
    fn should_apply_straight_move() {
        let acceleration = AccelerationStats::new(2.0);
        let deceleration = AccelerationStats::new(1.0);
        let max_speed = SpeedStats::new(5.0);

        let player = Player::new(
            Position::new(1.0, 2.0),
            ANGLE_RIGHT,
            PlayerStats::new(acceleration, deceleration, max_speed),
        );
        let after_move = player.apply_force(Force::new(ANGLE_RIGHT, 1.0, ANGLE_RIGHT), 1000000);
        assert_that!(after_move.position().x()).is_equal_to(3.0);
        assert_that!(after_move.position().y()).is_equal_to(2.0);
    }

    #[test]
    fn should_have_inertia_when_there_is_no_move() {
        let acceleration = AccelerationStats::new(2.0);
        let deceleration = AccelerationStats::new(1.0);
        let max_speed = SpeedStats::new(5.0);

        let player = Player::new(
            Position::new(1.0, 2.0),
            ANGLE_RIGHT,
            PlayerStats::new(acceleration, deceleration, max_speed),
        )
            .with_inertia(Speed::new(ANGLE_RIGHT, 3.0));
        let after_move = player.apply_force(Force::new(ANGLE_RIGHT, 0.0, ANGLE_RIGHT), 1000000);
        assert_that!(after_move.position().x()).is_equal_to(3.0);
        assert_that!(after_move.position().y()).is_equal_to(2.0);
    }

    #[test]
    fn should_go_against_inertia() {
        let acceleration = AccelerationStats::new(2.0);
        let deceleration = AccelerationStats::new(1.0);
        let max_speed = SpeedStats::new(5.0);

        let player = Player::new(
            Position::new(1.0, 5.0),
            ANGLE_RIGHT,
            PlayerStats::new(acceleration, deceleration, max_speed),
        )
            .with_inertia(Speed::new(ANGLE_RIGHT, 3.0));
        let after_move = player.apply_force(Force::new(ANGLE_LEFT, 1.0, ANGLE_RIGHT), 1000000);
        assert_that!(after_move.position().x()).is_equal_to(2.0);
        assert_that!(after_move.position().y()).is_equal_to(5.0);
    }

    #[test]
    fn should_apply_rotation_move() {
        let acceleration = AccelerationStats::new(2.0);
        let deceleration = AccelerationStats::new(1.0);
        let max_speed = SpeedStats::new(5.0);

        let player = Player::new(
            Position::new(1.0, 2.0),
            Angle::new(1.3),
            PlayerStats::new(acceleration, deceleration, max_speed),
        );
        let after_move = player.apply_force(Force::new(ANGLE_RIGHT, 0.0, Angle::new(1.2)), 1000000);
        assert_that!(after_move.orientation.to_radiant()).is_equal_to(2.5);
    }

    #[test]
    fn should_keep_inertia_in_the_same_direction_as_player_orientation() {
        let acceleration = AccelerationStats::new(2.0);
        let deceleration = AccelerationStats::new(1.0);
        let max_speed = SpeedStats::new(5.0);

        let player = Player::new(
            Position::new(1.0, 2.0),
            ANGLE_RIGHT,
            PlayerStats::new(acceleration, deceleration, max_speed),
        )
            .with_inertia(Speed::new(ANGLE_RIGHT, 3.0));
        let after_move = player.apply_force(Force::new(ANGLE_RIGHT, 0.0, ANGLE_UP), 1000000);

        assert_that!(after_move.position.x()).is_close_to(1.0, 0.001);
        assert_that!(after_move.position.y()).is_equal_to(4.0);
    }
}

#[cfg(test)]
mod speed_stats_test {
    use spectral::prelude::*;
    use crate::domain::actors::actor::SpeedStats;

    #[test]
    fn to_speed_should_convert_to_units() {
        let stats = SpeedStats::new(123.0);
        assert_that!(stats.to_units(100000)).is_close_to(12.3, 0.01);
    }
}
