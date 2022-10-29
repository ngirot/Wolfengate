use sdl2::rect::Rect;

use crate::domain::draw_action::DrawAction;

use super::context::SdlContext;

pub fn draw(context: &mut SdlContext, actions: Vec<DrawAction>) {
    let canva = context.canva();

    for action in actions.iter() {
        match action {
            DrawAction::Rectangle(start, end, color) => draw_rectangle(canva, color, start, end),
            DrawAction::Line(start, end, color) => draw_line(canva, color, start, end),
            DrawAction::Clear(color) => clear_screen(canva, color),
        }
    }
}

pub fn ask_display(context: &mut SdlContext) {
    context.canva().present();
}

fn clear_screen(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    color: &crate::domain::color::Color,
) {
    canvas.set_draw_color(to_sdl_color(color));
    canvas.clear();
}

fn draw_line(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    color: &crate::domain::color::Color,
    start: &crate::domain::coord::ScreenPoint,
    end: &crate::domain::coord::ScreenPoint,
) {
    canvas.set_draw_color(to_sdl_color(color));
    canvas
        .draw_line(to_sdl_point(start), to_sdl_point(end))
        .expect("cannot render");
}

fn draw_rectangle(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    color: &crate::domain::color::Color,
    start: &crate::domain::coord::ScreenPoint,
    end: &crate::domain::coord::ScreenPoint,
) {
    canvas.set_draw_color(to_sdl_color(color));
    canvas
        .fill_rect(to_sdl_rect(start, end))
        .expect("Cannot render a rectangle");
}

fn to_sdl_color(color: &crate::domain::color::Color) -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGB(color.red(), color.green(), color.blue())
}

fn to_sdl_point(point: &crate::domain::coord::ScreenPoint) -> sdl2::rect::Point {
    sdl2::rect::Point::new(point.x(), point.y())
}

fn to_sdl_rect(
    start: &crate::domain::coord::ScreenPoint,
    end: &crate::domain::coord::ScreenPoint,
) -> Rect {
    let width: u32 = (end.x() - start.x())
        .try_into()
        .expect("Unable to draw a rectable");
    let height: u32 = (end.y() - start.y())
        .try_into()
        .expect("Unable to draw a rectable");

    Rect::new(start.x(), start.y(), width, height)
}
