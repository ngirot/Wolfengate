use super::{color::Color, point::ScreenPoint};

pub enum DrawAction {
    Rectangle(ScreenPoint, ScreenPoint, Color),
    Line(ScreenPoint, ScreenPoint, Color),
    Clear(Color),
}
