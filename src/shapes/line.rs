use bevy::prelude::*;

use super::{AddLines, Shape, ShapeHandle};

pub struct Line {
    pub start: Vec3,
    pub end: Vec3,
    pub start_color: Color,
    pub end_color: Color,
    pub duration: f32,
}

impl Line {
    pub fn new(start: Vec3, end: Vec3) -> Self {
        Self {
            start,
            end,
            ..Default::default()
        }
    }
}

impl Default for Line {
    fn default() -> Self {
        Self {
            start: Vec3::ZERO,
            end: Vec3::ZERO,
            start_color: Color::WHITE,
            end_color: Color::WHITE,
            duration: 0.0,
        }
    }
}

impl From<Line> for Shape {
    fn from(line: Line) -> Self {
        Shape::Line(line)
    }
}

impl AddLines for Line {
    fn add_lines(&self, lines: &mut crate::DebugLines) {
        lines.line_gradient(
            self.start,
            self.end,
            self.duration,
            self.start_color,
            self.end_color,
        )
    }
}

impl<'a> ShapeHandle<'a, Line> {
    pub fn color(self, color: Color) -> Self {
        self.gradient(color, color)
    }

    pub fn gradient(self, start_color: Color, end_color: Color) -> Self {
        if let Shape::Line(line) = &mut self.shapes.shapes[self.index] {
            line.start_color = start_color;
            line.end_color = end_color;
        }
        self
    }

    pub fn duration(self, duration: f32) -> Self {
        if let Shape::Line(line) = &mut self.shapes.shapes[self.index] {
            line.duration = duration;
        }
        self
    }
}
