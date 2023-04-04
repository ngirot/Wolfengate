use serde::{Deserialize, Serialize};

use crate::domain::actors::actor::SpeedStats;
use crate::domain::control::actions::{ActionStateBuilder, LinearActionState, NothingActionState};
use crate::domain::topology::map::MapConfiguration;
use crate::sdl::texture::ResourceRegistry;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub state_type: String,
    pub speed: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Tile {
    pub id: String,
    pub tile_type: String,
    pub texture: Option<String>,
    pub state: Option<State>,
}

#[derive(Serialize, Deserialize)]
pub struct Json {
    tiles: Vec<Tile>,
}

pub fn load_configuration(content: String, resource_registry: &mut ResourceRegistry) -> MapConfiguration {
    let data = load(content);
    to_conf(data, resource_registry)
}

fn load(content: String) -> Json {
    serde_json::from_str(&content).unwrap()
}

fn to_conf(data: Json, resource_registry: &mut ResourceRegistry) -> MapConfiguration {
    let transparency = resource_registry.load_texture(String::from("transparency.png"));
    let mut conf = MapConfiguration::new(transparency);

    for tile in data.tiles {
        let texture = tile.texture
            .map_or_else(
                || transparency,
                |id| resource_registry.load_texture(id));

        if tile.tile_type == "NOTHING" {
            conf.add(tile.id.as_bytes()[0] as char, super::super::domain::topology::map::Tile::NOTHING)
        }
        if tile.tile_type == "SOLID" {
            conf.add(tile.id.as_bytes()[0] as char, super::super::domain::topology::map::Tile::SOLID(texture))
        }
        if tile.tile_type == "DYNAMIC" {
            let state = tile.state.map_or_else(
                || {
                    ActionStateBuilder::new(0.0, |_| Box::new(NothingActionState::new()))
                },
                |state| {
                    ActionStateBuilder::new(state.speed, |context| Box::new(LinearActionState::new(SpeedStats::new(context))))
                });
            conf.add(tile.id.as_bytes()[0] as char, super::super::domain::topology::map::Tile::DYNAMIC(texture, transparency, state))
        }
    }

    conf
}