pub struct Map {
    paving: Vec<Vec<Tile>>
}

pub enum Tile {
    Wall,
    Nothing,
}

impl Map {
    pub fn new(paving: &str) -> Self {
        let mut p: Vec<Vec<Tile>> = vec![];

        for line in paving.split("\n") {
            let mut chars: Vec<Tile> = vec![];
            for char in line.chars() {
                if char == '#' {
                    chars.push(Tile::Wall);
                } else {
                    chars.push(Tile::Nothing)
                }
            }
            p.push(chars);
        }
        Map {paving: p}
    }

    pub fn paving_at(&self, x: u8, y: u8) -> &Tile {
        let v: &Vec<Tile> = self.paving.get(x as usize).unwrap();
        let x: &Tile = v.get(y as usize).unwrap();
        x
    }
}

mod map_test {
    use crate::domain::map::{Map, Tile};

    #[test]
    fn should_read_paving_information() {
        let paving = String::from("###\n# #\n###");
        let map = Map::new(&paving);

        assert!(matches!(&map.paving_at(0, 0), Tile::Wall));
        assert!(matches!(&map.paving_at(0, 1), Tile::Wall));
        assert!(matches!(&map.paving_at(0, 2), Tile::Wall));

        assert!(matches!(&map.paving_at(1, 0), Tile::Wall));
        assert!(matches!(&map.paving_at(1, 1), Tile::Nothing));
        assert!(matches!(&map.paving_at(1, 2), Tile::Wall));

        assert!(matches!(&map.paving_at(2, 0), Tile::Wall));
        assert!(matches!(&map.paving_at(2, 1), Tile::Wall));
        assert!(matches!(&map.paving_at(2, 2), Tile::Wall));
    }
}
