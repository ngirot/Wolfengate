pub struct Map {
    paving: Vec<Vec<Tile>>,
    width: i16,
    height: i16,
}

pub enum Tile {
    Wall,
    Nothing,
}

impl Map {
    pub fn new(paving: &str) -> Self {
        let mut p: Vec<Vec<Tile>> = vec![];
        let mut height: i16 = 0;
        let mut width: i16 = 0;

        for line in paving.split("\n") {
            height = height + 1;
            let mut chars: Vec<Tile> = vec![];
            for char in line.chars() {
                if char == '#' {
                    chars.push(Tile::Wall);
                } else {
                    chars.push(Tile::Nothing)
                }
            }
            width = chars.len() as i16;
            p.push(chars);
        }

        p.reverse();

        Map {
            paving: p,
            height,
            width,
        }
    }

    pub fn paving_at(&self, x: i16, y: i16) -> Option<&Tile> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }

        let v: &Vec<Tile> = self.paving.get(y as usize).unwrap();
        let x: &Tile = v.get(x as usize).unwrap();
        Some(x)
    }
}

#[cfg(test)]
mod map_test {
    use crate::domain::map::{Map, Tile};

    #[test]
    fn should_read_paving_information() {
        let paving = String::from("###\n# #\n# #\n###");
        let map = Map::new(&paving);

        assert!(matches!(&map.paving_at(0, 0), Some(Tile::Wall)));
        assert!(matches!(&map.paving_at(1, 0), Some(Tile::Wall)));
        assert!(matches!(&map.paving_at(2, 0), Some(Tile::Wall)));

        assert!(matches!(&map.paving_at(0, 1), Some(Tile::Wall)));
        assert!(matches!(&map.paving_at(1, 1), Some(Tile::Nothing)));
        assert!(matches!(&map.paving_at(2, 1), Some(Tile::Wall)));

        assert!(matches!(&map.paving_at(0, 2), Some(Tile::Wall)));
        assert!(matches!(&map.paving_at(1, 2), Some(Tile::Nothing)));
        assert!(matches!(&map.paving_at(2, 2), Some(Tile::Wall)));

        assert!(matches!(&map.paving_at(0, 3), Some(Tile::Wall)));
        assert!(matches!(&map.paving_at(1, 3), Some(Tile::Wall)));
        assert!(matches!(&map.paving_at(2, 3), Some(Tile::Wall)));
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_x_coordinate_bigger_than_width_map() {
        let map = Map::new("  \n  ");
        let tile = map.paving_at(0, 2);
        assert!(matches!(tile, None));
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_x_coordinate_bigger_than_height_map() {
        let map = Map::new("  \n  ");
        let tile = map.paving_at(2, 0);
        assert!(matches!(tile, None));
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_negative_x_coordinate() {
        let map = Map::new("  \n  ");
        let tile = map.paving_at(-1, 0);
        assert!(matches!(tile, None));
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_negative_u_coordinate() {
        let map = Map::new("  \n  ");
        let tile = map.paving_at(0, -1);
        assert!(matches!(tile, None));
    }
}
