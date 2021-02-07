use bevy::prelude::*;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};

mod grids;
use grids::*;

mod main_camera;
use main_camera::*;

mod light_rings;
use light_rings::*;

mod shared;
use shared::*;

mod ui;
use ui::*;

use lazy_static::*;

lazy_static! {
    static ref DESCRIPTIONS: [Mat4; 4] = {
        [
            // Pedestal
            Mat4::from_scale_rotation_translation(
                Vec3::new(0.34, 0.7, 0.34),
                Quat::identity(),
                Vec3::new(0.0, -0.75, 0.0),
            ),
            // X-column
            Mat4::from_scale_rotation_translation(
                Vec3::new(2.0, 0.125, 0.125),
                Quat::identity(),
                Vec3::new(-0.05, -1.0, -1.0),
            ),
            // Y-column
            Mat4::from_scale_rotation_translation(
                Vec3::new(0.125, 2.0, 0.125),
                Quat::identity(),
                Vec3::new(1.0, 0.05, -1.0),
            ),
            // Z-column
            Mat4::from_scale_rotation_translation(
                Vec3::new(0.125, 0.125, 2.0),
                Quat::identity(),
                Vec3::new(1.0, -1.0, 0.05),
            ),
        ]
    };
}

fn setup(
    commands: &mut Commands,
    shared_data: Res<SharedData>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ---- Pedestal & columns ----
    let material = materials.add(Color::rgb(0.7, 0.7, 0.7).into());

    for d in DESCRIPTIONS.iter() {
        commands.spawn(PbrBundle {
            transform: Transform::from_matrix(*d),
            material: material.clone(),
            mesh: shared_data.cube.clone(),
            ..Default::default()
        });
    }
}

#[bevy_main]
fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Cubism".to_string(),
            width: 1280.,
            height: 720.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(PrintDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_system(PrintDiagnosticsPlugin::print_diagnostics_system.system())
        .init_resource::<SharedData>()
        .add_plugin(GridsPlugin)
        .add_plugin(MainCameraPlugin)
        .add_plugin(LightRingsPlugin)
        .add_plugin(UiPlugin)
        .add_startup_system(setup.system())
        .run();
}
