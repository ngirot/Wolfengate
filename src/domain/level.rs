use super::{color::Color, draw_action::DrawAction, point::Point};

pub struct Level {
    screen_height: u16,
    screen_width: u16,
}

impl Level {
    pub fn new(screen_width: u16, screen_height: u16) -> Self {
        Self {
            screen_height,
            screen_width,
        }
    }

    pub fn generate_actions(&self) -> Vec<DrawAction> {
        let mut actions: Vec<DrawAction> = vec![];

        actions.extend(build_clear_actions());

        actions.extend(build_background_actions(
            self.screen_width,
            self.screen_height,
        ));

        actions.extend(build_walls(self.screen_width, self.screen_height));

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
            Point::new(0, 0),
            Point::new(width, mid_screen),
            Color::new(50, 50, 50),
        ),
        DrawAction::Rectangle(
            Point::new(0, mid_screen),
            Point::new(width, height),
            Color::new(100, 100, 100),
        ),
    ]
}

fn build_walls(width: u16, height: u16) -> Vec<DrawAction> {
    let mut actions = vec![];
    for i in 0..width {
        let distance = if i < 300 {
            3
        } else if i > 600 {
            2
        } else {
            1
        };

        let column: i32 = i.into();
        let screen_length: i32 = height.into();

        let biais: i32 = (distance * 75).into();
        let start = Point::new(column, biais);
        let end = Point::new(column, screen_length - biais);
        let color = Color::new(0, 0, 255 / distance);

        actions.push(DrawAction::Line(start, end, color));
    }
    actions
}

#[cfg(test)]
mod level_test {
    use crate::domain::draw_action::DrawAction;

    use super::Level;
    use spectral::prelude::*;

    #[test]
    fn actions_should_start_with_a_clear() {
        let level = Level::new(100, 100);

        let actions = level.generate_actions();

        assert_that(&actions.len()).is_greater_than(0);

        assert!(matches!(actions[0], DrawAction::Clear { .. }));
    }

    #[test]
    fn actions_should_draw_ceiling() {
        let level = Level::new(100, 200);
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
        let level = Level::new(100, 200);
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
