use bevy::prelude::*;

use crate::constants::{MAP_N_TILES_WIDTH, TILES_SIZE};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(camera.system());
    }
}

pub struct CameraMarker;

fn camera(mut commands: Commands, windows: Res<Windows>) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    let camera_pos = camera_bundle.transform.translation;

    let new_camera_x = windows.get_primary().map_or(0.0, |window| {
        let camera_pos = camera_bundle.transform.translation;
        let map_width = MAP_N_TILES_WIDTH * TILES_SIZE;
        camera_pos.x - map_width / 2.0 + window.width() / 2.0
    });

    camera_bundle.transform.translation = Vec3::new(new_camera_x, camera_pos.y, camera_pos.z);
    commands.spawn(camera_bundle).with(CameraMarker);
}
