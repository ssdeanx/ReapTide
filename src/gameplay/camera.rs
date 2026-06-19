use crate::gameplay::components::*;
use crate::gameplay::resources::*;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use rand::Rng;

/// Marker component for the isometric camera.
#[derive(Component)]
pub struct IsometricCamera;

/// Spawn the 3D isometric camera, directional light, and ground plane.
/// Runs once at startup before any Playing-state systems.
pub fn spawn_scene(mut commands: Commands, meshes: Res<GameMeshes>, materials: Res<GameMaterials>) {
    use crate::constants::*;

    // ── Ground Plane ──
    commands.spawn((
        Mesh3d(meshes.ground.clone()),
        MeshMaterial3d(materials.ground.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // ── Directional Light ──
    commands.spawn((
        DirectionalLight {
            illuminance: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(200.0, ISO_CAMERA_HEIGHT, 200.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // ── Ambient (fill) light ──
    commands.spawn((
        PointLight {
            intensity: 500_000.0,
            color: Color::srgb(0.3, 0.4, 0.6),
            range: 3000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 800.0, 0.0),
    ));

    // ── Isometric Camera ──
    let iso_dist = ISO_CAMERA_HEIGHT;
    commands.spawn((
        Camera3d::default(),
        Camera::default(),
        Projection::Orthographic(OrthographicProjection {
            scale: ZOOM_DEFAULT,
            scaling_mode: ScalingMode::WindowSize(1.0),
            near: ORTHO_NEAR,
            far: ORTHO_FAR,
            ..default()
        }),
        Tonemapping::None,
        IsometricCamera,
        Transform::from_xyz(0.0, iso_dist, iso_dist).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// ── Camera Follow & Zoom ──

pub fn camera_follow_and_zoom(
    player_q: Query<&Transform, (With<Player>, Without<IsometricCamera>)>,
    mut camera_q: Query<&mut Transform, (With<IsometricCamera>, Without<Player>)>,
    mut scroll_events: MessageReader<MouseWheel>,
    mut zoom: ResMut<ZoomLevel>,
) {
    // Follow player (XZ plane follow, Y height stays fixed)
    if let Ok(p_tf) = player_q.single() {
        if let Ok(mut c_tf) = camera_q.single_mut() {
            let target_xz = Vec2::new(p_tf.translation.x, p_tf.translation.z);
            let current_xz = Vec2::new(c_tf.translation.x, c_tf.translation.z);
            let smooth = current_xz.lerp(target_xz, crate::constants::CAMERA_FOLLOW_SPEED);
            c_tf.translation.x = smooth.x;
            c_tf.translation.z = smooth.y;
        }
    }

    // Zoom input
    for ev in scroll_events.read() {
        zoom.0 = (zoom.0 - ev.y * crate::constants::ZOOM_STEP)
            .clamp(crate::constants::ZOOM_MIN, crate::constants::ZOOM_MAX);
    }
}

// ── Zoom via OrthographicProjection scale ──

pub fn update_zoom(
    mut camera_q: Query<&mut Projection, With<IsometricCamera>>,
    zoom: Res<ZoomLevel>,
) {
    if let Ok(mut proj) = camera_q.single_mut() {
        if let Projection::Orthographic(ref mut ortho) = *proj {
            let target = zoom.0;
            ortho.scale += (target - ortho.scale) * 0.15;
        }
    }
}

// ── Screen Shake ──

pub fn camera_shake(
    time: Res<Time>,
    mut commands: Commands,
    shake: Option<ResMut<ScreenShake>>,
    mut camera_q: Query<&mut Transform, (With<IsometricCamera>, Without<Player>)>,
) {
    let Some(mut s) = shake else {
        return;
    };
    s.timer.tick(time.delta());

    if s.timer.just_finished() {
        if let Ok(mut tf) = camera_q.single_mut() {
            tf.translation = s.current_offset;
        }
        commands.remove_resource::<ScreenShake>();
        return;
    }

    if let Ok(mut tf) = camera_q.single_mut() {
        let mut rng = rand::rng();
        let offset = Vec3::new(
            rng.gen_range(-s.magnitude..s.magnitude),
            0.0,
            rng.gen_range(-s.magnitude..s.magnitude),
        );
        tf.translation = s.current_offset + offset;
    }
}
