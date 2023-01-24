use std::f32::consts::PI;

use crate::domain::actor::{Enemy, Player};
use crate::domain::coord::Vector;
use crate::domain::force::Force;
use crate::domain::index::TextureIndex;

use super::{
    color::Color,
    coord::{Position, ScreenPoint},
    distance::distance,
    draw_action::DrawAction,
    map::Map,
};

const WALL_MINIMUM_DISTANCE: f32 = 0.1;

pub struct Level {
    screen_height: u16,
    screen_width: u16,
    view_angle: f32,
    map: Map,
    player: Player,
    enemies: Vec<Enemy>,
}

impl Level {
    pub fn new(screen_width: u16, screen_height: u16, map: Map, player: Player) -> Self {
        Self {
            screen_height,
            screen_width,
            view_angle: PI / 2.0, // 45° fov
            map,
            player,
            enemies: vec![Enemy::new(Position::new(5.0, 5.0))],
        }
    }

    pub fn apply_forces(&mut self, force: Force) {
        let relative_force = force.for_relative_view(self.player.orientation());
        let no_limit = self.player.apply_force(relative_force);
        let constrained = self.constrains(*self.player.position(), relative_force.orientation(), *no_limit.position());

        self.player = Player::new(constrained, no_limit.orientation())
    }

    pub fn generate_actions(&self) -> Vec<DrawAction> {
        let mut actions: Vec<DrawAction> = vec![];

        actions.extend(build_clear_actions());

        actions.extend(build_background_actions(
            self.screen_width,
            self.screen_height,
        ));

        actions.extend(build_walls(
            self.screen_width,
            self.screen_height,
            self.view_angle,
            &self.player.position(),
            self.player.orientation(),
            &self.map,
        ));

        actions.extend(build_enemies(
            self.screen_height,
            self.screen_width,
            self.view_angle,
            *self.player.position(),
            &self.player.orientation(),
            &self.enemies,
        ));

        actions
    }

    fn constrains(&self, start: Position, angle: f32, end: Position) -> Position {
        let distance_wall = distance(start, angle, &self.map).distance();

        let distance_move = start.distance(&end);

        return if distance_move < distance_wall - WALL_MINIMUM_DISTANCE {
            end
        } else {
            let angle_x = if angle.cos() >= 0.0 { 0.0 } else { PI };
            let angle_y = if angle.sin() >= 0.0 { PI / 2.0 } else { 3.0 * PI / 2.0 };

            let distance_x = self.distance(start, angle_x);
            let distance_y = self.distance(start, angle_y);

            if distance_x < distance_wall && distance_y < distance_wall {
                return start;
            }

            if distance_x == distance_y {
                return start;
            }
            if distance_x < distance_y {
                start.with_y(end.y())
            } else {
                start.with_x(end.x())
            }
        };
    }

    fn distance(&self, start: Position, angle: f32) -> f32 {
        distance(start, angle, &self.map)
            .distance()
    }
}

fn build_clear_actions() -> Vec<DrawAction> {
    vec![DrawAction::Clear(Color::new(0, 0, 0))]
}

fn build_background_actions(width: u16, height: u16) -> Vec<DrawAction> {
    let height: i32 = height.into();
    let width: i32 = width.into();
    let mid_screen = height / 2;

    vec![
        DrawAction::Rectangle(
            ScreenPoint::new(0, 0),
            ScreenPoint::new(width, mid_screen),
            Color::new(50, 50, 50),
        ),
        DrawAction::Rectangle(
            ScreenPoint::new(0, mid_screen),
            ScreenPoint::new(width, height),
            Color::new(100, 100, 100),
        ),
    ]
}

fn build_walls(
    width: u16,
    height: u16,
    view_angle: f32,
    position: &Position,
    angle: f32,
    map: &Map,
) -> Vec<DrawAction> {
    let mut actions = vec![];

    let min = angle + (view_angle / 2.0);
    let step = view_angle / width as f32;

    for i in 0..width {
        let current_angle = min - step * i as f32;

        let projected_point = distance(*position, current_angle, map);

        let column: i32 = i.into();
        let screen_length: i32 = height.into();

        let wall_height = height_ratio(height) / projected_point.distance();
        let start = ScreenPoint::new(
            column,
            (screen_length as f32 / 2.0 - wall_height / 2.0) as i32,
        );
        let end = ScreenPoint::new(
            column,
            (screen_length as f32 / 2.0 + wall_height / 2.0) as i32,
        );

        actions.push(DrawAction::TexturedLine(
            start,
            end,
            projected_point.tile_type(),
            projected_point.offset_in_bloc(),
        ));
    }
    actions
}

fn build_enemies(height: u16, width: u16, view_angle: f32, view_position: Position, orientation: &f32, enemies: &Vec<Enemy>) -> Vec<DrawAction> {
    let mut actions = vec![];
    for enemy in enemies {
        let view_vector = Vector::new(view_position, Position::new(view_position.x() + orientation.cos(), view_position.y() + orientation.sin()));
        let enemy_vector = Vector::new(view_position, enemy.position());

        let angle = (view_vector.angle(enemy_vector).unwrap()) % (2.0 * PI);
        let angle_sign = view_vector.angle_sign(enemy_vector);
        let step = width as f32 / view_angle as f32;

        let x = width as f32 / 2.0 + angle * step * angle_sign;

        let distance = view_position.distance(&enemy.position());
        let sprite_height = height_ratio(height) / distance;
        let start = ScreenPoint::new((x - sprite_height / 2.0) as i32, (height as f32 / 2.0 - sprite_height / 2.0) as i32);
        let end = ScreenPoint::new((x + sprite_height / 2.0) as i32, (height as f32 / 2.0 + sprite_height / 2.0) as i32);

        actions.push(DrawAction::Sprite(start, end, TextureIndex::ENEMY))
    }
    actions
}

fn height_ratio(screen_height: u16) -> f32 {
    screen_height as f32 * 0.8
}

#[cfg(test)]
mod level_test {
    use std::f32::consts::PI;

    use spectral::prelude::*;

    use crate::domain::{coord::Position, draw_action::DrawAction, map::Map};
    use crate::domain::actor::Player;
    use crate::domain::force::Force;
    use crate::domain::level::WALL_MINIMUM_DISTANCE;

    use super::Level;

    #[test]
    fn actions_should_start_with_a_clear() {
        let player = Player::new(Position::new(0.0, 0.0), 0.0);
        let level = Level::new(0, 0, Map::new("#").unwrap(), player);

        let actions = level.generate_actions();

        assert_that(&actions.len()).is_greater_than(0);

        assert!(matches!(actions[0], DrawAction::Clear { .. }));
    }

    #[test]
    fn actions_should_draw_ceiling() {
        let player = Player::new(Position::new(0.0, 0.0), 0.0);
        let level = Level::new(100, 200, Map::new("#").unwrap(), player);
        let mut found = false;

        let actions = level.generate_actions();

        for action in actions {
            if let DrawAction::Rectangle(start, end, _) = action {
                if start.x() == 0 && start.y() == 0 && end.x() == 100 && end.y() == 100 {
                    found = true
                }
            }
        }

        assert_that(&found).is_true();
    }

    #[test]
    fn actions_should_draw_floor() {
        let player = Player::new(Position::new(0.0, 0.0), 0.0);
        let level = Level::new(100, 200, Map::new("#").unwrap(), player);
        let mut found = false;

        let actions = level.generate_actions();

        for action in actions {
            if let DrawAction::Rectangle(start, end, _) = action {
                if start.x() == 0 && start.y() == 100 && end.x() == 100 && end.y() == 200 {
                    found = true
                }
            }
        }

        assert_that(&found).is_true();
    }

    #[test]
    fn apply_force_should_constraint_moves() {
        let map = Map::new("# #").unwrap();
        let player = Player::new(Position::new(1.5, 0.5), 0.0);
        let mut level = Level::new(100, 100, map, player);

        level.apply_forces(Force::new(0.0, 10.0, 0.0));
        assert_that(&level.player.position().x()).is_less_than(2.0 - WALL_MINIMUM_DISTANCE);
    }

    #[test]
    fn apply_force_should_apply_not_constrained_moves() {
        let map = Map::new("# #").unwrap();
        let player = Player::new(Position::new(1.5, 0.5), 0.0);
        let mut level = Level::new(100, 100, map, player);

        level.apply_forces(Force::new(0.0, 0.2, 0.0));

        assert_that(&level.player.position().x()).is_equal_to(1.7);
    }

    #[test]
    fn apply_force_should_constrains_move_by_sliding_through_the_wall_by_top() {
        let map = Map::new("# #").unwrap();
        let player = Player::new(Position::new(1.5, 0.5), PI / 4.0);
        let mut level = Level::new(100, 100, map, player);

        level.apply_forces(Force::new(0.0, 0.2, 0.0));

        assert_that(&level.player.position().x()).is_close_to(1.641, 0.001);
        assert_that(&level.player.position().y()).is_close_to(0.641, 0.001);
    }

    #[test]
    fn apply_force_should_constrains_move_by_sliding_through_the_wall_by_bottom() {
        let map = Map::new("# #").unwrap();
        let player = Player::new(Position::new(1.5, 0.5), -PI / 4.0);
        let mut level = Level::new(100, 100, map, player);

        level.apply_forces(Force::new(0.0, 0.2, 0.0));

        assert_that(&level.player.position().x()).is_close_to(1.641, 0.001);
        assert_that(&level.player.position().y()).is_close_to(0.358, 0.001);
    }

    #[test]
    fn apply_force_should_constrains_move_by_sliding_through_the_wall_by_right() {
        let map = Map::new("#\n \n#").unwrap();
        let player = Player::new(Position::new(1.5, 0.5), PI / 4.0);
        let mut level = Level::new(100, 100, map, player);

        level.apply_forces(Force::new(PI / 2.0, 0.2, 0.0));

        assert_that(&level.player.position().x()).is_close_to(1.358, 0.001);
        assert_that(&level.player.position().y()).is_close_to(0.641, 0.001);
    }

    #[test]
    fn apply_force_should_constrains_move_by_sliding_through_the_wall_by_left() {
        let map = Map::new("#\n \n#").unwrap();
        let player = Player::new(Position::new(1.5, 0.5), 3.0 * PI / 4.0);
        let mut level = Level::new(100, 100, map, player);

        level.apply_forces(Force::new(PI / 2.0, 0.2, 0.0));

        assert_that(&level.player.position().x()).is_close_to(1.358, 0.001);
        assert_that(&level.player.position().y()).is_close_to(0.358, 0.001);
    }
}
