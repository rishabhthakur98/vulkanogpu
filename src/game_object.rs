use crate::mesh::MeshData;
use crate::transform::Transform;
use crate::material::Material;

pub struct GameObject {
    pub mesh: MeshData,
    pub transform: Transform,
    pub material: Material,
}

impl GameObject {
    pub fn new(mesh: MeshData) -> Self {
        Self {
            mesh,
            transform: Transform::default(),
            material: Material::default(),
        }
    }
}