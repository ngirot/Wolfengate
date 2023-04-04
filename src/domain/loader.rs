use crate::domain::resources::ResourceLoader;
use crate::domain::topology::map::Map;
use crate::fs::json::load_configuration;
use crate::sdl::texture::ResourceRegistry;

pub fn map_loader(registry: &mut ResourceRegistry, resource_loader: ResourceLoader) -> Map {
    let configuration_content = resource_loader.load_as_string(String::from("conf.json"));
    let configuration = load_configuration(configuration_content, registry);

    let map_content = resource_loader.load_as_string(String::from("1.map"));
    let map = Map::new(
        &map_content,
        configuration)
        .unwrap();

    map
}
