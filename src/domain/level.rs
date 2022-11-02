use std::f32::consts::PI;

use super::{
    color::Color,
    coord::{Position, ScreenPoint},
    distance::distance,
    draw_action::DrawAction,
    map::Map,
};

pub struct Level {
    screen_height: u16,
    screen_width: u16,
    map: Map,
}

impl Level {
    pub fn new(screen_width: u16, screen_height: u16, map: Map) -> Self {
        Self {
            screen_height,
            screen_width,
            map,
        }
    }

    pub fn generate_actions(&self, position: Position, angle: f32) -> Vec<DrawAction> {
        let mut actions: Vec<DrawAction> = vec![];

        actions.extend(build_clear_actions());

        actions.extend(build_background_actions(
            self.screen_width,
            self.screen_height,
        ));

        actions.extend(build_walls(
            self.screen_width,
            self.screen_height,
            &position,
            angle,
            &self.map,
        ));

        actions
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
        let distance = distance_option.unwrap();

        let column: i32 = i.into();
        let screen_length: i32 = height.into();

        let wall_height = (height as f32 * 0.8) / distance;
        let start = ScreenPoint::new(
            column,
            (screen_length as f32 / 2.0 - wall_height / 2.0) as i32,
        );
        let end = ScreenPoint::new(
            column,
            (screen_length as f32 / 2.0 + wall_height / 2.0) as i32,
        );
        let color = Color::new(0, 0, (255.0 / (distance / 2.0)) as u8);

        actions.push(DrawAction::Line(start, end, color));
    }
    actions
}

#[cfg(test)]
mod level_test {
    use crate::domain::{coord::Position, draw_action::DrawAction, map::Map};

    use super::Level;
    use spectral::prelude::*;

    #[test]
    fn actions_should_start_with_a_clear() {
        let level = Level::new(0, 0, Map::new("#"));

        let actions = level.generate_actions(Position::new(0.0, 0.0), 0.0);

        assert_that(&actions.len()).is_greater_than(0);

        assert!(matches!(actions[0], DrawAction::Clear { .. }));
    }

    #[test]
    fn actions_should_draw_ceiling() {
        let level = Level::new(100, 200, Map::new("#"));
        let mut found = false;

        let actions = level.generate_actions(Position::new(0.0, 0.0), 0.0);

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
        let level = Level::new(100, 200, Map::new("#"));
        let mut found = false;

        let actions = level.generate_actions(Position::new(0.0, 0.0), 0.0);

        for action in actions {
            if let DrawAction::Rectangle(start, end, _) = action {
                if start.x() == 0 && start.y() == 100 && end.x() == 100 && end.y() == 200 {
                    found = true
                }
            }
        }

        assert_that(&found).is_true();
    }
}
