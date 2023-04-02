use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::domain::topology::coord::ScreenPoint;
use crate::domain::topology::index::{FontIndex, TextureIndex};
use crate::domain::ui::draw_action::DrawAction;

use super::context::SdlContext;
use super::texture::ResourceRegistry;

pub fn draw(context: &mut SdlContext, registry: &ResourceRegistry, actions: Vec<DrawAction>) {
    let canvas = context.canvas();

    for action in actions.iter() {
        match action {
            DrawAction::Rectangle(start, end, color) => draw_rectangle(canvas, color, start, end),
            DrawAction::Line(start, end, color) => draw_line(canvas, color, start, end),
            DrawAction::TexturedLine(start, end, texture_index, position_on_texture) => {
                draw_textured_line(
                    canvas,
                    position_on_texture,
                    start,
                    end,
                    registry,
                    *texture_index,
                )
            }
            DrawAction::Clear(color) => clear_screen(canvas, color),
            DrawAction::Text(text, start, end, font) => draw_text(canvas, registry, text, start, end, font),
            DrawAction::Sprite(start, end, texture) => {
                draw_sprite(canvas, *start, *end, registry, *texture)
            }
        }
    }
}

pub fn ask_display(context: &mut SdlContext) {
    context.canvas().present();
}

fn draw_text(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    registry: &ResourceRegistry,
    text: &str,
    start: &ScreenPoint,
    end: &ScreenPoint,
    font: &FontIndex
) {
    let display_zone = to_sdl_rect(start, end);
    let font = registry.get_font(*font).unwrap();

    let surface = font
        .render(text)
        .blended(Color::RGBA(255, 0, 0, 255))
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    canvas.copy(&texture, None, Some(display_zone)).unwrap();
}

fn clear_screen(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    color: &crate::domain::ui::color::Color,
) {
    canvas.set_draw_color(to_sdl_color(color));
    canvas.clear();
}

fn draw_line(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    color: &crate::domain::ui::color::Color,
    start: &ScreenPoint,
    end: &ScreenPoint,
) {
    canvas.set_draw_color(to_sdl_color(color));
    canvas
        .draw_line(to_sdl_point(start), to_sdl_point(end))
        .expect("cannot render");
}

fn draw_textured_line(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    position_on_texture: &f32,
    start: &ScreenPoint,
    end: &ScreenPoint,
    registry: &ResourceRegistry,
    texture_index: TextureIndex,
) {
    let texture = registry
        .get_texture(texture_index)
        .expect("No texture loaded");

    let rect_texture = Rect::new(
        (texture.width() as f32 * (*position_on_texture)) as i32,
        0,
        1,
        texture.height(),
    );
    canvas
        .copy(
            texture.data(),
            Some(rect_texture),
            Some(to_sdl_rect(start, end)),
        )
        .expect("Cannot render texture");
}

fn draw_sprite(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    start: ScreenPoint,
    end: ScreenPoint,
    registry: &ResourceRegistry,
    texture_index: TextureIndex,
) {
    let texture = registry
        .get_texture(texture_index)
        .expect("No texture loaded");

    canvas
        .copy(texture.data(), None, Some(to_sdl_rect(&start, &end)))
        .expect("Cannot render a sprite");
}

fn draw_rectangle(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    color: &crate::domain::ui::color::Color,
    start: &ScreenPoint,
    end: &ScreenPoint,
) {
    canvas.set_draw_color(to_sdl_color(color));
    canvas
        .fill_rect(to_sdl_rect(start, end))
        .expect("Cannot render a rectangle");
}

fn to_sdl_color(color: &crate::domain::ui::color::Color) -> Color {
    Color::RGB(color.red(), color.green(), color.blue())
}

fn to_sdl_point(point: &ScreenPoint) -> sdl2::rect::Point {
    sdl2::rect::Point::new(point.x(), point.y())
}

fn to_sdl_rect(start: &ScreenPoint, end: &ScreenPoint) -> Rect {
    let width: u32 = (end.x() - start.x())
        .try_into()
        .expect("Unable to draw a rectangle");
    let height: u32 = (end.y() - start.y())
        .try_into()
        .expect("Unable to draw a rectangle");

    Rect::new(start.x(), start.y(), width, height)
}
