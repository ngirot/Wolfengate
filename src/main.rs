use std::f32::consts::PI;
use std::time::Duration;
use wolfengate::domain::actor::{ActorStats, Player};
use wolfengate::domain::coord::Position;
use wolfengate::domain::input::Input;
use wolfengate::domain::level::Level;
use wolfengate::domain::map::Map;
use wolfengate::sdl::context::SdlContext;
use wolfengate::sdl::drawer::ask_display;
use wolfengate::sdl::input::poll_input;

use wolfengate::domain::level;
use wolfengate::sdl::drawer;

fn render(context: &mut SdlContext, level: &Level, player: &Player) {
    let actions = level.generate_actions(*player.position(), player.orientation());
    drawer::draw(context, actions);
    ask_display(context);
}

fn main() -> Result<(), String> {
    let map = Map::new(
        "\
        ##############\n\
        #            #\n\
        #            #\n\
        #        #   #\n\
        #        #   #\n\
        #        #    \n\
        #        #   #\n\
        #        #   #\n\
        ##############",
    )
    .unwrap();
    let position = Position::new(12.0, 3.0);
    let player_stats = ActorStats::new(0.2, 0.005);
    let mut player = Player::new(position, PI / 2.0, player_stats);
    let level = level::Level::new(800, 500, map);

    let mut sdl_context = SdlContext::new()?;

    'running: loop {
        for input in poll_input(&mut sdl_context) {
            match input {
                Input::Quit => break 'running,
                Input::Forward => player = player.move_forward(),
                Input::Backward => player = player.move_backward(),
                Input::StrafeLeft => player = player.move_left(),
                Input::StrafeRight => player = player.move_right(),
                Input::Rotate(x) => player = player.rotate(x),
                Input::ToggleFullscreen => sdl_context.toggle_fullscreen(),
            }
        }

        // Render
        render(&mut sdl_context, &level, &player);

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
