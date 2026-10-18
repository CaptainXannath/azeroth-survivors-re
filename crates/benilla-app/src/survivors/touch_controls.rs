//! Landscape touch controls for iPhone and iPad.
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::Phase;

#[derive(Resource, Default)]
pub(super) struct TouchControls {
    pub(super) movement: Vec2,
    pub(super) jump_pressed: bool,
    pub(super) mobility_pressed: bool,
    pub(super) stick_offset: Vec2,
}

pub(super) fn read_touches(
    touches: Res<Touches>,
    windows: Query<&Window, With<PrimaryWindow>>,
    phase: Res<State<Phase>>,
    mut controls: ResMut<TouchControls>,
) {
    controls.movement = Vec2::ZERO;
    controls.jump_pressed = false;
    controls.mobility_pressed = false;
    controls.stick_offset = Vec2::ZERO;

    if *phase.get() != Phase::Playing {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let size = window.size();
    let stick_center = Vec2::new(size.x * 0.12, size.y * 0.85);
    const STICK_RADIUS: f32 = 58.0;

    if let Some(touch) = touches
        .iter()
        .filter(|touch| (touch.position() - stick_center).length() <= 100.0)
        .min_by(|a, b| {
            (a.position() - stick_center)
                .length_squared()
                .total_cmp(&(b.position() - stick_center).length_squared())
        })
    {
        let delta = touch.position() - stick_center;
        let length = delta.length();

        if length > 10.0 {
            let magnitude = ((length - 10.0) / (STICK_RADIUS - 10.0)).clamp(0.0, 1.0);
            controls.movement = delta.normalize() * magnitude;
            controls.stick_offset = controls.movement * 38.0;
        }
    }

    let jump_center = Vec2::new(size.x * 0.89, size.y * 0.86);
    let mobility_center = Vec2::new(size.x * 0.76, size.y * 0.75);

    for touch in touches.iter_just_pressed() {
        let position = touch.position();
        controls.jump_pressed |= position.distance(jump_center) <= 58.0;
        controls.mobility_pressed |= position.distance(mobility_center) <= 58.0;
    }
}
