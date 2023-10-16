use std::f32::consts::PI;
use rayon::prelude::*;

use crate::domain::actors::actor::{Enemy, Player};
use crate::domain::actors::shoot::{Weapon, ShootState};
use crate::domain::control::actions::Actions;
use crate::domain::control::force::Force;
use crate::domain::level_drawer::{build_background_actions, build_clear_actions, build_enemies, build_walls, build_weapons, DrawActionZIndex};
use crate::domain::maths::{Angle, signed_angle, Vector};
use crate::domain::topology::coord::{Position};
use crate::domain::topology::map::Map;
use crate::domain::topology::projection::{project, ProjectedPoint};
use crate::domain::ui::draw_action::DrawAction;
use crate::domain::ui::view::ViewScreen;

const WALL_MINIMUM_DISTANCE: f32 = 0.1;

pub struct Level {
    view: ViewScreen,
    map: Map,
    actions: Actions,
    player: Player,
    enemies: Vec<Enemy>,
    current_weapon: Weapon,
}


impl Level {
    pub fn new(view: ViewScreen, map: Map) -> Self {
        let actions = Actions::new(&map);

        Self {
            view,
            current_weapon: map.generate_weapon(),
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

    pub fn apply_shoots(&mut self) {
        if matches!(self.current_weapon.state(), ShootState::Active) {
            Level::sword(&mut self.enemies, self.player);
        }
    }

    fn sword(enemies: &mut Vec<Enemy>, player: Player) {
        let range_distance = 0.5;
        let range_angle = Angle::new(PI / 4.0);

        enemies.par_iter_mut().for_each(|enemy| {
            let enemy_size = 0.5;
            let distance = player.position().distance(&enemy.position());
            let look_at = Vector::from_angle(player.orientation());
            let enemy_look = Vector::new(*player.position(), enemy.position());

            let hit = look_at.angle(enemy_look)
                .map(|angle| distance < range_distance + enemy_size && angle.to_radiant() < range_angle.to_radiant())
                .unwrap_or(false);

            if hit {
                println!("Enemy hit!");
            }
        });
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

    pub fn handle_shoot(&mut self) {
        self.current_weapon.action();
    }

    pub fn notify_elapsed(&mut self, microseconds: u128) {
        self.actions.notify_elapsed(microseconds);
        self.current_weapon.notify_elapsed(microseconds);
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

        actions_ordered.sort_by(|a, b| a.z_index().total_cmp(&b.z_index()).reverse());
        actions.extend(actions_ordered.iter().map(|ordered| ordered.action().clone()));

        actions.push(build_weapons(self.view, self.current_weapon));

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

#[cfg(test)]
mod level_test {
    use std::f32::consts::PI;

    use spectral::prelude::*;

    use crate::domain::control::force::Force;
    use crate::domain::level::WALL_MINIMUM_DISTANCE;
    use crate::domain::maths::{Angle, ANGLE_0, ANGLE_90, ANGLE_RIGHT};
    use crate::domain::topology::coord::Position;
    use crate::domain::topology::map::map_test::build_map;
    use crate::domain::ui::draw_action::DrawAction;
    use crate::domain::ui::view::ViewScreen;

    use super::Level;

    const TOLERANCE: f32 = WALL_MINIMUM_DISTANCE + 0.01;

    #[test]
    fn actions_should_start_with_a_clear() {
        let view = ViewScreen::new(0, 0, ANGLE_90);
        let level = Level::new(view, build_map("r#"));

        let actions = level.generate_actions();

        assert_that!(actions.len()).is_greater_than(0);

        assert!(matches!(actions[0], DrawAction::Clear { .. }));
    }

    #[test]
    fn actions_should_draw_ceiling() {
        let view = ViewScreen::new(200, 100, ANGLE_90);
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
        let view = ViewScreen::new(200, 100, ANGLE_90);

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
        let view = ViewScreen::new(100, 100, ANGLE_90);

        let mut level = Level::new(view, map);

        level.apply_forces(Force::new(ANGLE_RIGHT, 10.0, ANGLE_0), 1000000);
        assert_that!(level.player.position().x()).is_less_than_or_equal_to(2.0);
    }

    #[test]
    fn apply_force_should_apply_not_constrained_moves() {
        let map = build_map("#r#");
        let view = ViewScreen::new(100, 100, ANGLE_90);

        let mut level = Level::new(view, map);

        level.apply_forces(Force::new(ANGLE_RIGHT, 0.2, ANGLE_0), 1000000);

        assert_that!(level.player.position().x()).is_greater_than(1.5);
    }

    #[test]
    fn enemy_should_be_in_the_list_after_the_wall_behind_him() {
        let map = build_map("#rE #");
        let view = ViewScreen::new(100, 100, ANGLE_90);

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
        let view = ViewScreen::new(100, 100, ANGLE_90);
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
        let view = ViewScreen::new(100, 100, ANGLE_90);

        let mut level = Level::new(view, map);

        level.apply_forces(
            Force::new(Angle::new(PI / 16.0), 1.0, ANGLE_0),
            10000000,
        );

        assert_that!(level.player.position().x()).is_close_to(2.0, TOLERANCE);
        assert_that!(level.player.position().y()).is_greater_than(0.5);
    }

    #[test]
    fn apply_force_should_constrains_move_by_sliding_through_the_wall_by_bottom() {
        let map = build_map("#l#");
        let view = ViewScreen::new(100, 100, ANGLE_90);

        let mut level = Level::new(view, map);

        level.apply_forces(Force::new(Angle::new(PI / 16.0), 1.0, ANGLE_0), 1000000);

        assert_that!(level.player.position().x()).is_close_to(1.0, TOLERANCE);
        assert_that!(level.player.position().y()).is_less_than(0.5);
    }

    #[test]
    fn apply_force_should_constrains_move_by_sliding_through_the_wall_by_right() {
        let map = build_map("#\nu\n#");
        let view = ViewScreen::new(100, 100, ANGLE_90);

        let mut level = Level::new(view, map);

        level.apply_forces(
            Force::new(Angle::new(-PI / 16.0), 1.0, ANGLE_0),
            1000000,
        );

        assert_that!(level.player.position().x()).is_greater_than(0.5);
        assert_that!(level.player.position().y()).is_close_to(2.0, TOLERANCE);
    }

    #[test]
    fn apply_force_should_constrains_move_by_sliding_through_the_wall_by_left() {
        let map = build_map("#\nd\n#");
        let view = ViewScreen::new(100, 100, ANGLE_90);

        let mut level = Level::new(view, map);

        level.apply_forces(
            Force::new(Angle::new(-PI / 16.0), 1.0, ANGLE_0),
            1000000,
        );

        assert_that!(level.player.position().x()).is_less_than(0.5);
        assert_that!(level.player.position().y()).is_close_to(1.0, TOLERANCE);
    }
}
