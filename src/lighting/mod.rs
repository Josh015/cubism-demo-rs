use bevy::{
    prelude::*,
    render::{
        pipeline::PipelineDescriptor,
        render_graph::{base, AssetRenderResourcesNode, RenderGraph, RenderResourcesNode},
        shader,
    },
};

mod custom_pipeline;
pub use custom_pipeline::*;

mod entity;
pub use entity::*;

mod light;
pub use light::*;

mod light_node;
pub use light_node::*;

mod material;
pub use material::*;

/// the names of pbr graph nodes
pub mod node {
    // pub const TRANSFORM: &str = "transform";
    pub const CUSTOM_MATERIAL: &str = "custom_material";
    pub const CUSTOM_LIGHTS: &str = "custom_lights";
}

#[derive(Default)]
pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<CustomMaterial>()
            .register_type::<CustomPointLight>()
            .add_system_to_stage(
                stage::POST_UPDATE,
                shader::asset_shader_defs_system::<CustomMaterial>.system(),
            );
        // .init_resource::<AmbientLight>();

        let resources = app.resources();
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();
        let mut shaders = resources.get_mut::<Assets<Shader>>().unwrap();
        let mut pipelines = resources.get_mut::<Assets<PipelineDescriptor>>().unwrap();

        // render_graph.add_system_node(
        //     node::TRANSFORM,
        //     RenderResourcesNode::<GlobalTransform>::new(true),
        // );

        // Add an AssetRenderResourcesNode to our Render Graph. This will bind CustomMaterial resources to our shader
        render_graph.add_system_node(
            node::CUSTOM_MATERIAL,
            AssetRenderResourcesNode::<CustomMaterial>::new(true),
        );

        render_graph.add_system_node(node::CUSTOM_LIGHTS, LightsNode::new(10));

        // Create a new shader pipeline
        pipelines.set_untracked(CUSTOM_PIPELINE_HANDLE, build_custom_pipeline(&mut shaders));

        // Add a Render Graph edge connecting our new "my_material" node to the main pass node. This ensures "my_material" runs before the main pass
        render_graph
            .add_node_edge(node::CUSTOM_MATERIAL, base::node::MAIN_PASS)
            .unwrap();

        // render_graph
        //     .add_node_edge(node::TRANSFORM, base::node::MAIN_PASS)
        //     .unwrap();
        render_graph
            .add_node_edge(node::CUSTOM_LIGHTS, base::node::MAIN_PASS)
            .unwrap();

        // // add default StandardMaterial
        // let mut materials = app
        //     .resources()
        //     .get_mut::<Assets<StandardMaterial>>()
        //     .unwrap();
        // materials.set_untracked(
        //     Handle::<StandardMaterial>::default(),
        //     StandardMaterial {
        //         albedo: Color::PINK,
        //         shaded: false,
        //         albedo_texture: None,
        //     },
        // );
    }
}
