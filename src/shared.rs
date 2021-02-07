use bevy::prelude::*;

pub struct SharedData {
    pub unit_cube: Handle<Mesh>,
}

impl FromResources for SharedData {
    fn from_resources(resources: &Resources) -> Self {
        let mut meshes = resources.get_mut::<Assets<Mesh>>().unwrap();

        SharedData {
            unit_cube: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        }
    }
}
