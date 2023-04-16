use rayon::prelude::*;

use crate::domain::actors::actor::{Enemy, Player};
use crate::domain::control::actions::Actions;
use crate::domain::control::force::Force;
use crate::domain::maths::{Angle, signed_angle, Vector};
use crate::domain::topology::coord::{Position, ScreenPoint};
use crate::domain::topology::map::Map;
use crate::domain::topology::projection::{project, ProjectedPoint};
use crate::domain::ui::color::Color;
use crate::domain::ui::draw_action::DrawAction;
use crate::domain::ui::view::ViewScreen;

const WALL_MINIMUM_DISTANCE: f32 = 0.1;

pub struct Level {
    view: ViewScreen,
    map: Map,
    actions: Actions,
    player: Player,
    enemies: Vec<Enemy>,
}

struct DrawActionZIndex {
    action: DrawAction,
    z_index: f32,
}

impl Level {
    pub fn new(view: ViewScreen, map: Map) -> Self {
        let actions = Actions::new(&map);

        Self {
            view,
            player: map.generate_player().unwrap(),
            enemies: map.generate_enemies(),
            map,
            actions,
        }
    }

    pub fn apply_forces(&mut self, force: Force, microseconds_elapsed: u128) {
        let relative_force = force.for_relative_view(self.player.orientation());
        let no_limit = self
            .player
            .apply_force(relative_force, microseconds_elapsed);
        let constrained = self.constrains(*self.player.position(), *no_limit.position());

        self.player = Player::new(constrained, no_limit.orientation(), self.player.stats())
            .with_inertia(no_limit.inertia())
    }

    pub fn handle_action(&mut self) {
        let action_projected = project(*self.player.position(), self.player.orientation(), &self.map, &self.actions);
        if !action_projected.is_empty() {
            let closest = action_projected[0];
            if closest.distance() < 1.0 {
                let map_point = closest.map_point();
                self.actions.activate(map_point.x(), map_point.y());
            }
        }
    }

    pub fn notify_elapsed(&mut self, microseconds: u128) {
        self.actions.notify_elapsed(microseconds);
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
            &self.actions,
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

    pub fn teleport(&mut self, to: Position) {
        self.player = self.player.with_position(to);
    }

    fn constrains(&self, start: Position, end: Position) -> Position {
        let angle_opt = signed_angle(start, end);
        if angle_opt.is_none() {
            return end;
        }

        let angle = angle_opt.unwrap();

        let angle_x = angle.align_to_x();
        let angle_y = angle.align_to_y();

        let distance_x = self.distance(start, angle_x) - WALL_MINIMUM_DISTANCE;
        let distance_y = self.distance(start, angle_y) - WALL_MINIMUM_DISTANCE;

        let mov_x = (end.x() - start.x()).abs();
        let mov_y = (end.y() - start.y()).abs();

        let should_go_x = self.min(mov_x, distance_x);
        let should_go_y = self.min(mov_y, distance_y);

        start
            .with_x(start.x() + (should_go_x * angle.cos().signum()))
            .with_y(start.y() + (should_go_y * angle.sin().signum()))
    }

    fn min(&self, a: f32, b: f32) -> f32 {
        if a < b {
            a
        } else {
            b
        }
    }

    fn distance(&self, start: Position, angle: Angle) -> f32 {
        let projected: Vec<ProjectedPoint> = project(start, angle, &self.map, &self.actions)
            .iter()
            .filter(|projection| projection.blocking())
            .cloned()
            .collect();

        if !projected.is_empty() {
            projected[0].distance()
        } else {
            0.0
        }
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
    angle: Angle,
    map: &Map,
    actions: &Actions,
) -> Vec<DrawActionZIndex> {
    angle.discreet_cone(view.angle(), view.width())
        .par_iter()
        .enumerate()
        .map(|(i, angle)| (i, project(*position, *angle, map, actions)))
        .flat_map(|(i, projected)| projected.into_iter().map(|p| (i, p)).collect::<Vec<(usize, ProjectedPoint)>>())
        .map(|(i, projected_point)| {
            let screen_length: i32 = view.height();

            let projection_distance = projected_point.distance();
            let wall_height = object_height(view, projection_distance);
            let start = ScreenPoint::new(
                i as i32,
                (screen_length as f32 / 2.0 - wall_height / 2.0) as i32,
            );
            let end = ScreenPoint::new(
                i as i32,
                (screen_length as f32 / 2.0 + wall_height / 2.0) as i32,
            );

            let action = DrawAction::TexturedLine(
                start,
                end,
                projected_point.texture(),
                projected_point.offset_in_bloc(),
            );
            DrawActionZIndex::new(action, projection_distance)
        })
        .collect()
}

fn build_enemies(
    view: ViewScreen,
    view_position: Position,
    orientation: &Angle,
    enemies: &Vec<Enemy>,
) -> Vec<DrawActionZIndex> {
    let mut actions = vec![];
    for enemy in enemies {
        let view_vector = Vector::new(
            view_position,
            Position::new(
                view_position.x() + orientation.cos(),
                view_position.y() + orientation.sin(),
            ),
        );
        let enemy_vector = Vector::new(view_position, enemy.position());

        let angle = view_vector.angle(enemy_vector).unwrap();
        let angle_negative = view_vector.angle_sign_is_negative(enemy_vector);

        let x = angle.position_in_discreet_cone(view.angle(), view.width(), angle_negative);

        let distance = view_position.distance(&enemy.position());
        let sprite_height = object_height(view, distance);
        let start = ScreenPoint::new(
            (x - sprite_height / 2.0) as i32,
            (view.height() as f32 / 2.0 - sprite_height / 2.0) as i32,
        );
        let end = ScreenPoint::new(
            (x + sprite_height / 2.0) as i32,
            (view.height() as f32 / 2.0 + sprite_height / 2.0) as i32,
        );

        let action = DrawAction::Sprite(start, end, enemy.texture());
        actions.push(DrawActionZIndex::new(action, distance))
    }
    actions
}

fn object_height(view: ViewScreen, distance: f32) -> f32 {
    (view.height() as f32 * view.ratio()) / distance
}

#[cfg(test)]
mod level_test {
    use std::f32::consts::PI;

    use spectral::prelude::*;

    use crate::domain::control::force::Force;
    use crate::domain::level::WALL_MINIMUM_DISTANCE;
    use crate::domain::maths::{Angle, ANGLE_RIGHT};
    use crate::domain::topology::coord::Position;
    use crate::domain::topology::map::map_test::build_map;
    use crate::domain::ui::draw_action::DrawAction;
    use crate::domain::ui::view::ViewScreen;

    use super::Level;

    const TOLERANCE: f32 = WALL_MINIMUM_DISTANCE + 0.01;

    #[test]
    fn actions_should_start_with_a_clear() {
        let view = ViewScreen::new(0, 0);
        let level = Level::new(view, build_map("r#"));

        let actions = level.generate_actions();

        assert_that!(actions.len()).is_greater_than(0);

        assert!(matches!(actions[0], DrawAction::Clear { .. }));
    }

    #[test]
    fn actions_should_draw_ceiling() {
        let view = ViewScreen::new(200, 100);
        let level = Level::new(view, build_map("r#"));
        let mut found = false;

        let actions = level.generate_actions();

        for action in actions {
            if let DrawAction::Rectangle(start, end, _) = action {
                if start.x() == 0 && start.y() == 0 && end.x() == 100 && end.y() == 100 {
                    found = true
                }
            }
        }

        assert_that!(found).is_true();
    }

    #[test]
    fn actions_should_draw_floor() {
        let view = ViewScreen::new(200, 100);

        let level = Level::new(view, build_map("r#"));
        let mut found = false;

        let actions = level.generate_actions();

        for action in actions {
            if let DrawAction::Rectangle(start, end, _) = action {
                if start.x() == 0 && start.y() == 100 && end.x() == 100 && end.y() == 200 {
                    found = true
                }
            }
        }

        assert_that!(found).is_true();
    }

    #[test]
    fn apply_force_should_constraint_moves() {
        let map = build_map("#r#");
        let view = ViewScreen::new(100, 100);

        let mut level = Level::new(view, map);

        level.apply_forces(Force::new(ANGLE_RIGHT, 10.0, ANGLE_RIGHT), 1000000);
        assert_that!(level.player.position().x()).is_less_than_or_equal_to(2.0);
    }

    #[test]
    fn apply_force_should_apply_not_constrained_moves() {
        let map = build_map("#r#");
        let view = ViewScreen::new(100, 100);

        let mut level = Level::new(view, map);

        level.apply_forces(Force::new(ANGLE_RIGHT, 0.2, ANGLE_RIGHT), 1000000);

        assert_that!(level.player.position().x()).is_greater_than(1.5);
    }

    #[test]
    fn enemy_should_be_in_the_list_after_the_wall_behind_him() {
        let map = build_map("#rE #");
        let view = ViewScreen::new(100, 100);

        let mut level = Level::new(view, map);
        level.teleport(Position::new(2.3, 0.5));

        let actions = level.generate_actions();

        let position_sprite = actions
            .iter()
            .position(|action| matches!(action, DrawAction::Sprite(_, _, _)));
        let position_wall = actions
            .iter()
            .enumerate()
            .filter(|(_, action)| matches!(action, DrawAction::TexturedLine(_, _, _, _)))
            .map(|(index, _)| index)
            .max();

        assert_that!(position_sprite).is_greater_than(position_wall);
    }

    #[test]
    fn enemy_should_be_in_the_list_before_the_wall_before_him() {
        let map = build_map("#r# E ");
        let view = ViewScreen::new(100, 100);
        let level = Level::new(view, map);

        let actions = level.generate_actions();

        let position_sprite = actions
            .iter()
            .position(|action| matches!(action, DrawAction::Sprite(_, _, _)));
        let position_wall = actions
            .iter()
            .enumerate()
            .filter(|(_, action)| matches!(action, DrawAction::TexturedLine(_, _, _, _)))
            .map(|(index, _)| index)
            .min();

        assert_that!(position_sprite).is_less_than(position_wall);
    }

    #[test]
    fn apply_force_should_constrains_move_by_sliding_through_the_wall_by_top() {
        let map = build_map("#r#");
        let view = ViewScreen::new(100, 100);

        let mut level = Level::new(view, map);

        level.apply_forces(
            Force::new(Angle::new(PI / 16.0), 1.0, ANGLE_RIGHT),
            10000000,
        );

        assert_that!(level.player.position().x()).is_close_to(2.0, TOLERANCE);
        assert_that!(level.player.position().y()).is_greater_than(0.5);
    }

    #[test]
    fn apply_force_should_constrains_move_by_sliding_through_the_wall_by_bottom() {
        let map = build_map("#l#");
        let view = ViewScreen::new(100, 100);

        let mut level = Level::new(view, map);

        level.apply_forces(Force::new(Angle::new(PI / 16.0), 1.0, ANGLE_RIGHT), 1000000);

        assert_that!(level.player.position().x()).is_close_to(1.0, TOLERANCE);
        assert_that!(level.player.position().y()).is_less_than(0.5);
    }

    #[test]
    fn apply_force_should_constrains_move_by_sliding_through_the_wall_by_right() {
        let map = build_map("#\nu\n#");
        let view = ViewScreen::new(100, 100);

        let mut level = Level::new(view, map);

        level.apply_forces(
            Force::new(Angle::new(-PI / 16.0), 1.0, ANGLE_RIGHT),
            1000000,
        );

        assert_that!(level.player.position().x()).is_greater_than(0.5);
        assert_that!(level.player.position().y()).is_close_to(2.0, TOLERANCE);
    }

    #[test]
    fn apply_force_should_constrains_move_by_sliding_through_the_wall_by_left() {
        let map = build_map("#\nd\n#");
        let view = ViewScreen::new(100, 100);

        let mut level = Level::new(view, map);

        level.apply_forces(
            Force::new(Angle::new(-PI / 16.0), 1.0, ANGLE_RIGHT),
            1000000,
        );

        assert_that!(level.player.position().x()).is_less_than(0.5);
        assert_that!(level.player.position().y()).is_close_to(1.0, TOLERANCE);
    }
}
