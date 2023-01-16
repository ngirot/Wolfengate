use std::f32::consts::PI;

use crate::domain::actor::Player;
use crate::domain::force::Force;

use super::{
    color::Color,
    coord::{Position, ScreenPoint},
    distance::distance,
    draw_action::DrawAction,
    map::Map,
    texture::TextureIndex,
};

const WALL_MINIMUM_DISTANCE: f32 = 0.1;

pub struct Level {
    screen_height: u16,
    screen_width: u16,
    map: Map,
    player: Player,
}

impl Level {
    pub fn new(screen_width: u16, screen_height: u16, map: Map, player: Player) -> Self {
        Self {
            screen_height,
            screen_width,
            map,
            player,
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
            &self.player.position(),
            self.player.orientation(),
            &self.map,
        ));

        actions
    }

    fn constrains(&self, start: Position, angle: f32, end: Position) -> Position {
        let distance_wall = distance(start, angle, &self.map);
        let distance_move = start.distance(&end);

        if distance_wall.is_none() {
            return end;
        }

        return if distance_move < distance_wall.unwrap().distance() - WALL_MINIMUM_DISTANCE {
            end
        } else {
            start
        };
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
    position: &Position,
    angle: f32,
    map: &Map,
) -> Vec<DrawAction> {
    let mut actions = vec![];

    let view_angle = PI / 2.0; // 45° fov
    let min = angle + (view_angle / 2.0);
    let step = view_angle / width as f32;

    'drawer: for i in 0..width {
        let current_angle = min - step * i as f32;

        let distance_option = distance(*position, current_angle, map);
        if distance_option.is_none() {
            continue 'drawer;
        }
        let projected_point = distance_option.unwrap();

        let column: i32 = i.into();
        let screen_length: i32 = height.into();

        let wall_height = (height as f32 * 0.8) / projected_point.distance();
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
            TextureIndex::WALL,
            projected_point.offset_in_bloc(),
        ));
    }
    actions
}

#[cfg(test)]
mod level_test {
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
}
