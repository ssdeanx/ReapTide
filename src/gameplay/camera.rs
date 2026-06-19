use crate::gameplay::components::*;
use crate::gameplay::resources::*;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use rand::Rng;

// ── Camera Follow & Zoom ──

pub fn camera_follow_and_zoom(
    player_q: Query<&Transform, (With<Player>, Without<GameCamera>)>,
    mut camera_q: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
    mut scroll_events: EventReader<MouseWheel>,
    mut zoom: ResMut<ZoomLevel>,
) {
    // Follow player
    if let Ok(p_tf) = player_q.single() {
        if let Ok(mut c_tf) = camera_q.single_mut() {
            let target = p_tf.translation.truncate();
            let current = c_tf.translation.truncate();
            let smooth = current.lerp(target, crate::constants::CAMERA_FOLLOW_SPEED);
            c_tf.translation.x = smooth.x;
            c_tf.translation.y = smooth.y;
        }
    }

    // Zoom
    for ev in scroll_events.read() {
        zoom.0 = (zoom.0 - ev.y * crate::constants::ZOOM_STEP).clamp(crate::constants::ZOOM_MIN, crate::constants::ZOOM_MAX);
    }

    if let Ok(mut c_tf) = camera_q.single_mut() {
        let target_scale = zoom.0;
        let current_scale = c_tf.scale.x;
        c_tf.scale = Vec3::splat(current_scale + (target_scale - current_scale) * 0.15);
    }
}

// ── Screen Shake ──

pub fn camera_shake(
    time: Res<Time>,
    mut commands: Commands,
    shake: Option<ResMut<ScreenShake>>,
    mut camera_q: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
) {
    let Some(mut s) = shake else { return; };
    s.timer.tick(time.delta());

    if s.timer.just_finished() {
        if let Ok(mut tf) = camera_q.single_mut() {
            tf.translation = s.current_offset;
        }
        commands.remove_resource::<ScreenShake>();
        return;
    }

    if let Ok(mut tf) = camera_q.single_mut() {
        let mut rng = rand::thread_rng();
        let offset = Vec3::new(
            rng.gen_range(-s.magnitude..s.magnitude),
            rng.gen_range(-s.magnitude..s.magnitude),
            0.0,
        );
        tf.translation = s.current_offset + offset;
    }
}

// ── Lighting / Visibility Overlay ──

pub fn update_lighting_overlay(
    mut overlay_q: Query<&mut Transform, (With<DarkOverlay>, Without<GameCamera>)>,
    camera_q: Query<&Transform, (With<GameCamera>, Without<DarkOverlay>)>,
) {
    if let Ok(mut o_tf) = overlay_q.single_mut() {
        if let Ok(c_tf) = camera_q.single() {
            o_tf.translation = c_tf.translation.truncate().extend(o_tf.translation.z);
        }
    }
}
