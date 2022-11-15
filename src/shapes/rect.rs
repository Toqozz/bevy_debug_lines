use bevy::prelude::*;

use super::{Shape, ShapeHandle, ToMeshAttributes};

pub struct Rect {
    pub position: Vec3,
    pub extent: Vec2,
    pub rotation: Quat,
    pub color: Color,
    pub duration: f32,
}

impl Rect {
    pub(crate) fn new(position: Vec3, size: Vec2) -> Self {
        Self {
            position,
            extent: size * 0.5,
            ..Default::default()
        }
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            extent: Vec2::ZERO,
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

impl ToMeshAttributes for Rect {
    fn positions(&self) -> Vec<[f32; 3]> {
        // verts in local space
        let v1 = Vec3::new(-self.extent.x, -self.extent.y, 0.0);
        let v2 = Vec3::new(self.extent.x, -self.extent.y, 0.0);
        let v3 = Vec3::new(self.extent.x, self.extent.y, 0.0);
        let v4 = Vec3::new(-self.extent.x, self.extent.y, 0.0);

        // verts in global space
        let v1 = (self.position + self.rotation.mul_vec3(v1)).into();
        let v2 = (self.position + self.rotation.mul_vec3(v2)).into();
        let v3 = (self.position + self.rotation.mul_vec3(v3)).into();
        let v4 = (self.position + self.rotation.mul_vec3(v4)).into();

        vec![v1, v2, v2, v3, v3, v4, v4, v1]
    }

    fn colors(&self) -> Vec<[f32; 4]> {
        vec![self.color.as_linear_rgba_f32(); 8]
    }

    fn duration(&self) -> f32 {
        self.duration
    }

    fn update(&mut self, dt: f32) {
        self.duration -= dt
    }
}

impl<'a> ShapeHandle<'a, Rect> {
    pub fn rotation(self, rotation: Quat) -> Self {
        if let Shape::Rect(rect) = &mut self.debug_lines.shapes[self.index] {
            rect.rotation = rotation;
        }
        self
    }

    pub fn angle(self, angle: f32) -> Self {
        if let Shape::Rect(rect) = &mut self.debug_lines.shapes[self.index] {
            rect.rotation = Quat::from_axis_angle(Vec3::Z, angle);
        }
        self
    }

    pub fn color(self, color: Color) -> Self {
        if let Shape::Rect(rect) = &mut self.debug_lines.shapes[self.index] {
            rect.color = color;
        }
        self
    }

    pub fn duration(self, duration: f32) -> Self {
        if let Shape::Rect(rect) = &mut self.debug_lines.shapes[self.index] {
            rect.duration = duration;
        }
        self
    }
}
