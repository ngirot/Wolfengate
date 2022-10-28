pub struct Map {
    paving: Vec<Vec<Tile>>,
    width: u8,
    height: u8,
}

pub enum Tile {
    Wall,
    Nothing,
}

impl Map {
    pub fn new(paving: &str) -> Self {
        let mut p: Vec<Vec<Tile>> = vec![];
        let mut height: u8 = 0;
        let mut width: u8 = 0;

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
            width = chars.len() as u8;
            p.push(chars);
        }

        p.reverse();

        Map {
            paving: p,
            height,
            width,
        }
    }

    pub fn paving_at(&self, x: u8, y: u8) -> &Tile {
        let v: &Vec<Tile> = self.paving.get(y as usize).unwrap();
        let x: &Tile = v.get(x as usize).unwrap();
        x
    }

    pub fn is_in_map(&self, x: u8, y: u8) -> bool {
        let in_map = x >= 0 as u8 && y >= 0 as u8 && x < self.width && y < self.height;
        in_map
    }
}

#[cfg(test)]
mod map_test {
    use spectral::{assert_that, prelude::BooleanAssertions};

    use crate::domain::map::{Map, Tile};

    #[test]
    fn should_read_paving_information() {
        let paving = String::from("###\n# #\n# #\n###");
        let map = Map::new(&paving);

        assert!(matches!(&map.paving_at(0, 0), Tile::Wall));
        assert!(matches!(&map.paving_at(1, 0), Tile::Wall));
        assert!(matches!(&map.paving_at(2, 0), Tile::Wall));

        assert!(matches!(&map.paving_at(0, 1), Tile::Wall));
        assert!(matches!(&map.paving_at(1, 1), Tile::Nothing));
        assert!(matches!(&map.paving_at(2, 1), Tile::Wall));

        assert!(matches!(&map.paving_at(0, 2), Tile::Wall));
        assert!(matches!(&map.paving_at(1, 2), Tile::Nothing));
        assert!(matches!(&map.paving_at(2, 2), Tile::Wall));

        assert!(matches!(&map.paving_at(0, 3), Tile::Wall));
        assert!(matches!(&map.paving_at(1, 3), Tile::Wall));
        assert!(matches!(&map.paving_at(2, 3), Tile::Wall));
    }

    #[test]
    fn all_tiles_should_be_in_map() {
        let map = Map::new("  \n  ");
        assert_that!(map.is_in_map(0, 0)).is_true();
        assert_that!(map.is_in_map(0, 1)).is_true();
        assert_that!(map.is_in_map(1, 0)).is_true();
        assert_that!(map.is_in_map(0, 1)).is_true();
    }

    #[test]
    fn tiles_bigger_than_width_should_not_be_in_map() {
        let map = Map::new("  \n  ");
        assert_that!(map.is_in_map(0, 2)).is_false();
    }

    #[test]
    fn tiles_bigger_than_height_should_not_be_in_map() {
        let map = Map::new("  \n  ");
        assert_that!(map.is_in_map(2, 0)).is_false();
    }
}
