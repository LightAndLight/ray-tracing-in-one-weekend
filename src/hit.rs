use crate::{material::Material, texture, vec3::Vec3};

pub enum Face {
    Front,
    Back,
}

pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub face: Face,
    pub material: Material,
    pub texture_coord: texture::Coord,
}
