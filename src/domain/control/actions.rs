use std::fmt::Debug;
use dyn_clone::DynClone;

use crate::domain::actors::actor::SpeedStats;
use crate::domain::maths::between;
use crate::domain::topology::door::{LateralOpening, Openable};
use crate::domain::topology::map::{Map, Tile};

pub struct Actions {
    paving: Vec<Vec<Box<dyn ActionState>>>,
    width: i16,
    height: i16,
}

pub trait ActionState: Sync + DynClone {
    fn elapsed(&self, microseconds: u128) -> Box<dyn ActionState>;
    fn trigger(&self) -> Box<dyn ActionState>;
    fn openable(&self) -> Box<dyn Openable>;
    fn activated_percentage(&self) -> f32;
}
dyn_clone::clone_trait_object!(ActionState);

#[derive(Debug, Clone)]
pub struct ActionStateBuilder {
    default_state: Box<dyn ActionState>,
}

#[derive(Clone)]
pub struct LinearActionState {
    opening_speed: SpeedStats,
    activated: bool,
    opening_percentage: f32,
    openable: Box<dyn Openable>,
}

#[derive(Debug, Copy, Clone)]
pub struct NothingActionState {}

impl Actions {
    pub fn new(map: &Map) -> Self {
        let mut paving = vec![];
        for x in 0..map.width() {
            let mut line = vec![];
            for y in 0..map.height() {
                let current_paving = map.paving_at(x, y).unwrap();
                match current_paving {
                    Tile::DYNAMIC(_, _, state_generator) => line.push(state_generator.build()),
                    _ => line.push(Box::new(NothingActionState::new()))
                }
            }
            paving.push(line);
        }

        Self {
            paving,
            width: map.width(),
            height: map.height(),
        }
    }

    pub fn state_at(&self, x: i16, y: i16) -> Option<&Box<dyn ActionState>> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }

        Some(&self.paving[x as usize][y as usize])
    }

    pub fn activate(&mut self, x: i16, y: i16) {
        let state = self.state_at(x, y);

        if let Some(value) = state {
            let new_state = value.trigger();
            self.paving[x as usize][y as usize] = new_state;
        }
    }

    pub fn notify_elapsed(&mut self, microseconds: u128) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.paving[x as usize][y as usize] = self.paving[x as usize][y as usize].elapsed(microseconds);
            }
        }
    }
}

impl Debug for Box<dyn ActionState> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Action state")
    }
}

impl ActionStateBuilder {
    pub fn new(default_state: Box<dyn ActionState>) -> Self {
        Self {
            default_state,
        }
    }

    pub fn build(&self) -> Box<dyn ActionState> {
        self.default_state.clone()
    }
}

impl LinearActionState {
    pub fn new(opening_speed: SpeedStats, openable: Box<dyn Openable>) -> Self {
        Self {
            activated: false,
            opening_percentage: 0.0,
            openable,
            opening_speed,
        }
    }
}

impl ActionState for LinearActionState {
    fn elapsed(&self, microseconds: u128) -> Box<dyn ActionState> {
        let direction = if self.activated { 1.0 } else { -1.0 };
        let increment = self.opening_speed.to_units(microseconds) * direction;

        let new_percentage = between(0.0, self.opening_percentage + increment, 1.0);

        Box::new(
            Self {
                opening_speed: self.opening_speed,
                activated: self.activated,
                openable: self.openable.clone(),
                opening_percentage: new_percentage,
            }
        )
    }


    fn trigger(&self) -> Box<dyn ActionState> {
        Box::new(
            Self {
                opening_speed: self.opening_speed,
                activated: !self.activated,
                openable: self.openable.clone(),
                opening_percentage: self.opening_percentage,
            }
        )
    }

    fn openable(&self) -> Box<dyn Openable> {
        self.openable.clone()
    }


    fn activated_percentage(&self) -> f32 {
        self.opening_percentage
    }
}

impl NothingActionState {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for NothingActionState {
    fn default() -> Self {
        NothingActionState::new()
    }
}

impl ActionState for NothingActionState {
    fn elapsed(&self, _microseconds: u128) -> Box<dyn ActionState> {
        Box::new(NothingActionState::new())
    }

    fn trigger(&self) -> Box<dyn ActionState> {
        Box::new(Self {})
    }

    fn openable(&self) -> Box<dyn Openable> {
        Box::new(LateralOpening::new())
    }

    fn activated_percentage(&self) -> f32 {
        0.0
    }
}

#[cfg(test)]
mod actions_test {
    use spectral::prelude::*;

    use crate::domain::control::actions::Actions;
    use crate::domain::topology::map::Map;
    use crate::domain::topology::map::map_test::default_configuration;

    #[test]
    fn should_read_paving_information() {
        let map = Map::new("###\n# #\n# #\n###", default_configuration()).unwrap();

        let actions = Actions::new(&map);

        assert_that!(actions.state_at(0, 0)).is_some();
        assert_that!(actions.state_at(1, 0)).is_some();
        assert_that!(actions.state_at(2, 0)).is_some();

        assert_that!(actions.state_at(0, 1)).is_some();
        assert_that!(actions.state_at(1, 1)).is_some();
        assert_that!(actions.state_at(2, 1)).is_some();

        assert_that!(actions.state_at(0, 2)).is_some();
        assert_that!(actions.state_at(1, 2)).is_some();
        assert_that!(actions.state_at(2, 2)).is_some();

        assert_that!(actions.state_at(0, 3)).is_some();
        assert_that!(actions.state_at(1, 3)).is_some();
        assert_that!(actions.state_at(2, 3)).is_some();
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_x_coordinate_bigger_than_width_map() {
        let map = Map::new("  \n  ", default_configuration()).unwrap();
        let actions = Actions::new(&map);

        let state = actions.state_at(0, 2);
        assert_that!(state).is_none()
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_x_coordinate_bigger_than_height_map() {
        let map = Map::new("  \n  ", default_configuration()).unwrap();
        let actions = Actions::new(&map);

        let state = actions.state_at(2, 0);
        assert_that!(state).is_none()
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_negative_x_coordinate() {
        let map = Map::new("  \n  ", default_configuration()).unwrap();
        let actions = Actions::new(&map);

        let state = actions.state_at(-1, 0);
        assert_that!(state).is_none()
    }

    #[test]
    fn should_not_get_paving_information_on_tiles_with_negative_y_coordinate() {
        let map = Map::new("  \n  ", default_configuration()).unwrap();
        let actions = Actions::new(&map);

        let state = actions.state_at(0, -1);
        assert_that!(state).is_none()
    }
}


#[cfg(test)]
mod linear_action_state_test {
    use spectral::prelude::*;

    use crate::domain::actors::actor::SpeedStats;
    use crate::domain::control::actions::ActionState;
    use crate::domain::control::actions::LinearActionState;
    use crate::domain::topology::door::LateralOpening;

    #[test]
    fn should_activate_at_50_percentage_at_mid_timer() {
        let action = LinearActionState::new(SpeedStats::new(1.0), Box::new(LateralOpening::new()))
            .trigger()
            .elapsed(500000);

        assert_that!(action.activated_percentage()).is_close_to(0.5, 0.01);
    }

    #[test]
    fn should_activate_at_25_percentage_at_quarter_timer() {
        let action = LinearActionState::new(SpeedStats::new(0.5), Box::new(LateralOpening::new()))
            .trigger()
            .elapsed(500000);

        assert_that!(action.activated_percentage()).is_close_to(0.25, 0.01);
    }

    #[test]
    fn percentage_should_not_go_below_0() {
        let action = LinearActionState::new(SpeedStats::new(999.0), Box::new(LateralOpening::new()))
            .elapsed(99999999999999999999999);

        assert_that!(action.activated_percentage()).is_equal_to(0.0);
    }

    #[test]
    fn percentage_should_not_go_above_1() {
        let action = LinearActionState::new(SpeedStats::new(999.0), Box::new(LateralOpening::new()))
            .trigger()
            .elapsed(99999999999999999999999);

        assert_that!(action.activated_percentage()).is_equal_to(1.0);
    }

    #[test]
    fn should_keep_opening_percentage_when_reactivating_before_previous_state_finished() {
        let action = LinearActionState::new(SpeedStats::new(1.0), Box::new(LateralOpening::new()))
            .trigger()
            .elapsed(500000)
            .trigger()
            .elapsed(250000);

        assert_that!(action.activated_percentage()).is_close_to(0.25, 0.01);
    }
}
