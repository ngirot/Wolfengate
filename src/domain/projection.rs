use crate::domain::actions::{Actions, ActionState};
use crate::domain::door::{CentralDoor, LateralDoor, Openable};
use crate::domain::index::TextureIndex;
use crate::domain::map::Tile;
use crate::domain::maths::{Angle, ANGLE_DOWN, decimal_part};

use super::{
    coord::{MapPoint, Position},
    map::Map,
};

#[derive(Debug, Copy, Clone)]
pub struct ProjectedPoint {
    source_point: Position,
    projected_point: Position,
    blocking: bool,
    offset_in_bloc: f32,
    map_point: MapPoint,
    texture: TextureIndex,
}

#[derive(Debug, Copy, Clone)]
struct Projection {
    projected_point: Position,
    blocking: bool,
    offset_in_bloc: f32,
    map_point: MapPoint,
    texture: TextureIndex,
}

pub fn project(position: Position, angle: Angle, map: &Map, actions: &Actions) -> Vec<ProjectedPoint> {
    inner_projection(position, angle, map, actions)
        .iter()
        .map(|projection| ProjectedPoint::new(position, *projection))
        .collect()
}

fn inner_projection(position: Position, angle: Angle, map: &Map, actions: &Actions) -> Vec<Projection> {
    projection_concat(position, angle, map, actions, vec![])
}

fn projection_concat(position: Position, angle: Angle, map: &Map, actions: &Actions, previous: Vec<Projection>) -> Vec<Projection> {
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

    let recursive = match bloc_tile {
        None =>
            vec![Projection::new(next_position, position_on_texture, true, bloc, map.border_texture())],
        Some(Tile::SOLID(texture)) =>
            vec![Projection::new(next_position, position_on_texture, true, bloc, *texture)],
        Some(Tile::DYNAMIC(texture_inside, texture_outside, _)) => {
            let projection_inside = projection_on_door(angle, map, actions, next_position, position_on_texture, door_up, bloc, *texture_inside, *texture_outside);
            let projection_behind = inner_projection(next_position, angle, map, actions);
            [projection_inside, projection_behind].concat()
        }
        Some(Tile::NOTHING) => inner_projection(next_position, angle, map, actions)
    };

    [previous, recursive].concat()
}

fn projection_on_door(angle: Angle, map: &Map, actions: &Actions, next_position: Position, position_on_texture: f32, door_up: bool, map_point: MapPoint, texture: TextureIndex, blocking_texture: TextureIndex) -> Vec<Projection> {
    let action_state = actions.state_at(map_point.x(), map_point.y()).unwrap();
    let blocking = action_state.activated_percentage() != 1.0;

    let invisible_wall = vec![Projection::new(next_position, position_on_texture, blocking, map_point, blocking_texture)];

    let actual_door = inner_door_projection(next_position, angle, door_up, map_point, texture, action_state)
        .map_or_else(
            || inner_projection(next_position, angle, map, actions),
            |d| vec![d],
        );

    [invisible_wall, actual_door].concat()
}

fn inner_door_projection(current_position: Position, angle: Angle, door_up: bool, map_point: MapPoint, texture: TextureIndex, action_state: &Box<dyn ActionState>) -> Option<Projection> {
    let opening_percentage = action_state.activated_percentage();
    let position_inside_tile = 0.5;

    if door_up {
        let door = CentralDoor::new(opening_percentage);

        let a = angle.add(ANGLE_DOWN).tan();
        let angle_sign = angle.sin().signum();
        let door_x = current_position.x() - (a / (2.0 * angle_sign));

        let new_position = current_position.with_x(door_x)
            .with_y(current_position.y() + position_inside_tile * angle_sign);

        position_on_texture_inside_tile(Box::new(door), door_x, current_position.x()).map(
            |position_on_texture| Projection::new(new_position, position_on_texture, true, map_point, texture),
        )
    } else {
        let door = LateralDoor::new(opening_percentage);
        let a = angle.tan();
        let angle_sign = angle.cos().signum();
        let door_y = current_position.y() + (a / (2.0 * angle_sign));

        let new_position = current_position.with_x(current_position.x() + position_inside_tile * angle_sign)
            .with_y(door_y);

        position_on_texture_inside_tile(Box::new(door), door_y, current_position.y())
            .map(|position_on_texture| Projection::new(new_position, position_on_texture, true, map_point, texture))
    }
}

fn position_on_texture_inside_tile(door: Box<dyn Openable>, door_position: f32, current_position: f32) -> Option<f32> {
    let right = current_position.ceil();
    let left = current_position.floor();

    if door_position > right || door_position < left {
        return None;
    }

    let offset = decimal_part(door_position);
    let position_on_texture = door.door_column(offset);

    position_on_texture
}


impl ProjectedPoint {
    fn new(source_point: Position, projection: Projection) -> Self {
        Self {
            source_point,
            projected_point: projection.projected_point,
            blocking: projection.blocking,
            offset_in_bloc: projection.offset_in_bloc,
            map_point: projection.map_point,
            texture: projection.texture,
        }
    }

    pub fn distance(&self) -> f32 {
        self.source_point.distance(&self.projected_point)
    }

    pub fn offset_in_bloc(&self) -> f32 {
        self.offset_in_bloc
    }

    pub fn map_point(&self) -> MapPoint {
        self.map_point
    }

    pub fn texture(&self) -> TextureIndex {
        self.texture
    }


    pub fn blocking(&self) -> bool {
        self.blocking
    }
}

impl Projection {
    pub fn new(projected_point: Position, offset_in_bloc: f32, blocking: bool, map_point: MapPoint, texture: TextureIndex) -> Self {
        Self {
            projected_point,
            blocking,
            offset_in_bloc,
            texture,
            map_point,
        }
    }
}

#[cfg(test)]
mod project_test {
    use std::f32::consts::PI;
    use std::time::Duration;

    use spectral::prelude::*;

    use crate::domain::{coord::Position, map::Map};
    use crate::domain::actions::Actions;
    use crate::domain::index::TextureIndex;
    use crate::domain::map::DOOR_OPENING_SPEED_IN_UNITS_PER_SECONDS;
    use crate::domain::map::map_test::default_configuration;
    use crate::domain::maths::{Angle, ANGLE_DOWN, ANGLE_LEFT, ANGLE_RIGHT, ANGLE_UP};
    use crate::domain::projection::ProjectedPoint;

    use super::project;

    fn project_single_wall(position: Position, angle: Angle, map: &Map, actions: &Actions) -> ProjectedPoint {
        let points = project(position, angle, &map, actions);
        assert_that!(points).has_length(1);

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
            default_configuration())
            .unwrap();
        let center = Position::new(2.5, 2.5);
        let projected = project_single_wall(center, ANGLE_UP, &map, &Actions::new(&map));

        assert_that!(projected.distance()).is_close_to(0.5, 0.1)
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
            default_configuration())
            .unwrap();
        let center = Position::new(2.5, 1.3);
        let projected = project_single_wall(center, ANGLE_UP, &map, &Actions::new(&map));

        assert_that!(projected.distance()).is_close_to(1.7, 0.1)
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
            default_configuration())
            .unwrap();
        let center = Position::new(2.5, 2.5);
        let projected = project_single_wall(center, ANGLE_DOWN, &map, &Actions::new(&map));

        assert_that!(projected.distance()).is_close_to(0.5, 0.1)
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
            default_configuration())
            .unwrap();
        let center = Position::new(2.5, 3.2);
        let projected = project_single_wall(center, ANGLE_DOWN, &map, &Actions::new(&map));

        assert_that!(projected.distance()).is_close_to(1.2, 0.1)
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
            default_configuration())
            .unwrap();
        let center = Position::new(2.5, 2.5);
        let projected = project_single_wall(center, ANGLE_LEFT, &map, &Actions::new(&map));

        assert_that!(projected.distance()).is_close_to(0.5, 0.1)
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
            default_configuration())
            .unwrap();
        let center = Position::new(3.1, 2.5);
        let projected = project_single_wall(center, ANGLE_LEFT, &map, &Actions::new(&map));

        assert_that!(projected.distance()).is_close_to(1.1, 0.1)
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
            default_configuration())
            .unwrap();
        let center = Position::new(2.5, 2.5);
        let projected = project_single_wall(center, ANGLE_RIGHT, &map, &Actions::new(&map));

        assert_that!(projected.distance()).is_close_to(0.5, 0.1)
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
            default_configuration())
            .unwrap();
        let center = Position::new(1.1, 2.5);
        let projected = project_single_wall(center, ANGLE_RIGHT, &map, &Actions::new(&map));

        assert_that!(projected.distance()).is_close_to(1.9, 0.1)
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
            default_configuration())
            .unwrap();
        let center = Position::new(2.5, 2.5);
        let projected = project_single_wall(center, Angle::new(0.7), &map, &Actions::new(&map));

        assert_that!(projected.distance()).is_close_to(0.7, 0.1)
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
            default_configuration())
            .unwrap();
        let center = Position::new(2.5, 2.5);
        let projected = project_single_wall(center, Angle::new(PI - 0.7), &map, &Actions::new(&map));

        assert_that!(projected.distance()).is_close_to(0.7, 0.1)
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
            default_configuration())
            .unwrap();
        let center = Position::new(2.5, 2.5);
        let projected = project_single_wall(center, Angle::new(-0.7), &map, &Actions::new(&map));

        assert_that!(projected.distance()).is_close_to(0.7, 0.1)
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
            default_configuration())
            .unwrap();
        let center = Position::new(2.5, 2.5);
        let projected = project_single_wall(center, Angle::new(PI + 0.7), &map, &Actions::new(&map));

        assert_that!(projected.distance()).is_close_to(0.7, 0.1)
    }

    #[test]
    fn should_return_some_void_when_there_is_no_border() {
        let map = Map::new(
            "\
            #####\n\
            #    \n\
            #####",
            default_configuration())
            .unwrap();
        let center = Position::new(1.5, 1.5);
        let projected = project_single_wall(center, ANGLE_RIGHT, &map, &Actions::new(&map));

        assert_that!(projected.offset_in_bloc()).is_close_to(0.5, 0.001);
        assert_that!(projected.texture()).is_equal_to(TextureIndex::VOID);
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
            default_configuration())
            .unwrap();
        let center = Position::new(2.5, 2.5);
        let projected = project_single_wall(center, ANGLE_UP, &map, &Actions::new(&map));

        assert_that!(projected.offset_in_bloc()).is_close_to(0.5, 0.001)
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
            default_configuration())
            .unwrap();
        let center = Position::new(2.5, 2.5);
        let projected = project_single_wall(center, Angle::new(PI / 2.0 + 0.23), &map, &Actions::new(&map));

        assert_that!(projected.offset_in_bloc()).is_close_to(0.382, 0.001)
    }

    #[test]
    fn door_should_be_at_half_distance_top() {
        let map = Map::new(
            "\
            ##D##\n\
            #   #\n\
            #   #\n\
            #####",
            default_configuration())
            .unwrap();
        let center = Position::new(2.5, 2.5);
        let projected = project(center, ANGLE_UP, &map, &Actions::new(&map));

        assert_that!(projected).has_length(3);
        assert_that!(projected[1].distance()).is_close_to(1.0, 0.001)
    }

    #[test]
    fn door_should_be_at_half_distance_down() {
        let map = Map::new(
            "\
            #####\n\
            #   #\n\
            #   #\n\
            ##D##",
            default_configuration())
            .unwrap();
        let center = Position::new(2.5, 2.5);
        let projected = project(center, ANGLE_DOWN, &map, &Actions::new(&map));

        assert_that!(projected).has_length(3);
        assert_that!(projected[1].distance()).is_close_to(2.0, 0.001)
    }

    #[test]
    fn door_should_be_at_half_distance_left() {
        let map = Map::new(
            "\
            ####\n\
            D  #\n\
            ####",
            default_configuration())
            .unwrap();
        let center = Position::new(2.0, 1.1);

        let projected = project(center, ANGLE_LEFT, &map, &Actions::new(&map));

        assert_that!(projected).has_length(3);
        assert_that!(projected[1].distance()).is_close_to(1.5, 0.001)
    }

    #[test]
    fn door_should_be_at_half_distance_right() {
        let map = Map::new(
            "\
            ####\n\
            #  D\n\
            ####",
            default_configuration())
            .unwrap();
        let center = Position::new(2.0, 1.1);
        let projected = project(center, ANGLE_RIGHT, &map, &Actions::new(&map));

        assert_that!(projected).has_length(3);
        assert_that!(projected[1].distance()).is_close_to(1.5, 0.001)
    }

    #[test]
    fn looking_at_a_closed_door_should_not_allow_moves_through_square() {
        let map = Map::new(
            "\
            ####\n\
            #  D\n\
            ####",
            default_configuration())
            .unwrap();
        let center = Position::new(2.0, 1.1);
        let projected = project(center, ANGLE_RIGHT, &map, &Actions::new(&map));

        assert_that!(projected).has_length(3);
        assert_that!(projected[0].distance()).is_close_to(1.0, 0.001);
        assert_that!(projected[0].texture()).is_equal_to(TextureIndex::VOID);
    }

    #[test]
    fn looking_at_a_door_should_draw_wall_behind() {
        let map = Map::new(
            "\
            #####\n\
            #  D#\n\
            #####",
            default_configuration())
            .unwrap();
        let center = Position::new(2.0, 1.1);
        let projected = project(center, ANGLE_RIGHT, &map, &Actions::new(&map));

        assert_that!(projected).has_length(3);
        assert_that!(projected[2].distance()).is_close_to(2.0, 0.001)
    }

    #[test]
    fn should_not_change_distance_when_looking_at_wall_at_side_of_the_door() {
        let door_map = Map::new(
            "\
            ##D##\n\
            #   #\n\
            #####",
            default_configuration())
            .unwrap();

        let no_door_map = Map::new(
            "\
            ## ##\n\
            #   #\n\
            #####",
            default_configuration())
            .unwrap();

        let center = Position::new(2.9, 1.9);
        let projected_door = project(center, Angle::new(PI - 0.2), &door_map, &Actions::new(&door_map));
        let projected_no_door = project_single_wall(center, Angle::new(PI - 0.2), &no_door_map, &Actions::new(&no_door_map));

        assert_that!(projected_door).has_length(3);
        assert_that!(projected_door[1].distance()).is_equal_to(&projected_no_door.distance());
    }

    #[test]
    fn closed_door_should_be_blocking() {
        let map = Map::new(" D    ", default_configuration()).unwrap();
        let position = Position::new(0.5, 0.5);
        let actions = Actions::new(&map);

        let projected = project(position, ANGLE_RIGHT, &map, &actions);

        assert_that!(projected[0].blocking()).is_true();
    }

    #[test]
    fn open_door_should_not_be_blocking() {
        let map = Map::new(" D    ", default_configuration()).unwrap();
        let position = Position::new(0.5, 0.5);
        let mut actions = Actions::new(&map);
        actions.activate(1, 0);
        actions.notify_elapsed(Duration::from_secs(100000).as_micros());

        let projected = project(position, ANGLE_RIGHT, &map, &actions);
        assert_that!(projected[0].blocking()).is_false();
    }

    #[test]
    fn half_open_door_should_be_blocking() {
        let map = Map::new(" D    ", default_configuration()).unwrap();
        let position = Position::new(0.5, 0.5);
        let mut actions = Actions::new(&map);
        actions.activate(1, 0);
        actions.notify_elapsed(((1000000.0 * 0.5) / DOOR_OPENING_SPEED_IN_UNITS_PER_SECONDS) as u128);

        let projected = project(position, ANGLE_RIGHT, &map, &actions);
        assert_that!(projected[0].blocking()).is_true();
    }
}
