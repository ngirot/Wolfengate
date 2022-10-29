use super::{color::Color, coord::ScreenPoint};

pub enum DrawAction {
    Rectangle(ScreenPoint, ScreenPoint, Color),
    Line(ScreenPoint, ScreenPoint, Color),
    Clear(Color),
}
