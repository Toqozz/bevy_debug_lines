use bevy::prelude::*;

use super::{AddLines, Shape, ShapeHandle};

pub struct Circle {
    pub position: Vec3,
    pub radius: f32,
    pub segments: u32,
    pub rotation: Quat,
    pub color: Color,
    pub duration: f32,
}

impl Circle {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            radius: 1.0,
            segments: 16,
            rotation: Quat::IDENTITY,
            color: Color::WHITE,
            duration: 0.0,
        }
    }
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Shape::Circle(circle)
    }
}

impl AddLines for Circle {
    fn add_lines(&self, lines: &mut crate::DebugLines) {
        let step_size = std::f32::consts::TAU / self.segments as f32;
        for i in 1..=self.segments {
            let start_angle = step_size * (i - 1) as f32;
            let end_angle = step_size * i as f32;
            let start = self.position
                + self
                    .rotation
                    .mul_vec3(Vec3::new(start_angle.cos(), start_angle.sin(), 0.0) * self.radius);
            let end = self.position
                + self
                    .rotation
                    .mul_vec3(Vec3::new(end_angle.cos(), end_angle.sin(), 0.0) * self.radius);

            lines.line_colored(start, end, self.duration, self.color);
        }
    }
}

impl<'a> ShapeHandle<'a, Circle> {
    pub fn position(self, position: Vec3) -> Self {
        if let Shape::Circle(circle) = &mut self.shapes.shapes[self.index] {
            circle.position = position;
        }
        self
    }

    pub fn radius(self, radius: f32) -> Self {
        if let Shape::Circle(circle) = &mut self.shapes.shapes[self.index] {
            circle.radius = radius;
        }
        self
    }

    pub fn segments(self, segments: u32) -> Self {
        if let Shape::Circle(circle) = &mut self.shapes.shapes[self.index] {
            circle.segments = segments;
        }
        self
    }

    pub fn rotation(self, rotation: Quat) -> Self {
        if let Shape::Circle(circle) = &mut self.shapes.shapes[self.index] {
            circle.rotation = rotation;
        }
        self
    }

    pub fn color(self, color: Color) -> Self {
        if let Shape::Circle(circle) = &mut self.shapes.shapes[self.index] {
            circle.color = color;
        }
        self
    }

    pub fn duration(self, duration: f32) -> Self {
        if let Shape::Circle(circle) = &mut self.shapes.shapes[self.index] {
            circle.duration = duration;
        }
        self
    }
}
