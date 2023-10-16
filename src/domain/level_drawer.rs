use rayon::prelude::*;

use crate::domain::actors::actor::Enemy;
use crate::domain::actors::shoot::{ShootState, Weapon};
use crate::domain::control::actions::Actions;
use crate::domain::maths::{Angle, Vector};
use crate::domain::topology::coord::{Position, ScreenPoint};
use crate::domain::topology::map::Map;
use crate::domain::topology::projection::{project, ProjectedPoint};
use crate::domain::ui::color::Color;
use crate::domain::ui::draw_action::DrawAction;
use crate::domain::ui::view::ViewScreen;

pub struct DrawActionZIndex {
    action: DrawAction,
    z_index: f32,
}

pub fn build_clear_actions() -> Vec<DrawAction> {
    vec![DrawAction::Clear(Color::new(0, 0, 0))]
}

pub fn build_background_actions(view: ViewScreen) -> Vec<DrawAction> {
    let height: i32 = view.height();
    let width: i32 = view.width();
    let mid_screen = height / 2;

    vec![
        DrawAction::Rectangle(
            ScreenPoint::new(0, 0),
            ScreenPoint::new(width, mid_screen),
            Color::new(50, 50, 50),
        ),
        DrawAction::Rectangle(
            ScreenPoint::new(0, mid_screen),
            ScreenPoint::new(width, height),
            Color::new(100, 100, 100),
        ),
    ]
}

pub fn build_walls(
    view: ViewScreen,
    position: &Position,
    view_angle: Angle,
    map: &Map,
    actions: &Actions,
) -> Vec<DrawActionZIndex> {
    view_angle.discreet_cone_straight_space(view.angle(), view.width())
        .par_iter()
        .enumerate()
        .map(|(i, angle)| (i, project(*position, *angle, map, actions)))
        .flat_map(|(i, projected)| projected.into_iter().map(|p| (i, p)).collect::<Vec<(usize, ProjectedPoint)>>())
        .map(|(i, projected_point)| {
            let screen_length: i32 = view.height();

            let cartesian_distance = projected_point.distance();
            let distance_for_height = projected_point.distance_no_fish_eye(view_angle);
            let wall_height = object_height(view, distance_for_height);
            let start = ScreenPoint::new(
                i as i32,
                (screen_length as f32 / 2.0 - wall_height / 2.0) as i32,
            );
            let end = ScreenPoint::new(
                i as i32,
                (screen_length as f32 / 2.0 + wall_height / 2.0) as i32,
            );

            let action = DrawAction::TexturedLine(
                start,
                end,
                projected_point.texture(),
                projected_point.offset_in_bloc(),
            );
            DrawActionZIndex::new(action, cartesian_distance)
        })
        .collect()
}

pub fn build_weapons(view: ViewScreen, weapon: Weapon) -> DrawAction {
    let state = weapon.state();

    let texture = match state {
        ShootState::Startup => weapon.configuration().startup().texture(),
        ShootState::Active => weapon.configuration().active().texture(),
        ShootState::Recovery => weapon.configuration().recovery().texture(),
        ShootState::Finished => weapon.configuration().default(),
    };

    DrawAction::Sprite(
        ScreenPoint::new(0, 0),
        ScreenPoint::new(view.width(), view.height()),
        texture,
    )
}

pub fn build_enemies(
    view: ViewScreen,
    view_position: Position,
    orientation: &Angle,
    enemies: &Vec<Enemy>,
) -> Vec<DrawActionZIndex> {
    let mut actions = vec![];
    for enemy in enemies {
        let view_vector = Vector::new(
            view_position,
            Position::new(
                view_position.x() + orientation.cos(),
                view_position.y() + orientation.sin(),
            ),
        );
        let enemy_vector = Vector::new(view_position, enemy.position());

        let angle = view_vector.angle(enemy_vector).unwrap();

        let sprite = enemy.position().with_reference_point(&view_position);
        let sprite_projection = angle.position_in_discreet_cone_straight(&view, orientation, sprite);

        if sprite_projection.is_some() {
            let projected = sprite_projection.unwrap();
            let sprite_height = object_height(view, projected.distance());
            let start = ScreenPoint::new(
                (projected.column() - sprite_height / 2.0) as i32,
                (view.height() as f32 / 2.0 - sprite_height / 2.0) as i32,
            );
            let end = ScreenPoint::new(
                (projected.column() + sprite_height / 2.0) as i32,
                (view.height() as f32 / 2.0 + sprite_height / 2.0) as i32,
            );

            let action = DrawAction::Sprite(start, end, enemy.texture());
            actions.push(DrawActionZIndex::new(action, projected.distance()))
        }
    }
    actions
}

fn object_height(view: ViewScreen, distance: f32) -> f32 {
    (view.height() as f32 * view.ratio()) / distance
}

impl DrawActionZIndex {
    pub fn new(action: DrawAction, z_index: f32) -> Self {
        Self { action, z_index }
    }

    pub fn action(&self) -> &DrawAction {
        &self.action
    }

    pub fn z_index(&self) -> f32 {
        self.z_index
    }
}
