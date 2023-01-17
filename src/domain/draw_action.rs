use super::{color::Color, coord::ScreenPoint, index::TextureIndex};

pub enum DrawAction {
    Rectangle(ScreenPoint, ScreenPoint, Color),
    Line(ScreenPoint, ScreenPoint, Color),
    TexturedLine(ScreenPoint, ScreenPoint, TextureIndex, f32),
    Text(String, ScreenPoint, ScreenPoint),
    Clear(Color),
}
