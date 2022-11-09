use sdl2::{
    render::WindowCanvas,
    video::{DisplayMode, FullscreenType},
    EventPump,
};

pub struct SdlContext {
    canva: WindowCanvas,
    event_pump: EventPump,
}

impl SdlContext {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;

        let video_subsystem = sdl_context.video()?;

        let mut window = video_subsystem
            .window("Wolfengate engine", 800, 500)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");

        window.set_display_mode(DisplayMode::new(
            sdl2::pixels::PixelFormatEnum::ARGB32,
            800,
            500,
            60,
        ))?;

        let canvas = window
            .into_canvas()
            .build()
            .expect("could not make a canvas");

        let event_pump = sdl_context.event_pump()?;

        sdl_context.mouse().set_relative_mouse_mode(true);

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

    pub fn toggle_fullscreen(&mut self) {
        let state = self.canva().window().fullscreen_state();
        if state == FullscreenType::True {
            Self::change_window_full_screen_state(self, FullscreenType::Off);
        } else {
            Self::change_window_full_screen_state(self, FullscreenType::True);
        }
    }

    fn change_window_full_screen_state(sdl_context: &mut SdlContext, new_type: FullscreenType) {
        sdl_context
            .canva
            .window_mut()
            .set_fullscreen(new_type)
            .expect("Unable to change Fullscreen/windowed mode");
    }
}
