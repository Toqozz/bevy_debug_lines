use bevy::prelude::*;

use super::{AddLines, Circle, Shape, ShapeHandle};

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub segments: u32,
    pub rotation: Quat,
    pub color: Color,
    pub duration: f32,
}

impl Sphere {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Sphere {
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

impl From<Sphere> for Shape {
    fn from(sphere: Sphere) -> Self {
        Shape::Sphere(sphere)
    }
}

impl AddLines for Sphere {
    fn add_lines(&self, lines: &mut crate::DebugLines) {
        use std::f32::consts::FRAC_PI_2;
        Circle {
            position: self.position,
            radius: self.radius,
            segments: self.segments,
            rotation: self.rotation,
            color: self.color,
            duration: self.duration,
        }
        .add_lines(lines);
        Circle {
            position: self.position,
            radius: self.radius,
            segments: self.segments,
            rotation: self.rotation.mul_quat(Quat::from_rotation_x(FRAC_PI_2)),
            color: self.color,
            duration: self.duration,
        }
        .add_lines(lines);
        Circle {
            position: self.position,
            radius: self.radius,
            segments: self.segments,
            rotation: self.rotation.mul_quat(Quat::from_rotation_y(FRAC_PI_2)),
            color: self.color,
            duration: self.duration,
        }
        .add_lines(lines);
    }
}

impl<'a> ShapeHandle<'a, Sphere> {
    pub fn position(self, position: Vec3) -> Self {
        if let Shape::Sphere(sphere) = &mut self.shapes.shapes[self.index] {
            sphere.position = position;
        }
        self
    }

    pub fn radius(self, radius: f32) -> Self {
        if let Shape::Sphere(sphere) = &mut self.shapes.shapes[self.index] {
            sphere.radius = radius;
        }
        self
    }

    pub fn segments(self, segments: u32) -> Self {
        if let Shape::Sphere(sphere) = &mut self.shapes.shapes[self.index] {
            sphere.segments = segments;
        }
        self
    }

    pub fn rotation(self, rotation: Quat) -> Self {
        if let Shape::Sphere(sphere) = &mut self.shapes.shapes[self.index] {
            sphere.rotation = rotation;
        }
        self
    }

    pub fn color(self, color: Color) -> Self {
        if let Shape::Sphere(sphere) = &mut self.shapes.shapes[self.index] {
            sphere.color = color;
        }
        self
    }

    pub fn duration(self, duration: f32) -> Self {
        if let Shape::Sphere(sphere) = &mut self.shapes.shapes[self.index] {
            sphere.duration = duration;
        }
        self
    }
}
