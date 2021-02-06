use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
};

mod grids;
use grids::*;

mod light_rings;
use light_rings::*;

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

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127e"]
pub struct EmissiveMaterial {
    pub color: Color,
}

fn setup(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut emissive_materials: ResMut<Assets<EmissiveMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Load cube mesh
    let cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    // Create a new shader pipeline
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("./shaders/emissive_material.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("./shaders/emissive_material.frag"),
        ))),
    }));

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind EmissiveMaterial resources to our shader
    render_graph.add_system_node(
        "emissive_material",
        AssetRenderResourcesNode::<EmissiveMaterial>::new(true),
    );

    // Add a Render Graph edge connecting our new "emissive_material" node to the main pass node. This ensures "emissive_material" runs before the main pass
    render_graph
        .add_node_edge("emissive_material", base::node::MAIN_PASS)
        .unwrap();

    // ---- Voxel grids ----
    // Sprite
    spawn_voxel_grid(
        commands,
        &mut materials,
        &cube,
        1.0,
        &SPRITE_XPM,
        GridVoxelMovementType::Static,
        Mat4::from_scale_rotation_translation(
            Vec3::splat(0.55),
            (Quat::from_axis_angle(Vec3::unit_x(), 90f32.to_radians())
                * Quat::from_axis_angle(Vec3::unit_z(), 45f32.to_radians()))
            .normalize(),
            -0.125 * Vec3::unit_y(),
        ),
    );

    let voxel_scale = 0.87;
    let grid_scale = Vec3::splat(1.8);

    // Magenta ripple
    spawn_voxel_grid(
        commands,
        &mut materials,
        &cube,
        voxel_scale,
        &MAGENTA_XPM,
        GridVoxelMovementType::Ripple,
        Mat4::from_scale_rotation_translation(
            grid_scale,
            Quat::from_axis_angle(Vec3::unit_z(), -90f32.to_radians()),
            Vec3::unit_x(),
        ),
    );

    // Orange ripple
    spawn_voxel_grid(
        commands,
        &mut materials,
        &cube,
        voxel_scale,
        &ORANGE_XPM,
        GridVoxelMovementType::Ripple,
        Mat4::from_scale_rotation_translation(
            grid_scale,
            (Quat::from_axis_angle(Vec3::unit_x(), 90f32.to_radians())
                * Quat::from_axis_angle(Vec3::unit_z(), 180f32.to_radians()))
            .normalize(),
            -Vec3::unit_z(),
        ),
    );

    // Blue wave
    spawn_voxel_grid(
        commands,
        &mut materials,
        &cube,
        voxel_scale,
        &BLUE_XPM,
        GridVoxelMovementType::Wave,
        Mat4::from_scale_rotation_translation(
            grid_scale,
            Quat::from_axis_angle(Vec3::unit_y(), -90f32.to_radians()),
            -Vec3::unit_y(),
        ),
    );

    // ---- Voxel light rings ----
    // Green-yellow light ring
    spawn_voxel_light_ring(
        commands,
        &mut emissive_materials,
        &pipeline_handle,
        &cube,
        200,
        0.5,
        0.4,
        0.7,
        Color::rgb(0.3, 0.3, 0.05),
        Color::rgb(0.6, 0.7, 0.1),
        Mat4::from_translation(-0.55 * Vec3::unit_y()),
    );

    // Cyan light ring
    spawn_voxel_light_ring(
        commands,
        &mut emissive_materials,
        &pipeline_handle,
        &cube,
        100,
        0.125,
        0.4,
        1.0,
        Color::rgb(0.05, 0.4, 0.5),
        Color::rgb(0.1, 0.5, 0.7),
        Mat4::from_translation(-1.2 * Vec3::unit_y()),
    );

    // Orange light ring
    spawn_voxel_light_ring(
        commands,
        &mut emissive_materials,
        &pipeline_handle,
        &cube,
        100,
        0.125,
        0.25,
        1.0,
        Color::rgb(0.5, 0.4, 0.05),
        Color::rgb(0.6, 0.5, 0.1),
        Mat4::from_rotation_translation(
            Quat::from_axis_angle(Vec3::unit_x(), 90f32.to_radians()),
            -1.2 * Vec3::unit_z(),
        ),
    );

    // Magenta light ring
    spawn_voxel_light_ring(
        commands,
        &mut emissive_materials,
        &pipeline_handle,
        &cube,
        100,
        0.125,
        0.25,
        1.0,
        Color::rgb(0.1, 0.1, 0.5),
        Color::rgb(0.6, 0.2, 0.7),
        Mat4::from_rotation_translation(
            Quat::from_axis_angle(Vec3::unit_z(), -90f32.to_radians()),
            1.2 * Vec3::unit_x(),
        ),
    );

    // ---- Pedestal & columns ----
    let material = materials.add(Color::rgb(0.7, 0.7, 0.7).into());
    let transforms: &[(Vec3, Vec3); 4] = &[
        (Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.340, 1.200, 0.340)),
        (Vec3::new(1.0, 0.05, -1.0), Vec3::new(0.125, 2.0, 0.125)),
        (Vec3::new(1.0, -1.0, 0.05), Vec3::new(0.125, 0.125, 2.0)),
        (Vec3::new(-0.05, -1.0, -1.0), Vec3::new(2.0, 0.125, 0.125)),
    ];

    for t in transforms {
        commands.spawn(PbrBundle {
            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                t.1,
                Quat::identity(),
                t.0,
            )),
            material: material.clone(),
            mesh: cube.clone(),
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
                mesh: cube.clone(),
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
        .add_plugin(GridsPlugin)
        .add_plugin(LightRingsPlugin)
        .add_asset::<EmissiveMaterial>()
        .add_startup_system(setup.system())
        .run();
}
