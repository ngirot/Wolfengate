use std::collections::HashMap;

use crate::domain::actors::actor::{Enemy, Player, PlayerStats};
use crate::domain::actors::shoot::{Weapon, WeaponConfiguration};
use crate::domain::control::actions::ActionStateBuilder;
use crate::domain::maths::Angle;
use crate::domain::topology::coord::Position;
use crate::domain::topology::index::TextureIndex;

pub struct Map {
    paving: Vec<Vec<Tile>>,
    border_texture: TextureIndex,
    enemies: Vec<Enemy>,
    player: Option<Player>,
    weapon: WeaponConfiguration,
    width: i16,
    height: i16,
}

#[derive(Clone)]
pub enum Tile {
    SOLID(TextureIndex),
    DYNAMIC(TextureIndex, TextureIndex, ActionStateBuilder),
    NOTHING,
}

#[derive(Copy, Clone)]
pub struct EnemyType {
    texture: TextureIndex,
}

#[derive(Copy, Clone)]
pub struct SpawnPoint {
    orientation: Angle,
}

#[derive(Clone)]
pub struct MapConfiguration {
    conf: HashMap<char, Tile>,
    enemies: HashMap<char, EnemyType>,
    spawn: HashMap<char, SpawnPoint>,
    map_border_texture: TextureIndex,
    player_conf: PlayerStats,
    weapon: WeaponConfiguration,
}

impl Map {
    pub fn new(paving: &str, configuration: MapConfiguration) -> Result<Self, String> {
        let mut enemies = vec![];
        let mut player = None;

        let mut pav_x: Vec<Vec<Tile>> = vec![];
        let split: Vec<&str> = paving.split('\n').collect();
        let mut y: i32 = split.len() as i32 - 1;

        for line in split {
            for (x, char) in line.chars().enumerate() {
                if pav_x.len() <= x {
                    pav_x.push(vec![]);
                }
                if let Some(spawn) = Self::char_to_spawn(&configuration, char) {
                    let position = Position::new(x as f32 + 0.5, y as f32 + 0.5);
                    let orientation = spawn.orientation();
                    player = Some(Player::new(position, orientation, configuration.player_conf()));
                    pav_x[x].push(Tile::NOTHING);
                } else if let Some(enemy) = Self::char_to_enemy(&configuration, char) {
                    let position = Position::new(x as f32 + 0.5, y as f32 + 0.5);
                    enemies.push(Enemy::new(enemy.texture(), position));
                    pav_x[x].push(Tile::NOTHING)
                } else {
                    let tile = Self::char_to_tile(&configuration, char)?;
                    pav_x[x].push(tile)
                }
            }
            y -= 1;
        }

        for x in &mut pav_x {
            x.reverse();
        }

        let mut current_height = 0;
        for line in &pav_x {
            let height = line.len() as i16;
            if current_height == 0 {
                current_height = height;
            }
            if current_height != height {
                return Err(String::from(
                    "Level is not valid: number of column is not consistent in every lines",
                ));
            }
        }
        let height: i16 = current_height;
        let width = pav_x.len() as i16;

        Ok(Self {
            paving: pav_x,
            border_texture: configuration.map_border_texture(),
            enemies,
            player,
            height,
            width,
           weapon: configuration.weapon,
        })
    }

    pub fn paving_at(&self, x: i16, y: i16) -> Option<&Tile> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }

        Some(&self.paving[x as usize][y as usize])
    }

    fn char_to_enemy(configuration: &MapConfiguration, c: char) -> Option<&EnemyType> {
        configuration.get_enemy(c)
    }

    fn char_to_spawn(configuration: &MapConfiguration, c: char) -> Option<&SpawnPoint> {
        configuration.get_spawn(c)
    }

    fn char_to_tile(configuration: &MapConfiguration, c: char) -> Result<Tile, String> {
        configuration.get(c)
            .ok_or_else(|| String::from("Unknown char is used in the map"))
            .map(|tile| tile.clone())
    }

    pub fn generate_enemies(&self) -> Vec<Enemy> {
        self.enemies.to_vec()
    }

    pub fn generate_player(&self) -> Option<Player> {
        self.player.clone()
    }

    pub fn generate_weapon(&self) -> Weapon {
        Weapon::new(self.weapon)
    }

    pub fn width(&self) -> i16 {
        self.width
    }

    pub fn height(&self) -> i16 {
        self.height
    }


    pub fn border_texture(&self) -> TextureIndex {
        self.border_texture
    }
}

impl MapConfiguration {
    pub fn new(map_border_texture: TextureIndex, player_conf: PlayerStats, weapon: WeaponConfiguration) -> Self {
        Self {
            conf: HashMap::new(),
            map_border_texture,
            enemies: HashMap::new(),
            spawn: HashMap::new(),
            player_conf,
            weapon,
        }
    }

    pub fn add(&mut self, c: char, conf: Tile) {
        self.conf.insert(c, conf);
    }

    pub fn add_enemy(&mut self, c: char, enemy: EnemyType) {
        self.enemies.insert(c, enemy);
    }

    pub fn add_spawn(&mut self, c: char, spawn_point: SpawnPoint) {
        self.spawn.insert(c, spawn_point);
    }

    pub fn get(&self, c: char) -> Option<&Tile> {
        self.conf.get(&c)
    }

    pub fn get_enemy(&self, c: char) -> Option<&EnemyType> {
        self.enemies.get(&c)
    }

    pub fn get_spawn(&self, c: char) -> Option<&SpawnPoint> {
        self.spawn.get(&c)
    }

    pub fn map_border_texture(&self) -> TextureIndex {
        self.map_border_texture
    }

    pub fn player_conf(&self) -> PlayerStats {
        self.player_conf
    }
}

impl EnemyType {
    pub fn new(texture: TextureIndex) -> Self {
        Self {
            texture
        }
    }

    pub fn texture(&self) -> TextureIndex {
        self.texture
    }
}

impl SpawnPoint {
    pub fn new(orientation: Angle) -> Self {
        Self { orientation }
    }

    pub fn orientation(&self) -> Angle {
        self.orientation
    }
}

#[cfg(test)]
pub mod map_test {
    use spectral::prelude::*;

    use crate::domain::actors::actor::{AccelerationStats, PlayerStats, SpeedStats};
    use crate::domain::actors::shoot::{AnimationStep, WeaponConfiguration};
    use crate::domain::control::actions::{ActionStateBuilder, LinearActionState, NothingActionState};
    use crate::domain::maths::{ANGLE_DOWN, ANGLE_LEFT, ANGLE_RIGHT, ANGLE_UP};
    use crate::domain::topology::door::LateralOpening;
    use crate::domain::topology::index::TextureIndex;
    use crate::domain::topology::map::{EnemyType, Map, MapConfiguration, SpawnPoint, Tile};

    pub const DOOR_OPENING_SPEED_IN_UNITS_PER_SECONDS: f32 = 3.0;

    pub fn default_configuration() -> MapConfiguration {
        let door_state_builder = ActionStateBuilder::new(Box::new(LinearActionState::new(SpeedStats::new(DOOR_OPENING_SPEED_IN_UNITS_PER_SECONDS), Box::new(LateralOpening::new()))));
        let glass_state_builder = ActionStateBuilder::new(Box::new(NothingActionState::new()));
        let weapon_animation = AnimationStep::new(0.1, TextureIndex::new(0));
        let weapon_configuration =WeaponConfiguration::new(TextureIndex::new(0), weapon_animation, weapon_animation, weapon_animation);

        let mut configuration = MapConfiguration::new(TextureIndex::new(0), default_stats(), weapon_configuration);
        configuration.add('#', Tile::SOLID(TextureIndex::new(1)));
        configuration.add('D', Tile::DYNAMIC(TextureIndex::new(2), TextureIndex::new(4), door_state_builder));
        configuration.add('G', Tile::DYNAMIC(TextureIndex::new(3), TextureIndex::new(4), glass_state_builder));
        configuration.add(' ', Tile::NOTHING);

        configuration.add_enemy('E', EnemyType::new(TextureIndex::new(4)));

        configuration.add_spawn('u', SpawnPoint::new(ANGLE_UP));
        configuration.add_spawn('d', SpawnPoint::new(ANGLE_DOWN));
        configuration.add_spawn('l', SpawnPoint::new(ANGLE_LEFT));
        configuration.add_spawn('r', SpawnPoint::new(ANGLE_RIGHT));

        configuration.clone()
    }

    pub fn build_map(paving: &str) -> Map {
        Map::new(paving, default_configuration()).unwrap()
    }

    pub fn default_stats() -> PlayerStats {
        let acceleration = AccelerationStats::new(1000000000.0);
        let deceleration = AccelerationStats::new(1.0);
        let max_speed = SpeedStats::new(100000.0);
        PlayerStats::new(acceleration, deceleration, max_speed)
    }

    #[test]
    fn should_read_paving_information() {
        let paving = String::from("###\n# #\n# #\n###");
        let map = build_map(&paving);

        assert!(matches!(&map.paving_at(0, 0), Some(Tile::SOLID(_))));
        assert!(matches!(&map.paving_at(1, 0), Some(Tile::SOLID(_))));
        assert!(matches!(&map.paving_at(2, 0), Some(Tile::SOLID(_))));

        assert!(matches!(&map.paving_at(0, 1), Some(Tile::SOLID(_))));
        assert!(matches!(&map.paving_at(1, 1), Some(Tile::NOTHING)));
        assert!(matches!(&map.paving_at(2, 1), Some(Tile::SOLID(_))));

        assert!(matches!(&map.paving_at(0, 2), Some(Tile::SOLID(_))));
        assert!(matches!(&map.paving_at(1, 2), Some(Tile::NOTHING)));
        assert!(matches!(&map.paving_at(2, 2), Some(Tile::SOLID(_))));

        assert!(matches!(&map.paving_at(0, 3), Some(Tile::SOLID(_))));
        assert!(matches!(&map.paving_at(1, 3), Some(Tile::SOLID(_))));
        assert!(matches!(&map.paving_at(2, 3), Some(Tile::SOLID(_))));
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_x_coordinate_bigger_than_width_map() {
        let map = build_map("  \n  ");
        let tile = map.paving_at(0, 2);
        assert!(matches!(tile, None));
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_x_coordinate_bigger_than_height_map() {
        let map = build_map("  \n  ");
        let tile = map.paving_at(2, 0);
        assert!(matches!(tile, None));
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_negative_x_coordinate() {
        let map = build_map("  \n  ");
        let tile = map.paving_at(-1, 0);
        assert!(matches!(tile, None));
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_negative_y_coordinate() {
        let map = build_map("  \n  ");
        let tile = map.paving_at(0, -1);
        assert!(matches!(tile, None));
    }

    #[test]
    fn should_not_validate_a_map_with_inconsistent_column_number() {
        let map = Map::new("   \n  ", default_configuration());
        assert_that!(map.err()).is_some();
    }

    #[test]
    fn should_not_validate_a_map_with_unknown_char() {
        let map = Map::new("#k\n #", default_configuration());
        assert_that!(map.err()).is_some();
    }
}
