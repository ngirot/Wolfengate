use std::collections::HashMap;

use crate::domain::actions::{ActionState, LinearActionState, NothingActionState};
use crate::domain::actor::SpeedStats;
use crate::domain::index::TextureIndex;

pub const DOOR_OPENING_SPEED_IN_UNITS_PER_SECONDS: f32 = 3.0;

pub struct Map {
    paving: Vec<Vec<Tile>>,
    width: i16,
    height: i16,
}

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    SOLID(TextureIndex),
    DYNAMIC(TextureIndex, TextureIndex, fn() -> Box<dyn ActionState>),
    NOTHING,
}

pub struct MapConfiguration {
    conf: HashMap<char, Tile>,
}

impl Map {
    pub fn new(paving: &str) -> Result<Self, String> {
        let mut p: Vec<Vec<Tile>> = vec![];
        let mut height: i16 = 0;
        let mut width: i16 = 0;


        let mut configuration = MapConfiguration::new();
        configuration.add('#', Tile::SOLID(TextureIndex::WALL));
        configuration.add('D', Tile::DYNAMIC(TextureIndex::DOOR, TextureIndex::VOID, || Box::new(LinearActionState::new(SpeedStats::new(DOOR_OPENING_SPEED_IN_UNITS_PER_SECONDS)))));
        configuration.add('G', Tile::DYNAMIC(TextureIndex::GLASS, TextureIndex::VOID, || Box::new(NothingActionState::new())));
        configuration.add(' ', Tile::NOTHING);

        for line in paving.split('\n') {
            height += 1;
            let mut chars: Vec<Tile> = vec![];
            for char in line.chars() {
                let tile = Self::char_to_tile(&configuration, char)?;
                chars.push(tile)
            }

            let current_width = chars.len() as i16;
            if width == 0 {
                width = current_width;
            }

            if current_width != width {
                return Err(String::from(
                    "Level is not valid: number of column is not consistent in every lines",
                ));
            }

            p.push(chars);
        }

        p.reverse();

        Ok(Self {
            paving: p,
            height,
            width,
        })
    }

    pub fn paving_at(&self, x: i16, y: i16) -> Option<&Tile> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }

        Some(&self.paving[y as usize][x as usize])
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
}

impl MapConfiguration {
    pub fn new() -> Self {
        Self {
            conf: HashMap::new()
        }
    }

    pub fn add(&mut self, c: char, conf: Tile) {
        self.conf.insert(c, conf);
    }

    pub fn get(&self, c: char) -> Option<&Tile> {
        self.conf.get(&c)
    }
}

#[cfg(test)]
mod map_test {
    use spectral::prelude::*;

    use crate::domain::map::{Map, Tile};

    #[test]
    fn should_read_paving_information() {
        let paving = String::from("###\n# #\n# #\n###");
        let map = Map::new(&paving).unwrap();

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
        let map = Map::new("  \n  ").unwrap();
        let tile = map.paving_at(0, 2);
        assert!(matches!(tile, None));
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_x_coordinate_bigger_than_height_map() {
        let map = Map::new("  \n  ").unwrap();
        let tile = map.paving_at(2, 0);
        assert!(matches!(tile, None));
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_negative_x_coordinate() {
        let map = Map::new("  \n  ").unwrap();
        let tile = map.paving_at(-1, 0);
        assert!(matches!(tile, None));
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_negative_y_coordinate() {
        let map = Map::new("  \n  ").unwrap();
        let tile = map.paving_at(0, -1);
        assert!(matches!(tile, None));
    }

    #[test]
    fn should_not_validate_a_map_with_inconsistent_column_number() {
        let map = Map::new("   \n  ");
        assert_that!(map.err()).is_some();
    }

    #[test]
    fn should_not_validate_a_map_with_unknown_char() {
        let map = Map::new("#k\n #");
        assert_that!(map.err()).is_some();
    }
}
