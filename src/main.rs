use std::f32::consts::PI;
use std::time::Instant;

use wolfengate::domain::actor::Player;
use wolfengate::domain::coord::Position;
use wolfengate::domain::force::InputForce;
use wolfengate::domain::input::Input;
use wolfengate::domain::level;
use wolfengate::domain::level::Level;
use wolfengate::domain::map::Map;
use wolfengate::sdl::context::SdlContext;
use wolfengate::sdl::drawer;
use wolfengate::sdl::drawer::ask_display;
use wolfengate::sdl::input::poll_input;

fn render(context: &mut SdlContext, level: &Level, player: &Player) {
    let actions = level.generate_actions(*player.position(), player.orientation());
    drawer::draw(context, actions);
    ask_display(context);
}

fn main() -> Result<(), String> {
    let width = 800;
    let height = 500;

    let map = Map::new(
        "\
        ##############\n\
        #      #     #\n\
        #  #   #######\n\
        #  #     #   #\n\
        #  ####  # ###\n\
        #     #      #\n\
        #### #########\n\
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
    let input_force = InputForce::new(0.004, 0.005);
    let mut player = Player::new(position, PI / 2.0);
    let level = level::Level::new(width, height, map);

    let mut sdl_context = SdlContext::new(width, height)?;

    let mut start = Instant::now();
    'running: loop {
        let elapsed = start.elapsed().as_millis();
        start = Instant::now();
        for input in poll_input(&mut sdl_context) {
            match input {
                Input::Quit => break 'running,
                Input::Forward => player = player.apply_force(input_force.forward(elapsed)),
                Input::Backward => player = player.apply_force(input_force.backward(elapsed)),
                Input::StrafeLeft => player = player.apply_force(input_force.strafe_left(elapsed)),
                Input::StrafeRight => player = player.apply_force(input_force.state_right(elapsed)),
                Input::Rotate(x) => player = player.apply_force(input_force.rotate(x)),
                Input::ToggleFullscreen => sdl_context.toggle_fullscreen(),
            }
        }

        // Render
        render(&mut sdl_context, &level, &player);
    }

    Ok(())
}
