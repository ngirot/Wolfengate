use super::{color::Color, point::Point};

pub enum DrawAction {
    Rectangle(Point, Point, Color),
    Line(Point, Point, Color),
    Clear(Color),
}