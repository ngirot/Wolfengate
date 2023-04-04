use std::collections::HashMap;

use crate::domain::control::actions::ActionStateBuilder;
use crate::domain::topology::index::TextureIndex;

pub const DOOR_OPENING_SPEED_IN_UNITS_PER_SECONDS: f32 = 3.0;

pub struct Map {
    paving: Vec<Vec<Tile>>,
    border_texture: TextureIndex,
    width: i16,
    height: i16,
}

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    SOLID(TextureIndex),
    DYNAMIC(TextureIndex, TextureIndex, ActionStateBuilder),
    NOTHING,
}

pub struct MapConfiguration {
    conf: HashMap<char, Tile>,
    map_border_texture: TextureIndex,
}

impl Map {
    pub fn new(paving: &str, configuration: MapConfiguration) -> Result<Self, String> {
        let mut pav_x: Vec<Vec<Tile>> = vec![];
        for line in paving.split('\n') {
            for (x, char) in line.chars().enumerate() {
                if pav_x.len() <= x {
                    pav_x.push(vec![]);
                }
                let tile = Self::char_to_tile(&configuration, char)?;
                pav_x[x].push(tile)
            }
        }

        for x in 0..pav_x.len() {
            pav_x[x].reverse();
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
            height,
            width,
        })
    }

    pub fn paving_at(&self, x: i16, y: i16) -> Option<&Tile> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }

        Some(&self.paving[x as usize][y as usize])
    }


    fn char_to_tile(configuration: &MapConfiguration, c: char) -> Result<Tile, String> {
        configuration.get(c)
            .ok_or(String::from("Unknown char is used in the map"))
            .copied()
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
    pub fn new(map_border_texture: TextureIndex) -> Self {
        Self {
            conf: HashMap::new(),
            map_border_texture,
        }
    }

    pub fn add(&mut self, c: char, conf: Tile) {
        self.conf.insert(c, conf);
    }

    pub fn get(&self, c: char) -> Option<&Tile> {
        self.conf.get(&c)
    }

    pub fn map_border_texture(&self) -> TextureIndex {
        self.map_border_texture
    }
}

#[cfg(test)]
pub mod map_test {
    use spectral::prelude::*;
    use crate::domain::actors::actor::SpeedStats;
    use crate::domain::control::actions::{ActionStateBuilder, LinearActionState, NothingActionState};
    use crate::domain::topology::index::TextureIndex;
    use crate::domain::topology::map::{DOOR_OPENING_SPEED_IN_UNITS_PER_SECONDS, Map, MapConfiguration, Tile};

    pub fn default_configuration() -> MapConfiguration {
        let door_state_builder = ActionStateBuilder::new(DOOR_OPENING_SPEED_IN_UNITS_PER_SECONDS, |context| Box::new(LinearActionState::new(SpeedStats::new(context))));
        let glass_state_builder = ActionStateBuilder::new(0.0, |_| Box::new(NothingActionState::new()));

        let mut configuration = MapConfiguration::new(TextureIndex::new(0));
        configuration.add('#', Tile::SOLID(TextureIndex::new(1)));
        configuration.add('D', Tile::DYNAMIC(TextureIndex::new(2), TextureIndex::new(4), door_state_builder));
        configuration.add('G', Tile::DYNAMIC(TextureIndex::new(3), TextureIndex::new(4), glass_state_builder));
        configuration.add(' ', Tile::NOTHING);

        configuration
    }

    #[test]
    fn should_read_paving_information() {
        let paving = String::from("###\n# #\n# #\n###");
        let map = Map::new(&paving, default_configuration()).unwrap();

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
        let map = Map::new("  \n  ", default_configuration()).unwrap();
        let tile = map.paving_at(0, 2);
        assert!(matches!(tile, None));
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_x_coordinate_bigger_than_height_map() {
        let map = Map::new("  \n  ", default_configuration()).unwrap();
        let tile = map.paving_at(2, 0);
        assert!(matches!(tile, None));
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_negative_x_coordinate() {
        let map = Map::new("  \n  ", default_configuration()).unwrap();
        let tile = map.paving_at(-1, 0);
        assert!(matches!(tile, None));
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_negative_y_coordinate() {
        let map = Map::new("  \n  ", default_configuration()).unwrap();
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
