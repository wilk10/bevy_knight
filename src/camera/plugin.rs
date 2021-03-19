use bevy::prelude::*;

use crate::constants::Const;

pub struct CameraMarker;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_camera.system());
    }
}

fn spawn_camera(mut commands: Commands, windows: Res<Windows>) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    let window = windows
        .get_primary()
        .expect("could not find any primary window");

    let camera_pos = camera_bundle.transform.translation;
    let new_x = camera_pos.x + window.width() / 2.0;
    let new_y = camera_pos.y + Const::tile_size().y / 2.0;
    let new_camera_pos = Vec3::new(new_x, new_y, camera_pos.z);
    camera_bundle.transform.translation = new_camera_pos;

    commands.spawn(camera_bundle).with(CameraMarker);
}
