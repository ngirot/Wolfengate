use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use super::context::SdlContext;

pub enum Input {
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
    Rotate(i32),
    ToggleFullscreen,
    Quit,
}

pub fn poll_input(sdl_context: &mut SdlContext) -> Vec<Input> {
    let mut inputs = vec![];

    let event_pump = sdl_context.event_pump();

    let keys: Vec<Keycode> = event_pump
        .keyboard_state()
        .pressed_scancodes()
        .filter_map(Keycode::from_scancode)
        .collect();

    let mut alt_pressed = false;

    for key in keys {
        match key {
            Keycode::W => inputs.push(Input::Forward),
            Keycode::Z => inputs.push(Input::Forward),

            Keycode::S => inputs.push(Input::Backward),

            Keycode::Q => inputs.push(Input::StrafeLeft),
            Keycode::A => inputs.push(Input::StrafeLeft),

            Keycode::D => inputs.push(Input::StrafeRight),

            Keycode::LAlt => alt_pressed = true,

            _ => (),
        }
    }

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => inputs.push(Input::Quit),
            Event::MouseMotion { xrel, .. } => inputs.push(Input::Rotate(xrel)),
            Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } => {
                if alt_pressed {
                    inputs.push(Input::ToggleFullscreen);
                }
            }
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
