use std::marker::PhantomData;

use bevy::prelude::*;

use crate::DebugLines;

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

    /// Short for [`DebugShapes::add`].
    pub fn cuboid(&mut self, position: Vec3, size: Vec3) -> ShapeHandle<'_, Cuboid> {
        self.add(Cuboid::new(position, size))
    }

    /// Short for [`DebugShapes::add`].
    pub fn line(&mut self, start: Vec3, end: Vec3) -> ShapeHandle<'_, Line> {
        self.add(Line::new(start, end))
    }

    /// Short for [`DebugShapes::add`].
    pub fn rect(&mut self, position: Vec3, size: Vec2) -> ShapeHandle<'_, Rect> {
        self.add(Rect::new(position, size))
    }
}

pub(crate) fn update(mut lines: ResMut<DebugLines>, mut shapes: ResMut<DebugShapes>) {
    for shape in &shapes.shapes {
        shape.add_lines(&mut lines);
    }
    shapes.shapes.clear();
}

pub(crate) trait AddLines {
    fn add_lines(&self, lines: &mut DebugLines);
}

pub enum Shape {
    Cuboid(Cuboid),
    Line(Line),
    Rect(Rect),
}

impl AddLines for Shape {
    fn add_lines(&self, lines: &mut DebugLines) {
        match self {
            Shape::Cuboid(s) => s.add_lines(lines),
            Shape::Line(s) => s.add_lines(lines),
            Shape::Rect(s) => s.add_lines(lines),
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
