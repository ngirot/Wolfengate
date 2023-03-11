use crate::domain::index::TextureIndex;
use crate::domain::maths::{Angle, ANGLE_DOWN};

use super::{
    coord::{MapPoint, Position},
    map::{Map, Tile},
};

#[derive(Debug, Copy, Clone)]
pub struct ProjectedPoint {
    distance: f32,
    offset_in_bloc: f32,
    tile_type: TextureIndex,
}

pub fn distance(position: Position, angle: Angle, map: &Map) -> Vec<ProjectedPoint> {
    distance_concat(position, angle, map, vec![])
}

pub fn distance_concat(position: Position, angle: Angle, map: &Map, previous: Vec<ProjectedPoint>) -> Vec<ProjectedPoint> {
    let direction_x = angle.cos().signum();
    let direction_y = angle.sin().signum();

    let next_x_position = position.projection_x(angle);
    let next_y_position = position.projection_y(angle);

    let distance_to_next_x = position.distance(&next_x_position);
    let distance_to_next_y = position.distance(&next_y_position);

    let bloc: MapPoint;
    let next_position;
    let position_on_texture;
    let door_up;
    if distance_to_next_x < distance_to_next_y {
        bloc = next_x_position.to_map_point(direction_x, 0.0);
        position_on_texture = decimal_part(next_x_position.y());
        door_up = false;
        next_position = next_x_position;
    } else {
        bloc = next_y_position.to_map_point(0.0, direction_y);
        position_on_texture = decimal_part(next_y_position.x());
        door_up = true;
        next_position = next_y_position
    };

    let bloc_tile = map.paving_at(bloc.x(), bloc.y());

    let distance_total = position.distance(&next_position);
    let recursive = match bloc_tile {
        None =>
            vec![ProjectedPoint::invisible(distance_total, position_on_texture)],
        Some(Tile::Wall) =>
            vec![ProjectedPoint::visible(distance_total, position_on_texture, TextureIndex::WALL)],
        Some(Tile::Glass) => {
            let distances_glass_tile = distance_on_door(angle, map, next_position, position_on_texture, door_up, distance_total, TextureIndex::GLASS);
            let distances_behind = distance(next_position, angle, map).iter().map(|projected| projected.with_distance_added(distance_total)).collect();
            [distances_glass_tile, distances_behind].concat()
        }
        Some(Tile::Door) => {
            let distances_door_tile = distance_on_door(angle, map, next_position, position_on_texture, door_up, distance_total, TextureIndex::DOOR);
            let distances_behind = distance(next_position, angle, map).iter().map(|projected| projected.with_distance_added(distance_total)).collect();
            [distances_door_tile, distances_behind].concat()
        }
        _ =>
            distance(next_position, angle, map)
                .iter()
                .map(|projected| projected.with_distance_added(distance_total))
                .collect(),
    };

    [previous, recursive].concat()
}

fn distance_on_door(angle: Angle, map: &Map, next_position: Position, position_on_texture: f32, door_up: bool, distance_total: f32, texture: TextureIndex) -> Vec<ProjectedPoint> {
    let invisible_wall = vec![ProjectedPoint::invisible(distance_total, position_on_texture)];

    let actual_door = inner_door_projection(next_position, angle, door_up, texture)
        .map_or_else(
            || distance(next_position, angle, map).iter().map(|projected| projected.with_distance_added(distance_total)).collect(),
            |d| vec![ProjectedPoint::visible(distance_total + d.distance(), d.offset_in_bloc(), d.tile_type())],
        );

    [invisible_wall, actual_door].concat()
}

fn inner_door_projection(next_position: Position, angle: Angle, door_up: bool, texture: TextureIndex) -> Option<ProjectedPoint> {
    let opening_percentage = 0.0;

    if door_up {
        let a = angle.add(ANGLE_DOWN).tan();
        let door_x = next_position.x() - (a / (2.0 * angle.sin().signum()));

        let new_position = next_position.with_x(door_x)
            .with_y(next_position.y() + 0.5);
        let distance = new_position
            .distance(&next_position);

        if new_position.x() > next_position.x().ceil() - opening_percentage || new_position.x() < next_position.x().floor() {
            return None;
        }

        let position_on_texture = decimal_part(new_position.x());

        Some(ProjectedPoint::visible(distance, position_on_texture - opening_percentage, texture))
    } else {
        let a = angle.tan();
        let door_y = next_position.y() + (a / (2.0 * angle.cos().signum()));

        let new_position = next_position.with_x(next_position.x() + 0.5)
            .with_y(door_y);
        let distance = new_position
            .distance(&next_position);

        if new_position.y() > next_position.y().ceil() - opening_percentage || new_position.y() < next_position.y().floor() {
            return None;
        }

        let position_on_texture = decimal_part(new_position.y());

        Some(ProjectedPoint::visible(distance, position_on_texture - opening_percentage, texture))
    }
}

impl ProjectedPoint {
    pub fn invisible(distance: f32, offset_in_bloc: f32) -> Self {
        Self {
            distance,
            offset_in_bloc,
            tile_type: TextureIndex::VOID,
        }
    }

    pub fn visible(distance: f32, offset_in_bloc: f32, tile_type: TextureIndex) -> Self {
        Self {
            distance,
            offset_in_bloc,
            tile_type,
        }
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn offset_in_bloc(&self) -> f32 {
        self.offset_in_bloc
    }

    pub fn tile_type(&self) -> TextureIndex {
        self.tile_type
    }

    fn with_distance_added(&self, distance_to_add: f32) -> Self {
        ProjectedPoint {
            distance: self.distance + distance_to_add,
            offset_in_bloc: self.offset_in_bloc,
            tile_type: self.tile_type,
        }
    }
}

fn decimal_part(number: f32) -> f32 {
    number.ceil() - number
}

#[cfg(test)]
mod distance_test {
    use std::f32::consts::PI;

    use spectral::prelude::*;

    use crate::domain::{coord::Position, map::Map};
    use crate::domain::distance::ProjectedPoint;
    use crate::domain::index::TextureIndex;
    use crate::domain::maths::{Angle, ANGLE_DOWN, ANGLE_LEFT, ANGLE_RIGHT, ANGLE_UP};

    use super::distance;

    fn distance_single_wall(position: Position, angle: Angle, map: &Map) -> ProjectedPoint {
        let points = distance(position, angle, &map);
        assert_that(&points).has_length(1);

        points[0]
    }

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
        let distance = distance_single_wall(center, ANGLE_UP, &map);

        assert_that(&distance.distance()).is_close_to(0.5, 0.1)
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
        let distance = distance_single_wall(center, ANGLE_UP, &map);

        assert_that(&distance.distance()).is_close_to(1.7, 0.1)
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
        let distance = distance_single_wall(center, ANGLE_DOWN, &map);

        assert_that(&distance.distance()).is_close_to(0.5, 0.1)
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
        let distance = distance_single_wall(center, ANGLE_DOWN, &map);

        assert_that(&distance.distance()).is_close_to(1.2, 0.1)
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
        let distance = distance_single_wall(center, ANGLE_LEFT, &map);

        assert_that(&distance.distance()).is_close_to(0.5, 0.1)
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
        let distance = distance_single_wall(center, ANGLE_LEFT, &map);

        assert_that(&distance.distance()).is_close_to(1.1, 0.1)
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
        let distance = distance_single_wall(center, ANGLE_RIGHT, &map);

        assert_that(&distance.distance()).is_close_to(0.5, 0.1)
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
        let distance = distance_single_wall(center, ANGLE_RIGHT, &map);

        assert_that(&distance.distance()).is_close_to(1.9, 0.1)
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
        let distance = distance_single_wall(center, Angle::new(0.7), &map);

        assert_that(&distance.distance()).is_close_to(0.7, 0.1)
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
        let distance = distance_single_wall(center, Angle::new(PI - 0.7), &map);

        assert_that(&distance.distance()).is_close_to(0.7, 0.1)
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
        let distance = distance_single_wall(center, Angle::new(-0.7), &map);

        assert_that(&distance.distance()).is_close_to(0.7, 0.1)
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
        let distance = distance_single_wall(center, Angle::new(PI + 0.7), &map);

        assert_that(&distance.distance()).is_close_to(0.7, 0.1)
    }

    #[test]
    fn should_return_some_void_when_there_is_no_border() {
        let map = Map::new(
            "\
            #####\n\
            #    \n\
            #####",
        )
            .unwrap();
        let center = Position::new(1.5, 1.5);
        let distance = distance_single_wall(center, ANGLE_RIGHT, &map);

        assert_that(&distance.offset_in_bloc()).is_close_to(0.5, 0.001);
        assert_that(&distance.tile_type()).is_equal_to(TextureIndex::VOID);
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
        let distance = distance_single_wall(center, ANGLE_UP, &map);

        assert_that(&distance.offset_in_bloc()).is_close_to(0.5, 0.001)
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
        let distance = distance_single_wall(center, Angle::new(PI / 2.0 + 0.23), &map);

        assert_that(&distance.offset_in_bloc()).is_close_to(0.617, 0.001)
    }

    #[test]
    fn door_should_be_at_half_distance_top() {
        let map = Map::new(
            "\
            ##D##\n\
            #   #\n\
            #   #\n\
            #####",
        )
            .unwrap();
        let center = Position::new(2.5, 2.5);
        let distances = distance(center, ANGLE_UP, &map);

        assert_that(&distances).has_length(3);
        assert_that(&distances[1].distance()).is_close_to(1.0, 0.001)
    }

    #[test]
    fn door_should_be_at_half_distance_down() {
        let map = Map::new(
            "\
            #####\n\
            #   #\n\
            #   #\n\
            ##D##",
        )
            .unwrap();
        let center = Position::new(2.5, 2.5);
        let distances = distance(center, ANGLE_DOWN, &map);

        assert_that(&distances).has_length(3);
        assert_that(&distances[1].distance()).is_close_to(2.0, 0.001)
    }

    #[test]
    fn door_should_be_at_half_distance_left() {
        let map = Map::new(
            "\
            ####\n\
            D  #\n\
            ####",
        )
            .unwrap();
        let center = Position::new(2.0, 1.1);

        let distances = distance(center, ANGLE_LEFT, &map);

        assert_that(&distances).has_length(3);
        assert_that(&distances[1].distance()).is_close_to(1.5, 0.001)
    }

    #[test]
    fn door_should_be_at_half_distance_right() {
        let map = Map::new(
            "\
            ####\n\
            #  D\n\
            ####",
        )
            .unwrap();
        let center = Position::new(2.0, 1.1);
        let distances = distance(center, ANGLE_RIGHT, &map);

        assert_that(&distances).has_length(3);
        assert_that(&distances[1].distance()).is_close_to(1.5, 0.001)
    }

    #[test]
    fn looking_at_a_closed_door_should_not_allow_moves_through_square() {
        let map = Map::new(
            "\
            ####\n\
            #  D\n\
            ####",
        )
            .unwrap();
        let center = Position::new(2.0, 1.1);
        let distances = distance(center, ANGLE_RIGHT, &map);

        assert_that(&distances).has_length(3);
        let projected = distances[0];
        assert_that(&projected.distance()).is_close_to(1.0, 0.001);
        assert_that(&projected.tile_type).is_equal_to(TextureIndex::VOID);
    }

    #[test]
    fn looking_at_a_door_should_draw_wall_behind() {
        let map = Map::new(
            "\
            #####\n\
            #  D#\n\
            #####",
        )
            .unwrap();
        let center = Position::new(2.0, 1.1);
        let distances = distance(center, ANGLE_RIGHT, &map);

        assert_that(&distances).has_length(3);
        assert_that(&distances[2].distance()).is_close_to(2.0, 0.001)
    }

    #[test]
    fn should_not_change_distance_when_looking_at_wall_at_side_of_the_door() {
        let door_map = Map::new(
            "\
            ##D##\n\
            #   #\n\
            #####",
        )
            .unwrap();

        let no_door_map = Map::new(
            "\
            ## ##\n\
            #   #\n\
            #####",
        )
            .unwrap();

        let center = Position::new(2.9, 1.9);
        let distances_door = distance(center, Angle::new(PI - 0.2), &door_map);
        let distance_no_door = distance_single_wall(center, Angle::new(PI - 0.2), &no_door_map);

        assert_that(&distances_door).has_length(3);
        assert_that(&distances_door[1].distance()).is_equal_to(&distance_no_door.distance());
    }
}
