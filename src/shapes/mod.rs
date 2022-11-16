use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{DebugLines, MAX_POINTS};

pub use self::cuboid::Cuboid;
pub use self::line::Line;
pub use self::rect::Rect;

mod cuboid;
mod line;
mod rect;

#[derive(Resource, Default)]
pub struct DebugShapes {
    pub shapes: Vec<Shape>,
}

impl DebugShapes {
    /// Add a generic shape to be drawn and return a handle to it.
    pub fn add<S>(&mut self, shape: S) -> ShapeHandle<'_, S>
    where
        S: Into<Shape>,
    {
        let index = self.shapes.len();
        self.shapes.push(shape.into());
        ShapeHandle::new(self, index)
    }

    /// Short for [`DebugLines::add`].
    pub fn cuboid(&mut self, position: Vec3, size: Vec3) -> ShapeHandle<'_, Cuboid> {
        self.add(Cuboid::new(position, size))
    }

    /// Short for [`DebugLines::add`].
    pub fn line(&mut self, start: Vec3, end: Vec3) -> ShapeHandle<'_, Line> {
        self.add(Line::new(start, end))
    }

    /// Short for [`DebugLines::add`].
    pub fn rect(&mut self, position: Vec3, size: Vec2) -> ShapeHandle<'_, Rect> {
        self.add(Rect::new(position, size))
    }
}

pub(crate) fn update(mut lines: ResMut<DebugLines>, mut shapes: ResMut<DebugShapes>) {
    if shapes.shapes.len() > 0 && lines.positions.len() >= MAX_POINTS {
        warn!("Tried to add a new line when existing number of lines was already at maximum, ignoring.");
    } else {
        lines
            .positions
            .extend(shapes.shapes.iter().flat_map(|shape| shape.positions()));
        lines
            .colors
            .extend(shapes.shapes.iter().flat_map(|shape| shape.colors()));
        lines
            .durations
            .extend(shapes.shapes.iter().map(|shape| shape.duration()));
    }
    shapes.shapes.clear();
}

pub(crate) trait ToMeshAttributes {
    fn positions(&self) -> Vec<[f32; 3]>;
    fn colors(&self) -> Vec<[f32; 4]>;
    fn duration(&self) -> f32;
}

pub enum Shape {
    Cuboid(Cuboid),
    Line(Line),
    Rect(Rect),
}

impl ToMeshAttributes for Shape {
    fn positions(&self) -> Vec<[f32; 3]> {
        match self {
            Shape::Cuboid(s) => s.positions(),
            Shape::Line(s) => s.positions(),
            Shape::Rect(s) => s.positions(),
        }
    }

    fn colors(&self) -> Vec<[f32; 4]> {
        match self {
            Shape::Cuboid(s) => s.colors(),
            Shape::Line(s) => s.colors(),
            Shape::Rect(s) => s.colors(),
        }
    }

    fn duration(&self) -> f32 {
        match self {
            Shape::Cuboid(s) => s.duration(),
            Shape::Line(s) => s.duration(),
            Shape::Rect(s) => s.duration(),
        }
    }
}

pub struct ShapeHandle<'a, S> {
    pub(crate) shapes: &'a mut DebugShapes,
    pub(crate) index: usize,
    _ty: PhantomData<S>,
}

impl<'a, S> ShapeHandle<'a, S> {
    pub(crate) fn new(shapes: &'a mut DebugShapes, index: usize) -> Self {
        Self {
            shapes,
            index,
            _ty: PhantomData,
        }
    }
}
