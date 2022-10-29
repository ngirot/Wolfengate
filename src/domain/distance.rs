use super::{
    map::{Map, Tile},
    coord::{MapPoint, Position},
};

pub fn distance(position: Position, angle: f32, map: &Map) -> Option<f32> {
    let direction_x = angle.cos().signum();
    let direction_y = angle.sin().signum();

    let next_x_position = position.projection_x(angle);
    let next_y_position = position.projection_y(angle);

    let distance_to_next_x = position.distance(&next_x_position);
    let distance_to_next_y = position.distance(&next_y_position);

    let bloc: MapPoint;
    let next_position;
    if distance_to_next_x < distance_to_next_y {
        bloc = next_x_position.to_map_point(direction_x, 0.0);
        next_position = next_x_position
    } else {
        bloc = next_y_position.to_map_point(0.0, direction_y);
        next_position = next_y_position
    };

    if !map.is_in_map(bloc.x(), bloc.y()) {
        return None;
    }

    let bloc_tile = map.paving_at(bloc.x(), bloc.y());

    let distance_total = position.distance(&next_position);
    match bloc_tile {
        Tile::Wall => Some(distance_total),
        _ => {
            let added = distance(next_position, angle, map);
            added.map(|d| d + distance_total)
        }
    }
}


#[cfg(test)]
mod distance_test {
    use std::f32::consts::PI;

    use crate::domain::{map::Map, coord::Position};
    use spectral::prelude::*;

    use super::distance;

    #[test]
    fn should_find_distance_one_tile_ahead() {
        let map = Map::new(
            "\"
            #####\n\
            # # #\n\
            #   #\n\
            #   #\n\
            #####",
        );
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, PI / 2.0, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap()).is_close_to(0.5, 0.1)
    }

    #[test]
    fn should_find_distance_multiple_tile_ahead() {
        let map = Map::new(
            "\"
            #####\n\
            # # #\n\
            #   #\n\
            #   #\n\
            #####",
        );
        let center = Position::new(2.5, 1.3);
        let distance = distance(center, PI / 2.0, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap()).is_close_to(1.7, 0.1)
    }

    #[test]
    fn should_find_distance_one_tile_behind() {
        let map = Map::new(
            "\"
            #####\n\
            #   #\n\
            #   #\n\
            # # #\n\
            #####",
        );
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, PI * 1.5, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap()).is_close_to(0.5, 0.1)
    }

    #[test]
    fn should_find_distance_multiple_tile_behind() {
        let map = Map::new(
            "\"
            #####\n\
            #   #\n\
            #   #\n\
            # # #\n\
            #####",
        );
        let center = Position::new(2.5, 3.2);
        let distance = distance(center, PI * 1.5, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap()).is_close_to(1.2, 0.1)
    }

    #[test]
    fn should_find_distance_one_tile_left() {
        let map = Map::new(
            "\"
            #####\n\
            #   #\n\
            ##  #\n\
            #   #\n\
            #####",
        );
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, PI, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap()).is_close_to(0.5, 0.1)
    }

    #[test]
    fn should_find_distance_multiple_tile_left() {
        let map = Map::new(
            "\"
            #####\n\
            #   #\n\
            ##  #\n\
            #   #\n\
            #####",
        );
        let center = Position::new(3.1, 2.5);
        let distance = distance(center, PI, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap()).is_close_to(1.1, 0.1)
    }

    #[test]
    fn should_find_distance_one_tile_right() {
        let map = Map::new(
            "\"
            #####\n\
            #   #\n\
            #  ##\n\
            #   #\n\
            #####",
        );
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, 0.0, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap()).is_close_to(0.5, 0.1)
    }

    #[test]
    fn should_find_distance_multiple_tile_right() {
        let map = Map::new(
            "\"
            #####\n\
            #   #\n\
            #  ##\n\
            #   #\n\
            #####",
        );
        let center = Position::new(1.1, 2.5);
        let distance = distance(center, 0.0, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap()).is_close_to(1.9, 0.1)
    }

    #[test]
    fn should_find_distance_with_angle_upper_right() {
        let map = Map::new(
            "\"
            #####\n\
            #  ##\n\
            #   #\n\
            #   #\n\
            #####",
        );
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, 0.7, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap()).is_close_to(0.7, 0.1)
    }

    #[test]
    fn should_find_distance_with_angle_upper_left() {
        let map = Map::new(
            "\"
            #####\n\
            ##  #\n\
            #   #\n\
            #   #\n\
            #####",
        );
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, PI - 0.7, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap()).is_close_to(0.7, 0.1)
    }

    #[test]
    fn should_find_distance_with_angle_lower_right() {
        let map = Map::new(
            "\"
            #####\n\
            #   #\n\
            #   #\n\
            #  ##\n\
            #####",
        );
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, -0.7, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap()).is_close_to(0.7, 0.1)
    }

    #[test]
    fn should_find_distance_with_angle_lower_left() {
        let map = Map::new(
            "\"
            #####\n\
            #   #\n\
            #   #\n\
            ##  #\n\
            #####",
        );
        let center = Position::new(2.5, 2.5);
        let distance = distance(center, PI + 0.7, &map);

        assert_that(&distance).is_some();
        assert_that(&distance.unwrap()).is_close_to(0.7, 0.1)
    }

    #[test]
    fn should_return_none_when_there_is_no_border() {
        let map = Map::new(
            "\"
            #####\n\
            #    \n\
            #####",
        );
        let center = Position::new(1.5, 1.5);
        let distance = distance(center, 0.0, &map);

        assert_that(&distance).is_none();
    }
}
