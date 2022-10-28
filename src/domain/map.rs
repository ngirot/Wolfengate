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

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn height(&self) -> u8 {
        self.height
    }
}

#[cfg(test)]
mod map_test {
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
}
