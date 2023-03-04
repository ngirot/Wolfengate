use std::time::Instant;

use sdl2::ttf;

use wolfengate::domain::actor::{AccelerationStats, Enemy, Player, PlayerStats, SpeedStats};
use wolfengate::domain::coord::Position;
use wolfengate::domain::debug::DebugInfo;
use wolfengate::domain::force::{Force, InputForce};
use wolfengate::domain::index::{FontIndex, TextureIndex};
use wolfengate::domain::input::Input;
use wolfengate::domain::level::Level;
use wolfengate::domain::map::Map;
use wolfengate::domain::maths::{ANGLE_RIGHT, ANGLE_UP};
use wolfengate::domain::view::ViewScreen;
use wolfengate::sdl::context::SdlContext;
use wolfengate::sdl::drawer;
use wolfengate::sdl::drawer::ask_display;
use wolfengate::sdl::input::poll_input;
use wolfengate::sdl::texture::ResourceRegistry;

fn render(
    context: &mut SdlContext,
    level: &Level,
    debug_info: &DebugInfo,
    registry: &ResourceRegistry,
) {
    let actions = level.generate_actions();
    drawer::draw(context, registry, actions);
    drawer::draw(context, registry, debug_info.generate_actions());
    ask_display(context);
}

fn main() -> Result<(), String> {
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
    let acceleration = AccelerationStats::new(70.0);
    let deceleration = AccelerationStats::new(40.0);
    let max_speed = SpeedStats::new(6.0);
    let player_stats = PlayerStats::new(acceleration, deceleration, max_speed);
    let player = Player::new(position, ANGLE_UP, player_stats);
    let enemy = Enemy::new(Position::new(5.0, 5.0));
    let view = ViewScreen::new(500, 800);
    let mut level = Level::new(view, map, player, Some(enemy));
    let mut debug_info = DebugInfo::new();

    let mut sdl_context = SdlContext::new(view)?;
    let texture_creator = sdl_context.canvas().texture_creator();
    let ttf_creator = ttf::init().unwrap();
    let mut registry = ResourceRegistry::new(&texture_creator, &ttf_creator);
    registry.load_texture(TextureIndex::WALL, String::from("wall.png"));
    registry.load_texture(TextureIndex::VOID, String::from("transparency.png"));
    registry.load_texture(TextureIndex::ENEMY, String::from("enemy.png"));
    registry.load_font(
        FontIndex::MONTSERRAT,
        String::from("MontserratAlternates-Medium.otf"),
    );

    let mut start = Instant::now();
    'running: loop {
        let elapsed = start.elapsed().as_micros();
        start = Instant::now();
        let mut current_force = Force::new(ANGLE_RIGHT, 0.0, ANGLE_RIGHT);
        for input in poll_input(&mut sdl_context) {
            match input {
                Input::Quit => break 'running,
                Input::Forward => current_force = current_force.add(input_force.forward()),
                Input::Backward => current_force = current_force.add(input_force.backward()),
                Input::StrafeLeft => current_force = current_force.add(input_force.strafe_left()),
                Input::StrafeRight => current_force = current_force.add(input_force.state_right()),
                Input::Rotate(x) => current_force = current_force.add(input_force.rotate(x)),
                Input::ToggleFullscreen => sdl_context.toggle_fullscreen(),
                Input::ShowFps => debug_info = debug_info.toggle_fps(),
            }
        }

        level.apply_forces(current_force, elapsed);

        // Render
        render(&mut sdl_context, &level, &debug_info, &registry);
        debug_info = debug_info.with_another_frame_displayed(elapsed);
    }

    Ok(())
}
