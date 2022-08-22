use super::Player;
use crate::config::PlayerConfig;
use crate::prelude::*;
use bevy::{
    ecs::event::{Events, ManualEventReader},
    input::mouse::MouseMotion,
};

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Default)]
pub struct MouseInputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

pub fn build(app: &mut App) {
    app.init_resource::<MouseInputState>()
        .add_system(Player::look.run_in_state(GameState::MainMenu));
}

impl Player {
    pub fn look(
        config: Res<PlayerConfig>,
        windows: Res<Windows>,
        mut state: ResMut<MouseInputState>,
        motion: Res<Events<MouseMotion>>,
        mut query: Query<&mut Transform, With<Player>>,
    ) {
        let window = match windows.get_primary() {
            Some(w) => w,
            None => {
                warn!("Primary window not found for `player_look`!");
                return;
            }
        };

        let mut delta_state = state.as_mut();
        let mut player_trans = query.single_mut();

        for ev in delta_state.reader_motion.iter(&motion) {
            if window.cursor_locked() {
                // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                let window_scale = window.height().min(window.width());
                delta_state.pitch -= (config.mouse_sens * ev.delta.y * window_scale).to_radians();
                delta_state.yaw -= (config.mouse_sens * ev.delta.x * window_scale).to_radians();
            }

            delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

            // Order is important to prevent unintended roll
            player_trans.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
        }
    }
}