use std::marker::PhantomData;

use bevy::prelude::*;

use crate::DebugLines;

pub use self::cuboid::Cuboid;
pub use self::line::Line;
pub use self::rect::Rect;

mod cuboid;
mod line;
mod rect;

/// Bevy resource providing facilities to draw shapes.
///
/// # Usage
/// ```
/// use bevy::prelude::*;
/// use bevy_prototype_debug_lines::*;
///
/// // Draws a red cuboid (box) rotating around X.
/// fn some_system(time: Res<Time>, mut shapes: ResMut<DebugShapes>) {
///     let seconds = time.elapsed_seconds();
///
///     shapes
///         .cuboid(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0))
///         .rotation(Quat::from_axis_angle(
///             Vec3::X,
///             seconds * std::f32::consts::FRAC_PI_4,
///         ))
///         .color(Color::RED);
/// }
/// ```
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

    /// Adds a [`Cuboid`] shape.
    ///
    /// See [`ShapeHandle`] impl on [`Cuboid`] for more shape properties.
    ///
    /// Short for [`DebugShapes::add`].
    ///
    /// # Arguments
    ///
    /// * `position` - Center position
    /// * `size` - Side lengths
    pub fn cuboid(&mut self, position: Vec3, size: Vec3) -> ShapeHandle<'_, Cuboid> {
        self.add(Cuboid::new(position, size))
    }

    /// Adds a [`Line`] shape.
    ///
    /// See [`ShapeHandle`] impl on [`Line`] for more shape properties.
    ///
    /// Short for [`DebugShapes::add`].
    ///
    /// # Arguments
    ///
    /// * `start` - Start position
    /// * `end` - End position
    pub fn line(&mut self, start: Vec3, end: Vec3) -> ShapeHandle<'_, Line> {
        self.add(Line::new(start, end))
    }

    /// Adds a [`Rect`] shape.
    ///
    /// See [`ShapeHandle`] impl on [`Rect`] for more shape properties.
    ///
    /// Short for [`DebugShapes::add`].
    ///
    /// # Arguments
    ///
    /// * `position` - Center position
    /// * `size` - Side lengths
    pub fn rect(&mut self, position: Vec3, size: Vec2) -> ShapeHandle<'_, Rect> {
        self.add(Rect::new(position, size))
    }
}

/// Implemented on shapes to add lines to [`DebugLines`].
pub(crate) trait AddLines {
    /// Add required lines to [`DebugLines`] for drawing shape.
    fn add_lines(&self, lines: &mut DebugLines);
}

/// Wrapper around all shape types to allow matching to specific shapes.
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

/// Used to modify shapes after they've been added to [`DebugShapes`].
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
