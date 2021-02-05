use std::{cmp, collections::HashMap};

use bevy::{prelude::*, render::color};
use cmp::max;

const SPRITE_XPM: [&str; 21] = [
    "16 16 4 1",
    " 	c None",
    ".	c #FFA044",
    "+	c #F84848",
    "@	c #5C40E4",
    " ..   ++++   .. ",
    " ... +@@@@+ ... ",
    " @@ +@@@@@@+ @@ ",
    " @@.@.+..+.@.@@ ",
    " @@...@..@...@@ ",
    "  @@........@@  ",
    "  @@@..@@..@@@  ",
    "  @@@+.@@.+@@@  ",
    "   @++++++++@   ",
    "   @++++++++@   ",
    "   +++@@@@+++   ",
    "   @@@@++@@@@   ",
    "   +++@@@@+++   ",
    "   ++++++++++   ",
    "    @@@  @@@    ",
    "    @@@  @@@    ",
];

const MAGENTA_XPM: [&str; 22] = [
    "20 20 1 1",
    ".	c #E61A80",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
];

const ORANGE_XPM: [&str; 22] = [
    "20 20 1 1",
    ".	c #E6801A",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
];

const BLUE_XPM: [&str; 23] = [
    "20 20 2 1",
    " 	c None",
    ".	c #1A80E6",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "........    ........",
    "........    ........",
    "........    ........",
    "........    ........",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
];

enum GridVoxelMovementType {
    Static,
    Ripple,
    Wave,
}

struct GridVoxel {
    movement_type: GridVoxelMovementType,
    grid_position: f32,
    grid_height: f32,
}

/// Animate all voxel entities based on their movement type.
fn move_grid_voxels() {}

/// Takes xpm image data and converts its pixels into a grid of cube entities.
fn spawn_voxel_grid(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    cube: &Handle<Mesh>,
    xpm_image: &[&str],
    movement_type: GridVoxelMovementType,
    translation: Vec3,
    grid_rotation: Quat,
    grid_scale: f32,
    voxel_scale: f32,
) {
    let mut transformation = Transform::from_rotation(grid_rotation);
    transformation.translation = translation;
    transformation.scale = Vec3::new(grid_scale, grid_scale, grid_scale);

    commands
        .spawn(PbrBundle {
            transform: transformation,
            // mesh: cube.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            // Build palette as hashmap of chars to materials
            let parts: Vec<&str> = xpm_image[0].split(" ").collect();
            let width: usize = parts[0].parse().unwrap();
            let height: usize = parts[1].parse().unwrap();
            let palette_size: usize = parts[2].parse().unwrap();
            let mut palette = HashMap::with_capacity(palette_size);

            for i in 1..=palette_size {
                let row = xpm_image[i];
                let palette_index: char = row.chars().nth(0).unwrap();
                let hex_color: &str = row.split(" ").last().unwrap();

                let material = match hex_color {
                    "None" => None,
                    _ => {
                        let parts: Vec<&str> = hex_color.split("#").collect();
                        let color = Color::hex(parts[1]).unwrap();

                        Some(materials.add(color.into()))
                    }
                };

                palette.insert(palette_index, material);
            }

            let half_width = width as f32 * 0.5;
            let width_offset = half_width - 0.5;
            let half_height = height as f32 * 0.5;
            let height_offset = half_height - 0.5;

            // Ensure that the largest dimension will be scaled into [0, 1].
            let scale_factor = cmp::max(width, height) as f32;

            for h in 0..height {
                let row = xpm_image[h + palette_size + 1];

                for w in 0..width {
                    let palette_index = row.chars().nth(w).unwrap();

                    if let Some(material) = &palette[&palette_index] {
                        let mut transform = Transform::from_translation(Vec3::new(
                            (w as f32 - width_offset) / (width as f32),
                            0.0,
                            (h as f32 - height_offset) / (height as f32),
                        ));

                        transform.scale = Vec3::new(
                            voxel_scale / scale_factor,
                            voxel_scale / scale_factor,
                            voxel_scale / scale_factor,
                        );

                        parent.spawn(PbrBundle {
                            transform,
                            mesh: cube.clone(),
                            material: material.clone(),
                            ..Default::default()
                        });

                        //        c = XpmImage[h + PaletteSize + 1][w];

                        //         if (0.0f != Palette[c].w)
                        //         {
                        //             Material.DiffuseTint = Palette[c];

                        //             // Determine position in imaginary "default" grid
                        //             // Centered at <0,0,0>, facing <0,0,1>
                        //             Position2D.x    = (x / (Width - 1.0f));
                        //             Position2D.y    = (y / (Height - 1.0f));
                        //             BasePosition.x  = Position2D.x * 2.0f - 1.0f;
                        //             BasePosition.y  = Position2D.y * 2.0f - 1.0f;
                        //             BasePosition.z  = 0.0f;

                        //             // Transform the Object's base position to its location in the grid
                        //             // in world-space.
                        //             BasePosition   *= GridScale;
                        //             BasePosition    = vector3(vector4(BasePosition, 1.0f) * Rotation);
                        //             BasePosition   += Translation;

                        //             // Each object must be scales by an amount proportional to the
                        //             // grid dimensions, and allowing for slight gaps to see lighting
                        //             // from the other side of the grid.
                        //             BaseScale = (2.0f * ObjectScale * GridScale) / vector3(Width, Height, Width);

                        //             // Generate entity based on template type
                        //             m_Entities.push_back(new T(Position2D, BasePosition, Rotation, BaseScale, Material, m_EntityModel));
                        //         }
                        //         x += 1.0f;
                        //     }
                        //     x = 0.0f;
                        //     y += 1.0f;
                    }
                }
            }
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
        // .add_plugin(BoardPlugin)
        // .add_plugin(PiecesPlugin)
        // .add_plugin(UiPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(create_scene.system())
        .run();
}

fn setup(commands: &mut Commands) {
    commands
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
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
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}

fn create_scene(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // materials: Res<SquareMaterials>,
) {
    // Add meshes
    let cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    // ---- Voxel grids ----
    // Sprite
    let rotation1 = Quat::from_axis_angle(Vec3::unit_x(), 90f32.to_radians());
    let rotation2 = Quat::from_axis_angle(Vec3::unit_z(), 45f32.to_radians());

    spawn_voxel_grid(
        commands,
        &mut materials,
        &cube,
        &SPRITE_XPM,
        GridVoxelMovementType::Static,
        0.0 * Vec3::unit_z(),
        (rotation1 * rotation2).normalize(),
        0.55,
        1.0,
    );

    // Magenta ripple
    spawn_voxel_grid(
        commands,
        &mut materials,
        &cube,
        &MAGENTA_XPM,
        GridVoxelMovementType::Ripple,
        1.1 * Vec3::unit_x(),
        Quat::from_axis_angle(Vec3::unit_z(), -90f32.to_radians()),
        2.1,
        0.87,
    );

    // Orange ripple
    spawn_voxel_grid(
        commands,
        &mut materials,
        &cube,
        &ORANGE_XPM,
        GridVoxelMovementType::Ripple,
        -1.1 * Vec3::unit_z(),
        Quat::from_axis_angle(Vec3::unit_x(), 90f32.to_radians()),
        2.1,
        0.87,
    );

    // Blue wave
    spawn_voxel_grid(
        commands,
        &mut materials,
        &cube,
        &BLUE_XPM,
        GridVoxelMovementType::Wave,
        -1.1 * Vec3::unit_y(),
        Quat::from_axis_angle(Vec3::unit_z(), 0f32.to_radians()),
        2.1,
        0.87,
    );

    // ---- Pedestal & Braces ----
    let mut transform = Transform::from_translation(-1.0 * Vec3::unit_y());

    transform.apply_non_uniform_scale(2.0 * Vec3::new(0.170, 0.600, 0.170));

    commands
        .spawn(PbrBundle {
            transform: transform,
            mesh: cube.clone(),
            ..Default::default()
        });
}



// afterlights_demo::GeneratePillars(
//     const vector3& Translation,
//     const matrix4x4& Rotation,
//     const float Scale)
// {
//     vector3 BasePosition;
//     vector3 ObjectTranslation;
//     vector3 ObjectScale;
//     material Material;

//     vector3 BasePositions[] = {
//         vector3( 0.0f,  0.0f, 0.1f),
//         vector3(-0.5f, -0.5f, 0.5f),
//         vector3( 0.0f, -0.5f, 0.0f),
//         vector3(-0.5f,  0.0f, 0.0f),
//     };

//     vector3 BaseScales[] = {
//         vector3(0.170f, 0.170f, 0.600f),
//         vector3(0.125f, 0.125f, 1.000f),
//         vector3(1.000f, 0.125f, 0.125f),
//         vector3(0.125f, 1.000f, 0.125f),
//     };

//     Material.MaterialShader = m_EntityShader;
//     Material.DiffuseTint    = GammaToLinear(vector4(0.7f, 0.7f, 0.7f, 1.0f));
//     Material.SpecularTint   = GammaToLinear(vector4(0.3f, 0.3f, 0.3f, 1.0f));
//     Material.EmissiveTint   = GammaToLinear(vector4(0.0f, 0.0f, 0.0f, 1.0f));

//     for (uint32 i = 0; i < COUNT_OF(BasePositions); ++i)
//     {
//         ObjectScale = BaseScales[i] * Scale;
//         BasePosition = BasePositions[i] * Scale;
//         BasePosition = vector3(vector4(BasePosition, 1.0f) * Rotation);
//         ObjectTranslation = BasePosition + Translation;
//         m_Entities.push_back(new static_object(vector2(0.0f), ObjectTranslation, Rotation, ObjectScale, Material, m_EntityModel));
//     }
// }
