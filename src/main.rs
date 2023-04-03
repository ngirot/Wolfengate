use std::time::Instant;

use sdl2::ttf;

use wolfengate::domain::actors::actor::{AccelerationStats, Enemy, Player, PlayerStats, SpeedStats};
use wolfengate::domain::topology::coord::Position;
use wolfengate::domain::control::force::{Force, InputForce};
use wolfengate::domain::control::input::Input;
use wolfengate::domain::level::Level;
use wolfengate::domain::loader::map_loader;
use wolfengate::domain::maths::{ANGLE_RIGHT, ANGLE_UP};
use wolfengate::domain::resources::ResourceLoader;
use wolfengate::domain::ui::debug::DebugInfo;
use wolfengate::domain::ui::view::ViewScreen;
use wolfengate::fs::filesystem::{load_as_binary, load_as_file};
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
    let view = ViewScreen::new(500, 800);
    let mut sdl_context = SdlContext::new(view)?;
    let texture_creator = sdl_context.canvas().texture_creator();
    let ttf_creator = ttf::init().unwrap();
    let resource_loader = ResourceLoader::new(load_as_binary, load_as_file);

    let mut registry = ResourceRegistry::new(&texture_creator, &ttf_creator, &resource_loader);
    let enemy_texture = registry.load_texture(String::from("enemy.png"));
    let debug_font = registry.load_font(String::from("MontserratAlternates-Medium.otf"));

    let map = map_loader(&mut registry, resource_loader);

    let position = Position::new(12.0, 3.0);
    let input_force = InputForce::new(0.004, 0.005);
    let acceleration = AccelerationStats::new(70.0);
    let deceleration = AccelerationStats::new(40.0);
    let max_speed = SpeedStats::new(6.0);
    let player_stats = PlayerStats::new(acceleration, deceleration, max_speed);
    let player = Player::new(position, ANGLE_UP, player_stats);
    let enemy = Enemy::new(enemy_texture, Position::new(5.0, 5.0));
    let mut level = Level::new(view, map, player, Some(enemy));
    let mut debug_info = DebugInfo::new(debug_font);

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
                Input::Action => level.handle_action(),
                Input::ShowFps => debug_info = debug_info.toggle_fps(),
            }
        }

        level.notify_elapsed(elapsed);
        level.apply_forces(current_force, elapsed);

        // Render
        render(&mut sdl_context, &level, &debug_info, &registry);
        debug_info = debug_info.with_another_frame_displayed(elapsed);
    }

    Ok(())
}
