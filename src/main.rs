use domain::level::Level;
use domain::map::Map;
use domain::point::Position;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use std::f32::consts::PI;
use std::time::Duration;

mod domain;
mod sdl;

use crate::domain::level;
use crate::sdl::drawer;

fn render_ask_display(canvas: &mut WindowCanvas) {
    canvas.present();
}

fn render(canvas: &mut WindowCanvas, level: &Level) {
    drawer::draw(canvas, level.generate_actions());
    render_ask_display(canvas);
}

fn main() -> Result<(), String> {
    let map = Map::new(
        "\"
        ################\n\
        ################\n\
        ##            ##\n\
        ##            ##\n\
        ##        #   ##\n\
        ##        #   ##\n\
        ##        #   ##\n\
        ##        #   ##\n\
        ##        #   ##\n\
        ################\n\
        ################",
    );
    let position = Position::new(12.0, 3.0, PI/2.0);

    let mut level = level::Level::new(800, 500, position, map);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Wolfengate engine", 800, 500)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Z), ..} => {level.forward()}
                | Event::KeyDown { keycode: Some(Keycode::S), ..} => {level.backward()}
                | Event::KeyDown { keycode: Some(Keycode::Q), ..} => {level.rotate_left()}
                | Event::KeyDown { keycode: Some(Keycode::D), ..} => {level.rotate_right()}
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Render
        render(&mut canvas, &level);

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
