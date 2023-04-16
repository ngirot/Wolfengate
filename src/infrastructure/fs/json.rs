use serde::{Deserialize, Serialize};

use crate::domain::actors::actor::SpeedStats;
use crate::domain::control::actions::{ActionStateBuilder, LinearActionState, NothingActionState};
use crate::domain::maths::Angle;
use crate::domain::topology::door::{CentralOpening, LateralOpening, Openable};
use crate::domain::topology::map::{EnemyType, MapConfiguration, SpawnPoint};
use crate::infrastructure::sdl::texture::ResourceRegistryLoader;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub state_type: String,
    pub opening_mode: String,
    pub speed: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Tile {
    pub id: String,
    pub tile_type: String,
    pub texture: Option<String>,
    pub state: Option<State>,
    pub orientation_in_degrees: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct Json {
    tiles: Vec<Tile>,
}

pub fn load_configuration(content: String, resource_registry: &mut dyn ResourceRegistryLoader) -> MapConfiguration {
    let data = load(content);
    to_conf(data, resource_registry)
}

fn load(content: String) -> Json {
    serde_json::from_str(&content).unwrap()
}

fn to_conf(data: Json, resource_registry: &mut dyn ResourceRegistryLoader) -> MapConfiguration {
    let transparency = resource_registry.load_texture(String::from("transparency.png"));
    let mut conf = MapConfiguration::new(transparency);

    for tile in data.tiles {
        let texture = tile.texture
            .map_or_else(
                || transparency,
                |id| resource_registry.load_texture(id));
        let id_char = tile.id.as_bytes()[0] as char;

        if tile.tile_type == "NOTHING" {
            conf.add(id_char, crate::domain::topology::map::Tile::NOTHING)
        }
        if tile.tile_type == "SOLID" {
            conf.add(id_char, crate::domain::topology::map::Tile::SOLID(texture))
        }
        if tile.tile_type == "ENEMY" {
            conf.add_enemy(id_char, EnemyType::new(texture));
        }
        if tile.tile_type == "PLAYER" {
            let angle = Angle::from_degree(tile.orientation_in_degrees.unwrap());
            conf.add_spawn(id_char, SpawnPoint::new(angle));
        }
        if tile.tile_type == "DYNAMIC" {
            let state = tile.state.map_or_else(
                || {
                    ActionStateBuilder::new(Box::new(NothingActionState::new()))
                },
                |state| {
                    let openable: Box<dyn Openable> = if state.opening_mode == "CENTER" {
                        Box::new(CentralOpening::new())
                    } else {
                        Box::new(LateralOpening::new())
                    };
                    ActionStateBuilder::new(Box::new(LinearActionState::new(SpeedStats::new(state.speed), openable)))
                });

            conf.add(id_char, crate::domain::topology::map::Tile::DYNAMIC(texture, transparency, state))
        }
    }

    conf
}