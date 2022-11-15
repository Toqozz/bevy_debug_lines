use std::marker::PhantomData;

use bevy::prelude::*;

use crate::DebugLines;

pub(crate) trait ToMeshAttributes {
    fn positions(&self) -> Vec<[f32; 3]>;
    fn colors(&self) -> Vec<[f32; 4]>;
    fn duration(&self) -> f32;
    fn update(&mut self, dt: f32);
}

pub(crate) enum Shape {
    Line(Line),
}

impl ToMeshAttributes for Shape {
    fn positions(&self) -> Vec<[f32; 3]> {
        match self {
            Shape::Line(line) => line.positions(),
        }
    }

    fn colors(&self) -> Vec<[f32; 4]> {
        match self {
            Shape::Line(line) => line.colors(),
        }
    }

    fn duration(&self) -> f32 {
        match self {
            Shape::Line(line) => line.duration(),
        }
    }

    fn update(&mut self, dt: f32) {
        match self {
            Shape::Line(line) => line.update(dt),
        }
    }
}

pub struct ShapeHandle<'a, T> {
    pub(crate) debug_lines: &'a mut DebugLines,
    pub(crate) index: usize,
    pub(crate) _ty: PhantomData<T>,
}

pub struct Line {
    pub start: Vec3,
    pub end: Vec3,
    pub start_color: Color,
    pub end_color: Color,
    pub duration: f32,
}

impl Line {
    pub(crate) fn new(start: Vec3, end: Vec3) -> Self {
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

impl ToMeshAttributes for Line {
    fn positions(&self) -> Vec<[f32; 3]> {
        vec![self.start.into(), self.end.into()]
    }

    fn colors(&self) -> Vec<[f32; 4]> {
        vec![
            self.start_color.as_linear_rgba_f32(),
            self.end_color.as_linear_rgba_f32(),
        ]
    }

    fn duration(&self) -> f32 {
        self.duration
    }

    fn update(&mut self, dt: f32) {
        self.duration -= dt
    }
}

impl<'a> ShapeHandle<'a, Line> {
    pub fn color(self, color: Color) -> Self {
        self.gradient(color, color)
    }

    pub fn gradient(self, start_color: Color, end_color: Color) -> Self {
        if let Shape::Line(line) = &mut self.debug_lines.shapes[self.index] {
            line.start_color = start_color;
            line.end_color = end_color;
        }
        self
    }

    pub fn duration(self, duration: f32) -> Self {
        if let Shape::Line(line) = &mut self.debug_lines.shapes[self.index] {
            line.duration = duration;
        }
        self
    }
}
