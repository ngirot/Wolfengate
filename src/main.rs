use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use std::time::Duration;

fn render_clear(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
}

fn render_background(canvas: &mut WindowCanvas, width: u32, height: u32) {
    canvas.set_draw_color(Color::RGB(50, 50, 50));
    let mid_screen = height/2;
    canvas.fill_rect(Rect::new(0, 0, width, mid_screen))
        .expect("Cannot render ceiling");
        
    canvas.set_draw_color(Color::RGB(100, 100, 100));
    canvas.fill_rect(Rect::new(0, (mid_screen+1).try_into().unwrap(), width, height))
        .expect("Cannot render floor");
}

fn render_walls(canvas: &mut WindowCanvas, width: i32, height: i32) {

    for i in 0..width {
        let distance = if i < 300 {3} else if i>600 {2} else {1};
        canvas.set_draw_color(Color::RGB(0, 0, 255/(distance)));

        let biais: i32 = (distance*75).into();
        let start = Point::new(i, biais);
        let end = Point::new(i, height-biais);

        canvas.draw_line(start, end).expect("cannot render");
    }
}

fn render_ask_display(canvas: &mut WindowCanvas) {
    canvas.present();
}

fn render(canvas: &mut WindowCanvas) {
    render_clear(canvas);
    render_background(canvas, 800, 500);
    render_walls(canvas, 800, 500);
    render_ask_display(canvas);
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Wolfengate engine", 800, 500)
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