use std::f32::consts::PI;

use crate::domain::actor::{Enemy, Player};
use crate::domain::coord::Vector;
use crate::domain::force::Force;
use crate::domain::index::TextureIndex;
use crate::domain::view::ViewScreen;

use super::{
    color::Color,
    coord::{Position, ScreenPoint},
    distance::distance,
    draw_action::DrawAction,
    map::Map,
};

const WALL_MINIMUM_DISTANCE: f32 = 0.1;

pub struct Level {
    view: ViewScreen,
    map: Map,
    player: Player,
    enemies: Vec<Enemy>,
}

struct DrawActionZIndex {
    action: DrawAction,
    z_index: f32,
}

impl Level {
    pub fn new(view: ViewScreen, map: Map, player: Player, enemy: Option<Enemy>) -> Self {
        let enemies = enemy.
            map_or_else(Vec::new, |p| vec![p]);

        Self {
            view,
            map,
            player,
            enemies,
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
        let mut actions_ordered: Vec<DrawActionZIndex> = vec![];

        actions.extend(build_clear_actions());

        actions.extend(build_background_actions(self.view));

        actions_ordered.extend(build_walls(
            self.view,
            self.player.position(),
            self.player.orientation(),
            &self.map,
        ));

        actions_ordered.extend(build_enemies(
            self.view,
            *self.player.position(),
            &self.player.orientation(),
            &self.enemies,
        ));

        actions_ordered.sort_by(|a, b| a.z_index.total_cmp(&b.z_index).reverse());
        actions.extend(actions_ordered.iter().map(|ordered| ordered.action.clone()));

        actions
    }

    fn constrains(&self, start: Position, angle: f32, end: Position) -> Position {
        let distance_wall = distance(start, angle, &self.map).distance();

        let distance_move = start.distance(&end);

        if distance_move < distance_wall - WALL_MINIMUM_DISTANCE {
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
        }
    }

    fn distance(&self, start: Position, angle: f32) -> f32 {
        distance(start, angle, &self.map)
            .distance()
    }
}

impl DrawActionZIndex {
    fn new(action: DrawAction, z_index: f32) -> Self {
        Self { action, z_index }
    }
}

fn build_clear_actions() -> Vec<DrawAction> {
    vec![DrawAction::Clear(Color::new(0, 0, 0))]
}

fn build_background_actions(view: ViewScreen) -> Vec<DrawAction> {
    let height: i32 = view.height();
    let width: i32 = view.width();
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
    view: ViewScreen,
    position: &Position,
    angle: f32,
    map: &Map,
) -> Vec<DrawActionZIndex> {
    let mut actions = vec![];

    let min = angle + (view.angle() / 2.0);
    let step = view.angle() / view.width() as f32;

    for i in 0..view.width() {
        let current_angle = min - step * i as f32;

        let projected_point = distance(*position, current_angle, map);

        let screen_length: i32 = view.height();

        let wall_height = height_ratio(view.height()) / projected_point.distance();
        let start = ScreenPoint::new(
            i,
            (screen_length as f32 / 2.0 - wall_height / 2.0) as i32,
        );
        let end = ScreenPoint::new(
            i,
            (screen_length as f32 / 2.0 + wall_height / 2.0) as i32,
        );

        let action = DrawAction::TexturedLine(
            start,
            end,
            projected_point.tile_type(),
            projected_point.offset_in_bloc(),
        );
        actions.push(DrawActionZIndex::new(action, projected_point.distance()));
    }
    actions
}

fn build_enemies(view: ViewScreen, view_position: Position, orientation: &f32, enemies: &Vec<Enemy>) -> Vec<DrawActionZIndex> {
    let mut actions = vec![];
    for enemy in enemies {
        let view_vector = Vector::new(view_position, Position::new(view_position.x() + orientation.cos(), view_position.y() + orientation.sin()));
        let enemy_vector = Vector::new(view_position, enemy.position());

        let angle = (view_vector.angle(enemy_vector).unwrap()) % (2.0 * PI);
        let angle_sign = view_vector.angle_sign(enemy_vector);
        let step = view.width() as f32 / view.angle() as f32;

        let x = view.width() as f32 / 2.0 + angle * step * angle_sign;

        let distance = view_position.distance(&enemy.position());
        let sprite_height = height_ratio(view.height()) / distance;
        let start = ScreenPoint::new((x - sprite_height / 2.0) as i32, (view.height() as f32 / 2.0 - sprite_height / 2.0) as i32);
        let end = ScreenPoint::new((x + sprite_height / 2.0) as i32, (view.height() as f32 / 2.0 + sprite_height / 2.0) as i32);

        let action = DrawAction::Sprite(start, end, TextureIndex::ENEMY);
        actions.push(DrawActionZIndex::new(action, distance))
    }
    actions
}

fn height_ratio(screen_height: i32) -> f32 {
    screen_height as f32 * 0.8
}

#[cfg(test)]
mod level_test {
    use std::f32::consts::PI;

    use spectral::prelude::*;

    use crate::domain::{coord::Position, draw_action::DrawAction, map::Map};
    use crate::domain::actor::{Enemy, Player};
    use crate::domain::force::Force;
    use crate::domain::level::WALL_MINIMUM_DISTANCE;
    use crate::domain::view::ViewScreen;

    use super::Level;

    #[test]
    fn actions_should_start_with_a_clear() {
        let player = Player::new(Position::new(0.0, 0.0), 0.0);
        let view = ViewScreen::new(0, 0, PI / 2.0);
        let level = Level::new(view, Map::new("#").unwrap(), player, None);

        let actions = level.generate_actions();

        assert_that(&actions.len()).is_greater_than(0);

        assert!(matches!(actions[0], DrawAction::Clear { .. }));
    }

    #[test]
    fn actions_should_draw_ceiling() {
        let player = Player::new(Position::new(0.0, 0.0), 0.0);
        let view = ViewScreen::new(200, 100, PI / 2.0);
        let level = Level::new(view, Map::new("#").unwrap(), player, None);
        let mut found = false;

        let actions = level.generate_actions();

        for action in actions {
            if let DrawAction::Rectangle(start, end, _) = action {
                println!("{} {} {} {}", start.x(), start.y(), end.x(), end.y());
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
        let view = ViewScreen::new(200, 100, PI / 2.0);

        let level = Level::new(view, Map::new("#").unwrap(), player, None);
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
        let view = ViewScreen::new(100, 100, PI / 2.0);

        let mut level = Level::new(view, map, player, None);

        level.apply_forces(Force::new(0.0, 10.0, 0.0));
        assert_that(&level.player.position().x()).is_less_than(2.0 - WALL_MINIMUM_DISTANCE);
    }

    #[test]
    fn apply_force_should_apply_not_constrained_moves() {
        let map = Map::new("# #").unwrap();
        let player = Player::new(Position::new(1.5, 0.5), 0.0);
        let view = ViewScreen::new(100, 100, PI / 2.0);

        let mut level = Level::new(view, map, player, None);

        level.apply_forces(Force::new(0.0, 0.2, 0.0));

        assert_that(&level.player.position().x()).is_equal_to(1.7);
    }

    #[test]
    fn enemy_should_be_in_the_list_after_the_wall_behind_him() {
        let map = Map::new("#  #").unwrap();
        let player = Player::new(Position::new(1.5, 0.5), 0.0);
        let enemy = Enemy::new(Position::new(2.0, 0.5));
        let view = ViewScreen::new(100, 100, PI / 2.0);

        let level = Level::new(view, map, player, Some(enemy));

        let actions = level.generate_actions();

        let position_sprite = actions.iter().position(|action| matches!(action, DrawAction::Sprite(_, _, _)));
        let position_wall = actions.iter()
            .enumerate()
            .filter(|(_, action)| matches!(action, DrawAction::TexturedLine(_, _, _,_)))
            .map(|(index, _)| index)
            .max();

        assert_that(&position_sprite).is_greater_than(position_wall);
    }

    #[test]
    fn enemy_should_be_in_the_list_before_the_wall_before_him() {
        let map = Map::new("# #   ").unwrap();
        let player = Player::new(Position::new(1.5, 0.5), 0.0);
        let enemy = Enemy::new(Position::new(4.5, 0.5));
        let view = ViewScreen::new(100, 100, PI / 2.0);

        let level = Level::new(view, map, player, Some(enemy));

        let actions = level.generate_actions();

        let position_sprite = actions.iter().position(|action| matches!(action, DrawAction::Sprite(_, _, _)));
        let position_wall = actions.iter()
            .enumerate()
            .filter(|(_, action)| matches!(action, DrawAction::TexturedLine(_, _, _,_)))
            .map(|(index, _)| index)
            .min();

        assert_that(&position_sprite).is_less_than(position_wall);
    }

    #[test]
    fn apply_force_should_constrains_move_by_sliding_through_the_wall_by_top() {
        let map = Map::new("# #").unwrap();
        let player = Player::new(Position::new(1.5, 0.5), PI / 4.0);
        let view = ViewScreen::new(100, 100, PI / 2.0);

        let mut level = Level::new(view, map, player, None);

        level.apply_forces(Force::new(0.0, 0.2, 0.0));

        assert_that(&level.player.position().x()).is_close_to(1.641, 0.001);
        assert_that(&level.player.position().y()).is_close_to(0.641, 0.001);
    }

    #[test]
    fn apply_force_should_constrains_move_by_sliding_through_the_wall_by_bottom() {
        let map = Map::new("# #").unwrap();
        let player = Player::new(Position::new(1.5, 0.5), -PI / 4.0);
        let view = ViewScreen::new(100, 100, PI / 2.0);

        let mut level = Level::new(view, map, player, None);

        level.apply_forces(Force::new(0.0, 0.2, 0.0));

        assert_that(&level.player.position().x()).is_close_to(1.641, 0.001);
        assert_that(&level.player.position().y()).is_close_to(0.358, 0.001);
    }

    #[test]
    fn apply_force_should_constrains_move_by_sliding_through_the_wall_by_right() {
        let map = Map::new("#\n \n#").unwrap();
        let player = Player::new(Position::new(1.5, 0.5), PI / 4.0);
        let view = ViewScreen::new(100, 100, PI / 2.0);

        let mut level = Level::new(view, map, player, None);

        level.apply_forces(Force::new(PI / 2.0, 0.2, 0.0));

        assert_that(&level.player.position().x()).is_close_to(1.358, 0.001);
        assert_that(&level.player.position().y()).is_close_to(0.641, 0.001);
    }

    #[test]
    fn apply_force_should_constrains_move_by_sliding_through_the_wall_by_left() {
        let map = Map::new("#\n \n#").unwrap();
        let player = Player::new(Position::new(1.5, 0.5), 3.0 * PI / 4.0);
        let view = ViewScreen::new(100, 100, PI / 2.0);

        let mut level = Level::new(view, map, player, None);

        level.apply_forces(Force::new(PI / 2.0, 0.2, 0.0));

        assert_that(&level.player.position().x()).is_close_to(1.358, 0.001);
        assert_that(&level.player.position().y()).is_close_to(0.358, 0.001);
    }
}
