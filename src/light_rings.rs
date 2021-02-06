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
use rand::distributions::{Distribution, Uniform};

const RING_ROTATION_SPEED: f32 = 1.0;

struct LightRingDesc {
    lights_count: u32,
    height: f32,
    inner_radius: f32,
    outer_radius: f32,
    min_color: Color,
    max_color: Color,
    transform: Mat4,
}

struct LightRing;
struct LightRingVoxel;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127e"]
pub struct LightRingMaterial {
    pub color: Color,
}

fn spawn_voxel_light_rings(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LightRingMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Load cube mesh
    let cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    // Create a new shader pipeline
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("./shaders/light_ring_material.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("./shaders/light_ring_material.frag"),
        ))),
    }));

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind LightRingMaterial resources to our shader
    render_graph.add_system_node(
        "light_ring_material",
        AssetRenderResourcesNode::<LightRingMaterial>::new(true),
    );

    // Add a Render Graph edge connecting our new "light_ring_material" node to the main pass node. This ensures "light_ring_material" runs before the main pass
    render_graph
        .add_node_edge("light_ring_material", base::node::MAIN_PASS)
        .unwrap();

    // Voxel light ring descriptions
    let descriptions: [LightRingDesc; 4] = [
        // Green-yellow light ring
        LightRingDesc {
            lights_count: 200,
            height: 0.5,
            inner_radius: 0.4,
            outer_radius: 0.7,
            min_color: Color::rgb(0.3, 0.3, 0.05),
            max_color: Color::rgb(0.6, 0.7, 0.1),
            transform: Mat4::from_translation(-0.55 * Vec3::unit_y()),
        },
        // Cyan light ring
        LightRingDesc {
            lights_count: 100,
            height: 0.125,
            inner_radius: 0.4,
            outer_radius: 1.0,
            min_color: Color::rgb(0.05, 0.4, 0.5),
            max_color: Color::rgb(0.1, 0.5, 0.7),
            transform: Mat4::from_translation(-1.2 * Vec3::unit_y()),
        },
        // Orange light ring
        LightRingDesc {
            lights_count: 100,
            height: 0.125,
            inner_radius: 0.25,
            outer_radius: 1.0,
            min_color: Color::rgb(0.5, 0.4, 0.05),
            max_color: Color::rgb(0.6, 0.5, 0.1),
            transform: Mat4::from_rotation_translation(
                Quat::from_axis_angle(Vec3::unit_x(), 90f32.to_radians()),
                -1.2 * Vec3::unit_z(),
            ),
        },
        // Magenta light ring
        LightRingDesc {
            lights_count: 100,
            height: 0.125,
            inner_radius: 0.25,
            outer_radius: 1.0,
            min_color: Color::rgb(0.1, 0.1, 0.5),
            max_color: Color::rgb(0.6, 0.2, 0.7),
            transform: Mat4::from_rotation_translation(
                Quat::from_axis_angle(Vec3::unit_z(), -90f32.to_radians()),
                1.2 * Vec3::unit_x(),
            ),
        },
    ];

    // Spawn voxel light rings
    for lr in descriptions.iter() {
        let voxel_scale = Vec3::splat(0.025);
        let mut rng = rand::thread_rng();
        let color_randomizer = Uniform::from(0f32..=1f32);
        let radius_randomizer = Uniform::from(lr.inner_radius..=lr.outer_radius);
        let height_randomizer = Uniform::from((-0.5 * lr.height)..=(0.5 * lr.height));
        let x_randomizer = Uniform::from(-1f32..=1f32);
        let z_randomizer = Uniform::from(-1f32..=1f32);

        commands
            .spawn(PbrBundle {
                transform: Transform::from_matrix(lr.transform),
                ..Default::default()
            })
            .with(LightRing)
            .with_children(|parent| {
                for _i in 0..lr.lights_count {
                    let mut translation = Vec3::new(
                        x_randomizer.sample(&mut rng),
                        0.0,
                        z_randomizer.sample(&mut rng),
                    );

                    translation = translation.normalize() * radius_randomizer.sample(&mut rng);
                    translation.y = height_randomizer.sample(&mut rng);

                    parent
                        .spawn(MeshBundle {
                            mesh: cube.clone(),
                            render_pipelines: RenderPipelines::from_pipelines(vec![
                                RenderPipeline::new(pipeline_handle.clone()),
                            ]),
                            transform: Transform::from_matrix(
                                Mat4::from_scale_rotation_translation(
                                    voxel_scale,
                                    Quat::identity(),
                                    translation,
                                ),
                            ),
                            ..Default::default()
                        })
                        .with(materials.add(LightRingMaterial {
                            color: Color::from(
                                1.5 * Vec4::from(lr.min_color).lerp(
                                    Vec4::from(lr.max_color),
                                    color_randomizer.sample(&mut rng),
                                ),
                            ),
                        }))
                        .with(LightRingVoxel);
                }
            });
    }
}

fn animate_light_ring(time: Res<Time>, mut query: Query<(&mut Transform, &LightRing)>) {
    for (mut transform, _) in query.iter_mut() {
        transform.rotate(Quat::from_axis_angle(
            Vec3::unit_y(),
            RING_ROTATION_SPEED * time.delta_seconds(),
        ));
    }
}

fn animate_light_ring_voxels(time: Res<Time>, mut query: Query<(&mut Transform, &LightRingVoxel)>) {
    for (mut transform, _) in query.iter_mut() {
        transform.rotate(Quat::from_axis_angle(
            Vec3::unit_y(),
            RING_ROTATION_SPEED * -time.delta_seconds(),
        ));
    }
}

pub struct LightRingsPlugin;
impl Plugin for LightRingsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<LightRingMaterial>()
            .add_startup_system(spawn_voxel_light_rings.system())
            .add_system(animate_light_ring.system())
            .add_system(animate_light_ring_voxels.system());
    }
}
