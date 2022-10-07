use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use std::time::Duration;

fn render(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(50, 50, 50));
    canvas.fill_rect(Rect::new(0, 0, 800, 250))
        .expect("Cannot render background");
        
    canvas.set_draw_color(Color::RGB(100, 100, 100));
    canvas.fill_rect(Rect::new(0, 251, 800, 500))
        .expect("Cannot render background");


    for i in 0..800 {
        let distance = if i < 300 {3} else if i>600 {2} else {1};
        canvas.set_draw_color(Color::RGB(0, 0, 255/(distance)));

        let screen_length = 500;
        let biais: i32 = (distance*75).into();
        let start = Point::new(i, biais);
        let end = Point::new(i, screen_length-biais);

        canvas.draw_line(start, end).expect("cannot render");

        
    }

    canvas.present();
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("wolfy engine", 800, 500)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        // Render
        render(&mut canvas);

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}