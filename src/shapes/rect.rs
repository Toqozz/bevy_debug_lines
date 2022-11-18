use bevy::prelude::*;

use super::{AddLines, Shape, ShapeHandle};

pub struct Rect {
    pub position: Vec3,
    pub extent: Vec2,
    pub rotation: Quat,
    pub color: Color,
    pub duration: f32,
}

impl Rect {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            extent: Vec2::ONE,
            rotation: Quat::IDENTITY,
            color: Color::WHITE,
            duration: 0.0,
        }
    }
}

impl From<Rect> for Shape {
    fn from(rect: Rect) -> Self {
        Shape::Rect(rect)
    }
}

impl AddLines for Rect {
    fn add_lines(&self, lines: &mut crate::DebugLines) {
        // verts in local space
        let v1 = Vec3::new(-self.extent.x, -self.extent.y, 0.0);
        let v2 = Vec3::new(self.extent.x, -self.extent.y, 0.0);
        let v3 = Vec3::new(self.extent.x, self.extent.y, 0.0);
        let v4 = Vec3::new(-self.extent.x, self.extent.y, 0.0);

        // verts in global space
        let v1 = self.position + self.rotation.mul_vec3(v1);
        let v2 = self.position + self.rotation.mul_vec3(v2);
        let v3 = self.position + self.rotation.mul_vec3(v3);
        let v4 = self.position + self.rotation.mul_vec3(v4);

        lines.line_colored(v1, v2, self.duration, self.color);
        lines.line_colored(v2, v3, self.duration, self.color);
        lines.line_colored(v3, v4, self.duration, self.color);
        lines.line_colored(v4, v1, self.duration, self.color);
    }
}

impl<'a> ShapeHandle<'a, Rect> {
    pub fn position(self, position: Vec3) -> Self {
        if let Shape::Rect(rect) = &mut self.shapes.shapes[self.index] {
            rect.position = position;
        }
        self
    }

    pub fn size(self, size: Vec2) -> Self {
        if let Shape::Rect(rect) = &mut self.shapes.shapes[self.index] {
            rect.extent = size * 0.5;
        }
        self
    }

    pub fn min_max(self, min: Vec2, max: Vec2) -> Self {
        if let Shape::Rect(rect) = &mut self.shapes.shapes[self.index] {
            rect.position = ((min + max) * 0.5).extend(rect.position.z);
            rect.extent = (max - min) * 0.5;
        }
        self
    }

    pub fn rotation(self, rotation: Quat) -> Self {
        if let Shape::Rect(rect) = &mut self.shapes.shapes[self.index] {
            rect.rotation = rotation;
        }
        self
    }

    pub fn angle(self, angle: f32) -> Self {
        self.rotation(Quat::from_rotation_z(angle))
    }

    pub fn color(self, color: Color) -> Self {
        if let Shape::Rect(rect) = &mut self.shapes.shapes[self.index] {
            rect.color = color;
        }
        self
    }

    pub fn duration(self, duration: f32) -> Self {
        if let Shape::Rect(rect) = &mut self.shapes.shapes[self.index] {
            rect.duration = duration;
        }
        self
    }
}
