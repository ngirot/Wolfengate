use sdl2::{render::WindowCanvas, EventPump};

pub struct SdlContext {
    canva: WindowCanvas,
    event_pump: EventPump,
}

impl SdlContext {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Wolfengate engine", 800, 500)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");

        let canvas = window
            .into_canvas()
            .build()
            .expect("could not make a canvas");

        let event_pump = sdl_context.event_pump()?;

        Ok(SdlContext {
            canva: canvas,
            event_pump,
        })
    }

    pub fn canva(&mut self) -> &mut WindowCanvas {
        &mut self.canva
    }

    pub fn event_pump(&mut self) -> &mut EventPump {
        &mut self.event_pump
    }
}
