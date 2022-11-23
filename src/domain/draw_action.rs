use super::{color::Color, coord::ScreenPoint, texture::TextureIndex};

pub enum DrawAction {
    Rectangle(ScreenPoint, ScreenPoint, Color),
    Line(ScreenPoint, ScreenPoint, Color),
    TexturedLine(ScreenPoint, ScreenPoint, TextureIndex, f32),
    Clear(Color),
}
