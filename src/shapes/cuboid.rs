use bevy::prelude::*;

use super::{Shape, ShapeHandle, ToMeshAttributes};

pub struct Cuboid {
    pub position: Vec3,
    pub extent: Vec3,
    pub rotation: Quat,
    pub color: Color,
    pub duration: f32,
}

impl Cuboid {
    pub fn new(position: Vec3, size: Vec3) -> Self {
        Self {
            position,
            extent: size * 0.5,
            ..Default::default()
        }
    }
}

impl Default for Cuboid {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            extent: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            color: Color::WHITE,
            duration: 0.0,
        }
    }
}

impl From<Cuboid> for Shape {
    fn from(cuboid: Cuboid) -> Self {
        Shape::Cuboid(cuboid)
    }
}

impl ToMeshAttributes for Cuboid {
    fn positions(&self) -> Vec<[f32; 3]> {
        // verts in local space
        let v1 = Vec3::new(-self.extent.x, -self.extent.y, -self.extent.z);
        let v2 = Vec3::new(self.extent.x, -self.extent.y, -self.extent.z);
        let v3 = Vec3::new(self.extent.x, self.extent.y, -self.extent.z);
        let v4 = Vec3::new(-self.extent.x, self.extent.y, -self.extent.z);
        let v5 = Vec3::new(-self.extent.x, -self.extent.y, self.extent.z);
        let v6 = Vec3::new(self.extent.x, -self.extent.y, self.extent.z);
        let v7 = Vec3::new(self.extent.x, self.extent.y, self.extent.z);
        let v8 = Vec3::new(-self.extent.x, self.extent.y, self.extent.z);

        // verts in global space
        let v1 = (self.position + self.rotation.mul_vec3(v1)).into();
        let v2 = (self.position + self.rotation.mul_vec3(v2)).into();
        let v3 = (self.position + self.rotation.mul_vec3(v3)).into();
        let v4 = (self.position + self.rotation.mul_vec3(v4)).into();
        let v5 = (self.position + self.rotation.mul_vec3(v5)).into();
        let v6 = (self.position + self.rotation.mul_vec3(v6)).into();
        let v7 = (self.position + self.rotation.mul_vec3(v7)).into();
        let v8 = (self.position + self.rotation.mul_vec3(v8)).into();

        vec![
            v1, v2, v2, v3, v3, v4, v4, v1, v5, v6, v6, v7, v7, v8, v8, v5, v1, v5, v2, v6, v3, v7,
            v4, v8,
        ]
    }

    fn colors(&self) -> Vec<[f32; 4]> {
        vec![self.color.as_linear_rgba_f32(); 24]
    }

    fn duration(&self) -> f32 {
        self.duration
    }

    fn update(&mut self, dt: f32) {
        self.duration -= dt
    }
}

impl<'a> ShapeHandle<'a, Cuboid> {
    pub fn rotation(self, rotation: Quat) -> Self {
        if let Shape::Cuboid(cuboid) = &mut self.debug_lines.shapes[self.index] {
            cuboid.rotation = rotation;
        }
        self
    }

    pub fn color(self, color: Color) -> Self {
        if let Shape::Cuboid(cuboid) = &mut self.debug_lines.shapes[self.index] {
            cuboid.color = color;
        }
        self
    }

    pub fn duration(self, duration: f32) -> Self {
        if let Shape::Cuboid(cuboid) = &mut self.debug_lines.shapes[self.index] {
            cuboid.duration = duration;
        }
        self
    }
}
