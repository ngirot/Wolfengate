use super::{
    coord::{MapPoint, Position},
    map::{Map, Tile},
};

#[derive(Debug)]
pub struct ProjectedPoint {
    distance: f32,
    offset_in_bloc: f32,
}

pub fn distance(position: Position, angle: f32, map: &Map) -> Option<ProjectedPoint> {
    let direction_x = angle.cos().signum();
    let direction_y = angle.sin().signum();

    let next_x_position = position.projection_x(angle);
    let next_y_position = position.projection_y(angle);

    let distance_to_next_x = position.distance(&next_x_position);
    let distance_to_next_y = position.distance(&next_y_position);

    let bloc: MapPoint;
    let next_position;
    let position_on_texture;
    if distance_to_next_x < distance_to_next_y {
        bloc = next_x_position.to_map_point(direction_x, 0.0);
        position_on_texture = decimal_part(next_x_position.y());
        next_position = next_x_position;
    } else {
        bloc = next_y_position.to_map_point(0.0, direction_y);
        position_on_texture = decimal_part(next_y_position.x());
        next_position = next_y_position
    };

    let bloc_tile = map.paving_at(bloc.x(), bloc.y());

    let distance_total = position.distance(&next_position);
    match bloc_tile {
        None => None,
        Some(Tile::Wall) => Some(ProjectedPoint::new(distance_total, position_on_texture)),
        _ => {
            let added = distance(next_position, angle, map);
            added.map(|d| d.with_distance_added(distance_total))
        }
    }
}

impl ProjectedPoint {
    pub fn new(distance: f32, offset_in_bloc: f32) -> Self {
        ProjectedPoint {
            distance,
            offset_in_bloc,
        }
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn offset_in_bloc(&self) -> f32 {
        self.offset_in_bloc
    }

    fn with_distance_added(&self, distance_to_add: f32) -> Self {
        ProjectedPoint {
            distance: self.distance + distance_to_add,
            offset_in_bloc: self.offset_in_bloc,
        }
    }
}

fn decimal_part(number: f32) -> f32 {
    number.ceil() - number
}

#[cfg(test)]
mod distance_test {
    use std::f32::consts::PI;

    use crate::domain::{coord::Position, map::Map};
    use spectral::prelude::*;

    use super::distance;

    #[test]
    fn should_find_distance_one_tile_ahead() {
        let map = Map::new(
            "\
            #####\n\
            # # #\n\
            #   #\n\
            #   #\n\
            #####",
        )
        .unwrap();
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, PI / 2.0, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap().distance()).is_close_to(0.5, 0.1)
    }

    #[test]
    fn should_find_distance_multiple_tile_ahead() {
        let map = Map::new(
            "\
            #####\n\
            # # #\n\
            #   #\n\
            #   #\n\
            #####",
        )
        .unwrap();
        let center = Position::new(2.5, 1.3);
        let distance = distance(center, PI / 2.0, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap().distance()).is_close_to(1.7, 0.1)
    }

    #[test]
    fn should_find_distance_one_tile_behind() {
        let map = Map::new(
            "\
            #####\n\
            #   #\n\
            #   #\n\
            # # #\n\
            #####",
        )
        .unwrap();
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, PI * 1.5, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap().distance()).is_close_to(0.5, 0.1)
    }

    #[test]
    fn should_find_distance_multiple_tile_behind() {
        let map = Map::new(
            "\
            #####\n\
            #   #\n\
            #   #\n\
            # # #\n\
            #####",
        )
        .unwrap();
        let center = Position::new(2.5, 3.2);
        let distance = distance(center, PI * 1.5, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap().distance()).is_close_to(1.2, 0.1)
    }

    #[test]
    fn should_find_distance_one_tile_left() {
        let map = Map::new(
            "\
            #####\n\
            #   #\n\
            ##  #\n\
            #   #\n\
            #####",
        )
        .unwrap();
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, PI, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap().distance()).is_close_to(0.5, 0.1)
    }

    #[test]
    fn should_find_distance_multiple_tile_left() {
        let map = Map::new(
            "\
            #####\n\
            #   #\n\
            ##  #\n\
            #   #\n\
            #####",
        )
        .unwrap();
        let center = Position::new(3.1, 2.5);
        let distance = distance(center, PI, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap().distance()).is_close_to(1.1, 0.1)
    }

    #[test]
    fn should_find_distance_one_tile_right() {
        let map = Map::new(
            "\
            #####\n\
            #   #\n\
            #  ##\n\
            #   #\n\
            #####",
        )
        .unwrap();
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, 0.0, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap().distance()).is_close_to(0.5, 0.1)
    }

    #[test]
    fn should_find_distance_multiple_tile_right() {
        let map = Map::new(
            "\
            #####\n\
            #   #\n\
            #  ##\n\
            #   #\n\
            #####",
        )
        .unwrap();
        let center = Position::new(1.1, 2.5);
        let distance = distance(center, 0.0, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap().distance()).is_close_to(1.9, 0.1)
    }

    #[test]
    fn should_find_distance_with_angle_upper_right() {
        let map = Map::new(
            "\
            #####\n\
            #  ##\n\
            #   #\n\
            #   #\n\
            #####",
        )
        .unwrap();
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, 0.7, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap().distance()).is_close_to(0.7, 0.1)
    }

    #[test]
    fn should_find_distance_with_angle_upper_left() {
        let map = Map::new(
            "\
            #####\n\
            ##  #\n\
            #   #\n\
            #   #\n\
            #####",
        )
        .unwrap();
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, PI - 0.7, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap().distance()).is_close_to(0.7, 0.1)
    }

    #[test]
    fn should_find_distance_with_angle_lower_right() {
        let map = Map::new(
            "\
            #####\n\
            #   #\n\
            #   #\n\
            #  ##\n\
            #####",
        )
        .unwrap();
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, -0.7, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap().distance()).is_close_to(0.7, 0.1)
    }

    #[test]
    fn should_find_distance_with_angle_lower_left() {
        let map = Map::new(
            "\
            #####\n\
            #   #\n\
            #   #\n\
            ##  #\n\
            #####",
        )
        .unwrap();
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, PI + 0.7, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap().distance()).is_close_to(0.7, 0.1)
    }

    #[test]
    fn should_return_none_when_there_is_no_border() {
        let map = Map::new(
            "\
            #####\n\
            #    \n\
            #####",
        )
        .unwrap();
        let center = Position::new(1.5, 1.5);
        let distance = distance(center, 0.0, &map);

        assert_that(&distance).is_none();
    }

    #[test]
    fn position_on_texture_on_straight_direction() {
        let map = Map::new(
            "\
            #####\n\
            # # #\n\
            #   #\n\
            #   #\n\
            #####",
        )
        .unwrap();
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, PI / 2.0, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap().offset_in_bloc()).is_close_to(0.5, 0.001)
    }

    #[test]
    fn position_on_texture_on_diagonal_direction() {
        let map = Map::new(
            "\
            #####\n\
            # # #\n\
            #   #\n\
            #   #\n\
            #####",
        )
        .unwrap();
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, PI / 2.0 + 0.23, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap().offset_in_bloc()).is_close_to(0.617, 0.001)
    }
}
