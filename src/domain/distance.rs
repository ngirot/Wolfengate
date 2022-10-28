use super::{
    map::{Map, Tile},
    point::{MapPoint, Position},
};

pub fn distance(position: Position, map: &Map) -> Option<f32> {
    let direction_x = position.angle().cos().signum();
    let direction_y = position.angle().sin().signum();

    let next_x = round(position.x(), direction_x);
    let next_y = round(position.y(), direction_y);

    let next_x_position = position
        .with_x(next_x)
        .with_y(position.y() + position.angle().tan() * (next_x - position.x()));
    let next_y_position = position
        .with_x(position.x() + (next_y - position.y()) / position.angle().tan())
        .with_y(next_y);

    let distance_to_next_x = position.distance(&next_x_position);
    let distance_to_next_y = position.distance(&next_y_position);

    let bloc: MapPoint;
    let next_position = if distance_to_next_x < distance_to_next_y {
        if direction_x > 0.0 {
            bloc = MapPoint::new(next_x_position.x() as u8, next_x_position.y().floor() as u8);
        } else {
            bloc = MapPoint::new(
                next_x_position.x() as u8 - 1,
                next_x_position.y().floor() as u8,
            );
        }
        next_x_position
    } else {
        if direction_y > 0.0 {
            bloc = MapPoint::new(next_y_position.x() as u8, next_y_position.y() as u8);
        } else {
            bloc = MapPoint::new(next_y_position.x() as u8, next_y_position.y() as u8 - 1);
        }
        next_y_position
    };

    if !map.is_in_map(bloc.x(), bloc.y()) {
        return None;
    }

    let bloc_tile = map.paving_at(bloc.x(), bloc.y());

    let distance_total = position.distance(&next_position);
    match bloc_tile {
        Tile::Wall => Some(distance_total),
        _ => {
            let added = distance(next_position, map);
            added.map(|d| d + distance_total)
        }
    }
}

fn round(number: f32, sign: f32) -> f32 {
    let rounded = if sign > 0.0 {
        number.ceil()
    } else {
        number.floor()
    };

    if rounded == number {
        number + sign
    } else {
        rounded
    }
}

#[cfg(test)]
mod distance_test {
    use std::f32::consts::PI;

    use crate::domain::{map::Map, point::Position};
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
        let center = Position::new(2.5, 2.5, PI / 2.0);
        let distance = distance(center, &map);

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
        let center = Position::new(2.5, 1.3, PI / 2.0);
        let distance = distance(center, &map);

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
        let center = Position::new(2.5, 2.5, PI * 1.5);
        let distance = distance(center, &map);

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
        let center = Position::new(2.5, 3.2, PI * 1.5);
        let distance = distance(center, &map);

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
        let center = Position::new(2.5, 2.5, PI);
        let distance = distance(center, &map);

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
        let center = Position::new(3.1, 2.5, PI);
        let distance = distance(center, &map);

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
        let center = Position::new(2.5, 2.5, 0.0);
        let distance = distance(center, &map);

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
        let center = Position::new(1.1, 2.5, 0.0);
        let distance = distance(center, &map);

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
        let center = Position::new(2.5, 2.5, 0.7);
        let distance = distance(center, &map);

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
        let center = Position::new(2.5, 2.5, PI - 0.7);
        let distance = distance(center, &map);

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
        let center = Position::new(2.5, 2.5, -0.7);
        let distance = distance(center, &map);

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
        let center = Position::new(2.5, 2.5, PI + 0.7);
        let distance = distance(center, &map);

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
        let center = Position::new(1.5, 1.5, 0.0);
        let distance = distance(center, &map);

        assert_that(&distance).is_none();
    }
}
