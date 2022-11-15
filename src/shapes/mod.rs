use std::marker::PhantomData;

use crate::DebugLines;

pub use self::cuboid::Cuboid;
pub use self::line::Line;
pub use self::rect::Rect;

mod cuboid;
mod line;
mod rect;

pub(crate) trait ToMeshAttributes {
    fn positions(&self) -> Vec<[f32; 3]>;
    fn colors(&self) -> Vec<[f32; 4]>;
    fn duration(&self) -> f32;
    fn update(&mut self, dt: f32);
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

    fn update(&mut self, dt: f32) {
        match self {
            Shape::Cuboid(s) => s.update(dt),
            Shape::Line(s) => s.update(dt),
            Shape::Rect(s) => s.update(dt),
        }
    }
}

pub struct ShapeHandle<'a, S> {
    pub(crate) debug_lines: &'a mut DebugLines,
    pub(crate) index: usize,
    _ty: PhantomData<S>,
}

impl<'a, S> ShapeHandle<'a, S> {
    pub(crate) fn new(debug_lines: &'a mut DebugLines, index: usize) -> Self {
        Self {
            debug_lines,
            index,
            _ty: PhantomData,
        }
    }
}
