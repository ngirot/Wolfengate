use crate::domain::coord::ScreenPoint;
use crate::domain::draw_action::DrawAction;

const MARGIN: i32 = 3;

pub struct DebugInfo {
    elapsed_time_in_microseconds: u128,
    frame_displayed: u128,
    last_fps: u128,
    display_fps: bool,
}

impl DebugInfo {
    pub fn new() -> Self {
        Self { elapsed_time_in_microseconds: 0, frame_displayed: 0, last_fps: 0, display_fps: false }
    }

    pub fn generate_actions(&self) -> Vec<DrawAction> {
        if self.display_fps && self.last_fps != 0 {
            let fps = format!("{} fps", self.last_fps);
            vec![DrawAction::Text(fps, ScreenPoint::new(MARGIN, 0), ScreenPoint::new(100, 50))]
        } else {
            vec![]
        }
    }

    pub fn toggle_fps(&self) -> Self {
        Self {
            elapsed_time_in_microseconds: self.elapsed_time_in_microseconds,
            frame_displayed: self.frame_displayed,
            last_fps: self.last_fps,
            display_fps: !self.display_fps,
        }
    }

    pub fn with_another_frame_displayed(&self, elapsed_time_in_microseconds: u128) -> Self {
        if self.elapsed_time_in_microseconds > 500000 {
            let fps = self.frame_displayed * 1000000 / self.elapsed_time_in_microseconds;
            Self {
                elapsed_time_in_microseconds: 0,
                frame_displayed: 0,
                last_fps: fps,
                display_fps: self.display_fps,
            }
        } else {
            Self {
                elapsed_time_in_microseconds: self.elapsed_time_in_microseconds + elapsed_time_in_microseconds,
                frame_displayed: self.frame_displayed + 1,
                last_fps: self.last_fps,
                display_fps: self.display_fps,
            }
        }
    }
}