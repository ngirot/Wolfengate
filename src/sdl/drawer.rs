use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf;

use crate::domain::draw_action::DrawAction;
use crate::domain::texture::TextureIndex;

use super::context::SdlContext;
use super::texture::TextureRegistry;

pub fn draw(context: &mut SdlContext, registry: &TextureRegistry, actions: Vec<DrawAction>) {
    let canva = context.canva();

    for action in actions.iter() {
        match action {
            DrawAction::Rectangle(start, end, color) => draw_rectangle(canva, color, start, end),
            DrawAction::Line(start, end, color) => draw_line(canva, color, start, end),
            DrawAction::TexturedLine(start, end, texture_index, position_on_texture) => {
                draw_textured_line(
                    canva,
                    position_on_texture,
                    start,
                    end,
                    &registry,
                    *texture_index,
                )
            }
            DrawAction::Clear(color) => clear_screen(canva, color),
            DrawAction::Text(text, start, end) => draw_text(canva, text, start, end)
        }
    }
}

pub fn ask_display(context: &mut SdlContext) {
    context.canva().present();
}

fn draw_text(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
             text: &str,
             start: &crate::domain::coord::ScreenPoint,
             end: &crate::domain::coord::ScreenPoint) {
    let display_zone = to_sdl_rect(start, end);
    let ttf_context = ttf::init().unwrap();
    let font = ttf_context.load_font("res/MontserratAlternates-Medium.otf", 128).unwrap();

    let surface = font.render(&text).blended(Color::RGBA(255, 0, 0, 255)).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    canvas.copy(&texture, None, Some(display_zone)).unwrap();
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

fn draw_textured_line(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    position_on_texture: &f32,
    start: &crate::domain::coord::ScreenPoint,
    end: &crate::domain::coord::ScreenPoint,
    texture_registry: &TextureRegistry,
    texture_index: TextureIndex,
) {
    let texture = texture_registry
        .get(texture_index)
        .expect("No texture loaded");

    let rect_texture = Rect::new(
        (texture.width() as f32 * (*position_on_texture)) as i32,
        0,
        1,
        texture.height(),
    );
    canvas
        .copy(
            &texture.data(),
            Some(rect_texture),
            Some(to_sdl_rect(start, end)),
        )
        .expect("Cannot render texture");
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

fn to_sdl_color(color: &crate::domain::color::Color) -> Color {
    Color::RGB(color.red(), color.green(), color.blue())
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
