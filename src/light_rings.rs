use crate::EmissiveMaterial;
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

struct LightRing;
struct LightRingVoxel;

/// Spawns a field of randomly colored and positioned lights that form a
/// tube/ring shape and spins in place.
pub fn spawn_voxel_light_ring(
    commands: &mut Commands,
    emissive_materials: &mut ResMut<Assets<EmissiveMaterial>>,
    pipeline_handle: &Handle<PipelineDescriptor>,
    cube: &Handle<Mesh>,
    lights_count: u32,
    height: f32,
    inner_radius: f32,
    outer_radius: f32,
    min_color: Color,
    max_color: Color,
    transform: Mat4,
) {
    let voxel_scale = Vec3::splat(0.025);
    let mut rng = rand::thread_rng();
    let color_randomizer = Uniform::from(0f32..=1f32);
    let radius_randomizer = Uniform::from(inner_radius..=outer_radius);
    let height_randomizer = Uniform::from((-0.5 * height)..=(0.5 * height));
    let x_randomizer = Uniform::from(-1f32..=1f32);
    let z_randomizer = Uniform::from(-1f32..=1f32);

    commands
        .spawn(PbrBundle {
            transform: Transform::from_matrix(transform),
            ..Default::default()
        })
        .with(LightRing)
        .with_children(|parent| {
            for _i in 0..lights_count {
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
                        transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                            voxel_scale,
                            Quat::identity(),
                            translation,
                        )),
                        ..Default::default()
                    })
                    .with(emissive_materials.add(EmissiveMaterial {
                        color:
                            Color::from(
                                1.5
                                    * Vec4::from(min_color).lerp(
                                        Vec4::from(max_color),
                                        color_randomizer.sample(&mut rng),
                                    ),
                            ),
                    }))
                    .with(LightRingVoxel);
            }
        });
}

/// Animate all light ring voxel entities.
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
        app.add_system(animate_light_ring.system())
            .add_system(animate_light_ring_voxels.system());
    }
}
