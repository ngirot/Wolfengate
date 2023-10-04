use crate::domain::maths::{Angle, ANGLE_90};
use crate::domain::topology::coord::Position;

#[derive(Copy, Clone)]
pub struct ViewScreen {
    height: u16,
    width: u16,
    angle: Angle,
    ratio: f32,
}

impl ViewScreen {
    pub fn new(height: u16, width: u16, angle: Angle) -> Self {
        Self {
            height,
            width,
            angle,
            ratio: height as f32 / width as f32,
        }
    }

    pub fn height(&self) -> i32 {
        self.height as i32
    }

    pub fn width(&self) -> i32 {
        self.width as i32
    }

    pub fn angle(&self) -> Angle {
        self.angle
    }

    pub fn ratio(&self) -> f32 {
        self.ratio
    }

    pub fn view_plane(&self, orientation: &Angle) -> Position {
        let norm = self.angle.multiplication(0.5).tan();
        let direction = orientation.addition(ANGLE_90);
        // let plane_position = plane.to_position(Angle::new(view.angle().to_radiant()/2.0).tan());
        Position::new(direction.cos()*norm, direction.sin()*norm)
    }
}


#[cfg(test)]
mod viewscreen_test {
    use std::f32::consts::PI;

    use spectral::prelude::*;

    use crate::domain::maths::{Angle, ANGLE_90};
    use crate::domain::ui::view::ViewScreen;

    #[test]
    fn should_get_plane_from_90_degrees_view() {
        let view = ViewScreen::new(400, 400, ANGLE_90);

        let plane = view.view_plane(&ANGLE_90);

        assert_that!(plane.x()).is_close_to(-1.0, 0.001);
        assert_that!(plane.y()).is_close_to(0.0, 0.001);
    }

    #[test]
    fn should_get_plane_on_very_large_view() {
        let view = ViewScreen::new(400, 400, Angle::new(PI - 0.01));

        let plane = view.view_plane(&ANGLE_90);

        assert_that!(plane.x()).is_close_to(-200.0, 0.001);
        assert_that!(plane.y()).is_close_to(0.0, 0.001);
    }

    #[test]
    fn should_get_plane_on_very_narrow_view() {
        let view = ViewScreen::new(400, 400, Angle::new(0.01));

        let plane = view.view_plane(&ANGLE_90);

        assert_that!(plane.x()).is_close_to(-0.005, 0.001);
        assert_that!(plane.y()).is_close_to(0.0, 0.001);
    }
}

