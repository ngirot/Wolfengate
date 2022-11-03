use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use super::context::SdlContext;

pub enum Input {
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
    Rotate(i32),
    Quit,
}

pub fn poll_input(sdl_context: &mut SdlContext) -> Vec<Input> {
    let mut inputs = vec![];

    let event_pump = sdl_context.event_pump();
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Z),
                ..
            } => inputs.push(Input::Forward),
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => inputs.push(Input::Backward),
            Event::MouseMotion { xrel, .. } => inputs.push(Input::Rotate(xrel)),
            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => inputs.push(Input::StrafeLeft),
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => inputs.push(Input::StrafeRight),
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                inputs.push(Input::Quit);
            }
            _ => {}
        }
    }

    inputs
}
