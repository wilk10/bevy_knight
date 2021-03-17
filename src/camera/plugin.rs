use bevy::prelude::*;

use crate::constants::TILE_SIZE;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_camera.system());
    }
}

pub struct CameraMarker;

fn spawn_camera(mut commands: Commands, windows: Res<Windows>) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();

    if let Some(window) = windows.get_primary() {
        let camera_pos = camera_bundle.transform.translation;
        let new_x = camera_pos.x + window.width() / 2.0;
        let new_y = camera_pos.y + TILE_SIZE / 2.0;
        let new_camera_pos = Vec3::new(new_x, new_y, camera_pos.z);
        camera_bundle.transform.translation = new_camera_pos;
    } else {
        return;
    }

    commands.spawn(camera_bundle).with(CameraMarker);
}
