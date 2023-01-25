use std::f32::consts::PI;
use std::time::Instant;

use sdl2::ttf;

use wolfengate::domain::actor::Player;
use wolfengate::domain::coord::Position;
use wolfengate::domain::debug::DebugInfo;
use wolfengate::domain::force::InputForce;
use wolfengate::domain::index::{FontIndex, TextureIndex};
use wolfengate::domain::input::Input;
use wolfengate::domain::level::Level;
use wolfengate::domain::map::Map;
use wolfengate::sdl::context::SdlContext;
use wolfengate::sdl::drawer;
use wolfengate::sdl::drawer::ask_display;
use wolfengate::sdl::input::poll_input;
use wolfengate::sdl::texture::ResourceRegistry;

fn render(context: &mut SdlContext, level: &Level, debug_info: &DebugInfo, registry: &ResourceRegistry) {
    let actions = level.generate_actions();
    drawer::draw(context, registry, actions);
    drawer::draw(context, registry, debug_info.generate_actions());
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
    let player = Player::new(position, PI / 2.0);
    let mut level = Level::new(width, height, map, player);
    let mut debug_info = DebugInfo::new();

    let mut sdl_context = SdlContext::new(width, height)?;
    let texture_creator = sdl_context.canvas().texture_creator();
    let ttf_creator = ttf::init().unwrap();
    let mut registry = ResourceRegistry::new(&texture_creator, &ttf_creator);
    registry.load_texture(TextureIndex::WALL, String::from("wall.png"));
    registry.load_texture(TextureIndex::VOID, String::from("transparency.png"));
    registry.load_texture(TextureIndex::ENEMY, String::from("enemy.png"));
    registry.load_font(FontIndex::MONTSERRAT, String::from("MontserratAlternates-Medium.otf"));

    let mut start = Instant::now();
    'running: loop {
        let elapsed = start.elapsed().as_micros();
        start = Instant::now();
        for input in poll_input(&mut sdl_context) {
            match input {
                Input::Quit => break 'running,
                Input::Forward => level.apply_forces(input_force.forward(elapsed)),
                Input::Backward => level.apply_forces(input_force.backward(elapsed)),
                Input::StrafeLeft => level.apply_forces(input_force.strafe_left(elapsed)),
                Input::StrafeRight => level.apply_forces(input_force.state_right(elapsed)),
                Input::Rotate(x) => level.apply_forces(input_force.rotate(x)),
                Input::ToggleFullscreen => sdl_context.toggle_fullscreen(),
                Input::ShowFps => { debug_info = debug_info.toggle_fps() }
            }
        }

        // Render
        render(&mut sdl_context, &level, &debug_info, &registry);
        debug_info = debug_info.with_another_frame_displayed(elapsed);
    }

    Ok(())
}
