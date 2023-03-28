use crate::domain::index::FontIndex;
use super::{color::Color, coord::ScreenPoint, index::TextureIndex};

#[derive(Clone)]
pub enum DrawAction {
    Rectangle(ScreenPoint, ScreenPoint, Color),
    Line(ScreenPoint, ScreenPoint, Color),
    TexturedLine(ScreenPoint, ScreenPoint, TextureIndex, f32),
    Text(String, ScreenPoint, ScreenPoint, FontIndex),
    Clear(Color),
    Sprite(ScreenPoint, ScreenPoint, TextureIndex),
}
