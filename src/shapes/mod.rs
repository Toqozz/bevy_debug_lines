use std::marker::PhantomData;

use crate::DebugLines;

pub use self::line::Line;

mod line;

pub(crate) trait ToMeshAttributes {
    fn positions(&self) -> Vec<[f32; 3]>;
    fn colors(&self) -> Vec<[f32; 4]>;
    fn duration(&self) -> f32;
    fn update(&mut self, dt: f32);
}

pub enum Shape {
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
