use bevy::prelude::*;

mod grids;
use grids::*;

mod light_rings;
use light_rings::*;

mod shared;
use shared::*;

use lazy_static::*;

lazy_static! {
    static ref DESCRIPTIONS: [Mat4; 4] = {
        [
            Mat4::from_scale_rotation_translation(
                Vec3::new(0.340, 1.200, 0.340),
                Quat::identity(),
                Vec3::new(0.0, -1.0, 0.0),
            ),
            Mat4::from_scale_rotation_translation(
                Vec3::new(0.125, 2.0, 0.125),
                Quat::identity(),
                Vec3::new(1.0, 0.05, -1.0),
            ),
            Mat4::from_scale_rotation_translation(
                Vec3::new(0.125, 0.125, 2.0),
                Quat::identity(),
                Vec3::new(1.0, -1.0, 0.05),
            ),
            Mat4::from_scale_rotation_translation(
                Vec3::new(2.0, 0.125, 0.125),
                Quat::identity(),
                Vec3::new(-0.05, -1.0, -1.0),
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

    commands
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                (Quat::from_axis_angle(Vec3::unit_y(), -45f32.to_radians())
                    * Quat::from_axis_angle(Vec3::unit_x(), -30f32.to_radians()))
                .normalize(),
                Vec3::new(-3.0, 2.25, 3.0),
            )),
            // transform: Transform::from_matrix(Mat4::look_at_rh(
            //     Vec3::new(0.0, 0.0, -5.0),
            //     Vec3::new(0.0, 0.0, 0.0), //?
            //     Vec3::unit_y(),
            // )),
            ..Default::default()
        })
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(-4.0, 6.0, 4.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: shared_data.cube.clone(),
                ..Default::default()
            });
        });
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
        .init_resource::<SharedData>()
        .add_plugin(GridsPlugin)
        .add_plugin(LightRingsPlugin)
        .add_startup_system(setup.system())
        .run();
}
