use serde::{Deserialize, Serialize};

use crate::domain::actors::actor::{AccelerationStats, PlayerStats, SpeedStats};
use crate::domain::actors::shoot::{AnimationStep, WeaponConfiguration};
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
    pub texture_dead: Option<String>,
    pub state: Option<State>,
    pub orientation_in_degrees: Option<f32>,
    pub health: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct JsonPlayer {
    acceleration: f32,
    deceleration: f32,
    maximum_speed: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Animation {
    pub texture: String,
    pub duration: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Weapon {
    idle: String,
    damage: u32,
    startup: Animation,
    active: Animation,
    recovery: Animation,
}

#[derive(Serialize, Deserialize)]
pub struct Json {
    player: JsonPlayer,
    tiles: Vec<Tile>,
    weapon: Weapon,
}

pub fn load_configuration(content: String, resource_registry: &mut dyn ResourceRegistryLoader) -> MapConfiguration {
    let data = load(content);
    to_conf(data, resource_registry)
}

fn load(content: String) -> Json {
    serde_json::from_str(&content).unwrap()
}

fn to_conf(data: Json, resource_registry: &mut dyn ResourceRegistryLoader) -> MapConfiguration {
    let player_conf = player_conf(data.player);
    let transparency = resource_registry.load_texture(String::from("transparency.png"));

    let shoot_configuration = WeaponConfiguration::new(
        resource_registry.load_texture(data.weapon.idle),
        AnimationStep::new(data.weapon.startup.duration, resource_registry.load_texture(data.weapon.startup.texture)),
        AnimationStep::new(data.weapon.active.duration, resource_registry.load_texture(data.weapon.active.texture)),
        AnimationStep::new(data.weapon.recovery.duration, resource_registry.load_texture(data.weapon.recovery.texture)),
        data.weapon.damage);
    let mut conf = MapConfiguration::new(transparency, player_conf, shoot_configuration);

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
            let texture_dead = tile.texture_dead
                .map_or_else(
                    || transparency,
                    |id| resource_registry.load_texture(id));

            let health = tile.health.unwrap();
            conf.add_enemy(id_char, EnemyType::new(texture, texture_dead, health));
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
                        Box::new(CentralOpening::default())
                    } else {
                        Box::new(LateralOpening::default())
                    };
                    ActionStateBuilder::new(Box::new(LinearActionState::new(SpeedStats::new(state.speed), openable)))
                });

            conf.add(id_char, crate::domain::topology::map::Tile::DYNAMIC(texture, transparency, state))
        }
    }

    conf
}

fn player_conf(data: JsonPlayer) -> PlayerStats {
    PlayerStats::new(
        AccelerationStats::new(data.acceleration),
        AccelerationStats::new(data.deceleration),
        SpeedStats::new(data.maximum_speed),
    )
}