use std::f32::consts::PI;

use super::{
    color::Color,
    distance::distance,
    draw_action::DrawAction,
    map::Map,
    point::{Position, ScreenPoint},
};

pub struct Level {
    screen_height: u16,
    screen_width: u16,
    position: Position,
    map: Map,
}

impl Level {
    pub fn new(screen_width: u16, screen_height: u16, position: Position, map: Map) -> Self {
        Self {
            screen_height,
            screen_width,
            position,
            map,
        }
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
            &self.position,
            &self.map,
        ));

        actions
    }

    pub fn rotate_right(&mut self) {
        self.position = self.position.with_angle(self.position.angle() - 0.05);
    }

    pub fn rotate_left(&mut self) {
        self.position = self.position.with_angle(self.position.angle() + 0.05);
    }

    pub fn forward(&mut self) {
        self.position = self
            .position
            .with_x(self.position.x() + self.position.angle().cos())
            .with_y(self.position.y() + self.position.angle().sin())
    }

    pub fn backward(&mut self) {
        self.position = self
            .position
            .with_x(self.position.x() - self.position.angle().cos())
            .with_y(self.position.y() - self.position.angle().sin())
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

fn build_walls(width: u16, height: u16, position: &Position, map: &Map) -> Vec<DrawAction> {
    let mut actions = vec![];

    let angle = PI / 2.0; // 45° fov
    let min = position.angle() + (angle / 2.0);
    let step = angle / width as f32;

    'drawer: for i in 0..width {
        let mult = min - step * i as f32;

        let pos = position.with_angle(mult);
        let distance_option = distance(pos, map);
        if distance_option.is_none() {
            break 'drawer;
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
    use crate::domain::{draw_action::DrawAction, map::Map, point::Position};

    use super::Level;
    use spectral::prelude::*;

    #[test]
    fn actions_should_start_with_a_clear() {
        let level = Level::new(0, 0, Position::new(0.0, 0.0, 0.0), Map::new("#"));

        let actions = level.generate_actions();

        assert_that(&actions.len()).is_greater_than(0);

        assert!(matches!(actions[0], DrawAction::Clear { .. }));
    }

    #[test]
    fn actions_should_draw_ceiling() {
        let level = Level::new(100, 200, Position::new(0.0, 0.0, 0.0), Map::new("#"));
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
        let level = Level::new(100, 200, Position::new(0.0, 0.0, 0.0), Map::new("#"));
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
}
