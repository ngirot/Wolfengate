use crate::domain::topology::index::{FontIndex, TextureIndex};
use crate::domain::topology::coord::ScreenPoint;
use crate::domain::ui::color::Color;

#[derive(Clone)]
pub enum DrawAction {
    Rectangle(ScreenPoint, ScreenPoint, Color),
    Line(ScreenPoint, ScreenPoint, Color),
    TexturedLine(ScreenPoint, ScreenPoint, TextureIndex, f32),
    Text(String, ScreenPoint, ScreenPoint, FontIndex),
    Clear(Color),
    Sprite(ScreenPoint, ScreenPoint, TextureIndex),
}
