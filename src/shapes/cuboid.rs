use bevy::prelude::*;

use super::{AddLines, Shape, ShapeHandle};

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

impl AddLines for Cuboid {
    fn add_lines(&self, lines: &mut crate::DebugLines) {
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
        let v1 = self.position + self.rotation.mul_vec3(v1);
        let v2 = self.position + self.rotation.mul_vec3(v2);
        let v3 = self.position + self.rotation.mul_vec3(v3);
        let v4 = self.position + self.rotation.mul_vec3(v4);
        let v5 = self.position + self.rotation.mul_vec3(v5);
        let v6 = self.position + self.rotation.mul_vec3(v6);
        let v7 = self.position + self.rotation.mul_vec3(v7);
        let v8 = self.position + self.rotation.mul_vec3(v8);

        lines.line_colored(v1, v2, self.duration, self.color);
        lines.line_colored(v2, v3, self.duration, self.color);
        lines.line_colored(v3, v4, self.duration, self.color);
        lines.line_colored(v4, v1, self.duration, self.color);
        lines.line_colored(v5, v6, self.duration, self.color);
        lines.line_colored(v6, v7, self.duration, self.color);
        lines.line_colored(v7, v8, self.duration, self.color);
        lines.line_colored(v8, v5, self.duration, self.color);
        lines.line_colored(v1, v5, self.duration, self.color);
        lines.line_colored(v2, v6, self.duration, self.color);
        lines.line_colored(v3, v7, self.duration, self.color);
        lines.line_colored(v4, v8, self.duration, self.color);
    }
}

impl<'a> ShapeHandle<'a, Cuboid> {
    pub fn rotation(self, rotation: Quat) -> Self {
        if let Shape::Cuboid(cuboid) = &mut self.shapes.shapes[self.index] {
            cuboid.rotation = rotation;
        }
        self
    }

    pub fn color(self, color: Color) -> Self {
        if let Shape::Cuboid(cuboid) = &mut self.shapes.shapes[self.index] {
            cuboid.color = color;
        }
        self
    }

    pub fn duration(self, duration: f32) -> Self {
        if let Shape::Cuboid(cuboid) = &mut self.shapes.shapes[self.index] {
            cuboid.duration = duration;
        }
        self
    }
}
