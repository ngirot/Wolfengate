use std::time::Instant;

use sdl2::ttf;

use wolfengate::domain::control::force::{Force, InputForce};
use wolfengate::domain::control::input::Input;
use wolfengate::domain::level::Level;
use wolfengate::domain::maths::{ANGLE_0, ANGLE_90, ANGLE_RIGHT};
use wolfengate::domain::resources::ResourceLoader;
use wolfengate::domain::topology::map::Map;
use wolfengate::domain::ui::debug::DebugInfo;
use wolfengate::domain::ui::view::ViewScreen;
use wolfengate::infrastructure::fs::filesystem::{load_as_binary, load_as_file};
use wolfengate::infrastructure::fs::json::load_configuration;
use wolfengate::infrastructure::sdl::context::SdlContext;
use wolfengate::infrastructure::sdl::drawer;
use wolfengate::infrastructure::sdl::drawer::ask_display;
use wolfengate::infrastructure::sdl::input::poll_input;
use wolfengate::infrastructure::sdl::texture::{ResourceRegistryLoader, ResourceRegistry};

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
    let view = ViewScreen::new(500, 800, ANGLE_90);
    let mut sdl_context = SdlContext::new(view)?;
    let texture_creator = sdl_context.canvas().texture_creator();
    let ttf_creator = ttf::init().unwrap();
    let resource_loader = ResourceLoader::new(load_as_binary, load_as_file);

    let mut registry = ResourceRegistry::new(&texture_creator, &ttf_creator, &resource_loader);
    let debug_font = registry.load_font(String::from("MontserratAlternates-Medium.otf"));

    let map = map_loader(&mut registry, resource_loader);

    let input_force = InputForce::new(0.004, 0.005);
    let mut level = Level::new(view, map);
    let mut debug_info = DebugInfo::new(debug_font);

    let mut start = Instant::now();
    'running: loop {
        let elapsed = start.elapsed().as_micros();
        start = Instant::now();
        let mut current_force = Force::new(ANGLE_RIGHT, 0.0, ANGLE_0);
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
                Input::Shoot => level.handle_shoot(),
                Input::ShowFps => debug_info = debug_info.toggle_fps(),
            }
        }

        level.notify_elapsed(elapsed);
        level.apply_forces(current_force, elapsed);
        level.apply_shoots();

        // Render
        render(&mut sdl_context, &level, &debug_info, &registry);
        debug_info = debug_info.with_another_frame_displayed(elapsed);
    }

    Ok(())
}

pub fn map_loader(registry: &mut ResourceRegistry, resource_loader: ResourceLoader) -> Map {
    let configuration_content = resource_loader.load_as_string(String::from("conf.json"));
    let configuration = load_configuration(configuration_content, registry);

    let map_content = resource_loader.load_as_string(String::from("1.map"));

    Map::new(
        &map_content,
        configuration)
        .unwrap()
}
