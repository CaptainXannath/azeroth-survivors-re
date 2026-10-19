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
    stick_touch: Option<u64>,
    pub(super) stick_origin: Option<Vec2>,
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
        controls.stick_touch = None;
        controls.stick_origin = None;
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let size = window.size();
    let stick_center = Vec2::new(size.x * 0.12, size.y * 0.85);
    let ui_scale = (size.y / 720.0).clamp(0.8, 1.5);
    let stick_radius = 82.0 * ui_scale;

    // Let the floating stick start anywhere in the lower-left half of the screen.
    if controls.stick_touch.is_some_and(|id| {
        touches.just_released(id) || touches.just_canceled(id) || touches.get_pressed(id).is_none()
    }) {
        controls.stick_touch = None;
        controls.stick_origin = None;
    }
    if controls.stick_touch.is_none() {
        if let Some(touch) = touches
            .iter_just_pressed()
            .filter(|touch| {
                let p = touch.position();
                p.x <= size.x * 0.48 && p.y >= size.y * 0.30
            })
            .min_by(|a, b| {
                (a.position() - stick_center)
                    .length_squared()
                    .total_cmp(&(b.position() - stick_center).length_squared())
            })
        {
            controls.stick_touch = Some(touch.id());
            controls.stick_origin = Some(touch.position());
        }
    }

    if let Some(touch) = controls.stick_touch.and_then(|id| touches.get_pressed(id)) {
        // Center the floating stick under the thumb; movement starts only after a drag.
        let delta = touch.position() - controls.stick_origin.unwrap_or_else(|| touch.position());
        let length = delta.length();

        if length > 12.0 * ui_scale {
            let magnitude =
                ((length - 12.0 * ui_scale) / (stick_radius - 12.0 * ui_scale)).clamp(0.0, 1.0);
            controls.movement = delta.normalize() * magnitude;
            controls.stick_offset = controls.movement * 48.0 * ui_scale;
        }
    }

    let jump_center = Vec2::new(size.x * 0.89, size.y * 0.86);
    let mobility_center = Vec2::new(size.x * 0.76, size.y * 0.75);

    for touch in touches.iter_just_pressed() {
        let position = touch.position();
        controls.jump_pressed |= position.distance(jump_center) <= 72.0 * ui_scale;
        controls.mobility_pressed |= position.distance(mobility_center) <= 72.0 * ui_scale;
    }
}
