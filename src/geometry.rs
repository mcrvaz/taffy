//! Geometric primitives useful for layout

use crate::style::{Dimension, FlexDirection};
use core::ops::Add;

/// An axis-aligned UI rectangle
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Rect<T> {
    /// This can represent either the x-coordinate of the starting edge,
    /// or the amount of padding on the starting side.
    ///
    /// The starting edge is the left edge when working with LTR text,
    /// and the right edge when working with RTL text.
    pub start: T,
    /// This can represent either the x-coordinate of the ending edge,
    /// or the amount of padding on the ending side.
    ///
    /// The ending edge is the right edge when working with LTR text,
    /// and the left edge when working with RTL text.
    pub end: T,
    /// This can represent either the y-coordinate of the top edge,
    /// or the amount of padding on the top side.
    pub top: T,
    /// This can represent either the y-coordinate of the bottom edge,
    /// or the amount of padding on the bottom side.
    pub bottom: T,
}

impl<T> Rect<T> {
    /// Applies the function `f` to all four sides of the rect
    ///
    /// When applied to the left and right sides, the width is used
    /// as the second parameter of `f`.
    /// When applied to the top or bottom sides, the height is used instead.
    pub(crate) fn zip_size<R, F, U>(self, size: Size<U>, f: F) -> Rect<R>
    where
        F: Fn(T, U) -> R,
        U: Copy,
    {
        Rect {
            start: f(self.start, size.width),
            end: f(self.end, size.width),
            top: f(self.top, size.height),
            bottom: f(self.bottom, size.height),
        }
    }
}

impl<T> Rect<T>
where
    T: Add<Output = T> + Copy + Clone,
{
    /// The sum of [`Rect.start`](Rect) and [`Rect.end`](Rect)
    ///
    /// This is typically used when computing total padding.
    ///
    /// **NOTE:** this is *not* the width of the rectangle.
    pub(crate) fn horizontal_axis_sum(&self) -> T {
        self.start + self.end
    }

    /// The sum of [`Rect.top`](Rect) and [`Rect.bottom`](Rect)
    ///
    /// This is typically used when computing total padding.
    ///
    /// **NOTE:** this is *not* the height of the rectangle.
    pub(crate) fn vertical_axis_sum(&self) -> T {
        self.top + self.bottom
    }

    /// The sum of the two fields of the [`Rect`] representing the main axis.
    ///
    /// This is typically used when computing total padding.
    ///
    /// If the [`FlexDirection`] is [`FlexDirection::Row`] or [`FlexDirection::RowReverse`], this is [`Rect::horizontal`].
    /// Otherwise, this is [`Rect::vertical`].
    pub(crate) fn main_axis_sum(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.horizontal_axis_sum()
        } else {
            self.vertical_axis_sum()
        }
    }

    /// The sum of the two fields of the [`Rect`] representing the cross axis.
    ///
    /// If the [`FlexDirection`] is [`FlexDirection::Row`] or [`FlexDirection::RowReverse`], this is [`Rect::vertical`].
    /// Otherwise, this is [`Rect::horizontal`].
    pub(crate) fn cross_axis_sum(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.vertical_axis_sum()
        } else {
            self.horizontal_axis_sum()
        }
    }
}

impl<T> Rect<T>
where
    T: Copy + Clone,
{
    /// The `start` or `top` value of the [`Rect`], from the perspective of the main layout axis
    pub(crate) fn main_start(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.start
        } else {
            self.top
        }
    }

    /// The `end` or `bottom` value of the [`Rect`], from the perspective of the main layout axis
    pub(crate) fn main_end(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.end
        } else {
            self.bottom
        }
    }

    /// The `start` or `top` value of the [`Rect`], from the perspective of the cross layout axis
    pub(crate) fn cross_start(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.top
        } else {
            self.start
        }
    }

    /// The `end` or `bottom` value of the [`Rect`], from the perspective of the main layout axis
    pub(crate) fn cross_end(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.bottom
        } else {
            self.end
        }
    }
}

impl Rect<f32> {
    /// Creates a new Rect with `0.0` as all parameters
    pub const ZERO: Rect<f32> = Self { start: 0.0, end: 0.0, top: 0.0, bottom: 0.0 };

    /// Creates a new Rect
    #[must_use]
    pub fn new(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Self { start, end, top, bottom }
    }
}

/// The width and height of a [`Rect`]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Size<T> {
    /// The x extent of the rectangle
    pub width: T,
    /// The y extent of the rectangle
    pub height: T,
}

impl Size<()> {
    /// Generates a `Size<Option<f32>>` with undefined width and height
    #[must_use]
    pub fn undefined() -> Size<Option<f32>> {
        Size { width: None, height: None }
    }
}

impl<T> Size<T> {
    /// Applies the function `f` to both the width and height
    ///
    /// This is used to transform a `Rect<T>` into a `Rect<R>`.
    pub fn map<R, F>(self, f: F) -> Size<R>
    where
        F: Fn(T) -> R,
    {
        Size { width: f(self.width), height: f(self.height) }
    }

    /// Sets the extent of the main layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    pub(crate) fn set_main(&mut self, direction: FlexDirection, value: T) {
        if direction.is_row() {
            self.width = value
        } else {
            self.height = value
        }
    }

    /// Sets the extent of the cross layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    pub(crate) fn set_cross(&mut self, direction: FlexDirection, value: T) {
        if direction.is_row() {
            self.height = value
        } else {
            self.width = value
        }
    }

    /// Gets the extent of the main layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    pub(crate) fn main(self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.width
        } else {
            self.height
        }
    }

    /// Gets the extent of the cross layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    pub(crate) fn cross(self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.height
        } else {
            self.width
        }
    }
}

impl Size<f32> {
    /// A [`Size`] with zero width and height
    pub const ZERO: Size<f32> = Self { width: 0.0, height: 0.0 };
}

impl Size<Option<f32>> {
    /// A [`Size`] with `None` width and height
    pub const NONE: Size<Option<f32>> = Self { width: None, height: None };

    /// A [`Size<Option<f32>>`] with `Some(width)` and `Some(height)` as parameters
    #[must_use]
    pub fn new(width: f32, height: f32) -> Self {
        Size { width: Some(width), height: Some(height) }
    }
}

impl Size<Dimension> {
    /// Generates a [`Size<Dimension>`] using [`Dimension::Points`] values
    #[must_use]
    pub fn from_points(width: f32, height: f32) -> Self {
        Size { width: Dimension::Points(width), height: Dimension::Points(height) }
    }

    /// Generates a [`Size<Dimension>`] using [`Dimension::Percent`] values
    #[must_use]
    pub fn from_percent(width: f32, height: f32) -> Self {
        Size { width: Dimension::Percent(width), height: Dimension::Percent(height) }
    }

    /// Generates a [`Size<Dimension>`] using [`Dimension::Auto`] in both width and height
    pub const AUTO: Size<Dimension> = Self { width: Dimension::Auto, height: Dimension::Auto };

    /// Generates a [`Size<Dimension>`] using [`Dimension::Undefined`] in both width and height
    pub const UNDEFINED: Size<Dimension> = Self { width: Dimension::Undefined, height: Dimension::Undefined };
}

/// A 2-dimensional coordinate.
///
/// When used in association with a [`Rect`], represents the bottom-left corner.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point<T> {
    /// The x-coordinate
    pub x: T,
    /// The y-coordinate
    pub y: T,
}

impl Point<f32> {
    /// A [`Point`] with values (0,0), representing the origin
    pub const ZERO: Point<f32> = Self { x: 0.0, y: 0.0 };
}
