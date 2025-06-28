//! Caixas (boxes in portuguese) is a simple library that implements combinators for text boxes.

use std::fmt::{Display, Write};

/// The main type of the library that represent a text box.
#[must_use]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Box {
    contents: String,
    width: usize,
    height: usize,
}

/// Sets the horizontal aligment of the text in a `Box`.
#[derive(Debug, Default, Clone, Copy)]
pub enum Horizontal {
    Left,
    #[default]
    Center,
    Right,
}

/// Sets the vertical aligment of the text in a `Box`.
#[derive(Debug, Default, Clone, Copy)]
pub enum Vertical {
    Top,
    #[default]
    Center,
    Bottom,
}

impl Box {
    /// Constructs an empty `Box`, i.e., the default one.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Constructs a `Box` filled with a given character.
    pub fn fill(c: char, width: usize, height: usize) -> Self {
        Self {
            contents: c.to_string().repeat(width * height),
            width,
            height,
        }
    }

    /// Constructs a `Box` filled with spaces.
    pub fn space(width: usize, height: usize) -> Self {
        Self::fill(' ', width, height)
    }

    /// The width of a `Box`.
    #[must_use]
    pub const fn width(&self) -> usize {
        self.width
    }

    /// The height of a `Box`.
    #[must_use]
    pub const fn height(&self) -> usize {
        self.height
    }

    /// Helper method to get the nth chunk of a `Box`.
    fn chunk(&self, n: usize) -> Option<&str> {
        let r = n * self.width..(n + 1) * self.width;
        self.contents.get(r)
    }

    /// Stacks `self` besides `other`.
    pub fn beside(self, other: Self, alignment: Vertical) -> Self {
        if self.width == 0 {
            return other;
        }

        if other.width == 0 {
            return self;
        }

        let height = self.height;
        let left = self.heighten(other.height, alignment);
        let right @ Self { height, .. } = other.heighten(height, alignment);

        let mut contents = String::with_capacity((left.width + right.width) * height);
        let mut i = 0;
        while let (Some(a), Some(b)) = (left.chunk(i), right.chunk(i)) {
            contents.push_str(a);
            contents.push_str(b);
            i += 1;
        }

        debug_assert!(left.width * i >= left.contents.len());
        debug_assert!(right.width * i >= right.contents.len());

        Self {
            contents,
            width: left.width + right.width,
            height,
        }
    }

    /// Stacks `self` above `other`.
    pub fn above(self, other: Self, alignment: Horizontal) -> Self {
        if self.width == 0 {
            return other;
        }

        if other.width == 0 {
            return self;
        }

        let (tw, bw) = (self.width, other.width);
        let t = self.widen(bw, alignment);
        let b = other.widen(tw, alignment);

        Self {
            contents: String::from_iter([t.contents, b.contents]),
            width: t.width,
            height: t.height + b.height,
        }
    }

    /// Creates a new `Box` at least `width` units wide.
    pub fn widen(self, width: usize, alignment: Horizontal) -> Self {
        if self.width >= width {
            return self;
        }

        let (w, h) = (width - self.width, self.height);
        match alignment {
            Horizontal::Left => self.beside(Self::space(w, h), Vertical::default()),
            Horizontal::Center => Self::hconcat(
                [Self::space(w / 2, h), self, Self::space(w - w / 2, h)],
                Vertical::default(),
            ),
            Horizontal::Right => Self::space(w, h).beside(self, Vertical::default()),
        }
    }

    /// Creates a new `Box` at least `height` units high.
    pub fn heighten(self, height: usize, alignment: Vertical) -> Self {
        if self.height >= height {
            return self;
        }

        let (w, h) = (self.width, height - self.height);
        match alignment {
            Vertical::Top => self.above(Self::space(w, h), Horizontal::default()),
            Vertical::Center => Self::vconcat(
                [Self::space(w, h / 2), self, Self::space(w, h - h / 2)],
                Horizontal::default(),
            ),
            Vertical::Bottom => Self::space(w, h).above(self, Horizontal::default()),
        }
    }

    /// Consumes and concatenates horizontally a sequence of boxes producing a new single box.
    pub fn hconcat(boxes: impl IntoIterator<Item = Self>, alignment: Vertical) -> Self {
        boxes
            .into_iter()
            .fold(Self::empty(), |acc, b| acc.beside(b, alignment))
    }

    /// Consumes and concatenates vertically a sequence of boxes producing a new single box.
    pub fn vconcat(boxes: impl IntoIterator<Item = Self>, alignment: Horizontal) -> Self {
        boxes
            .into_iter()
            .fold(Self::empty(), |acc, b| acc.above(b, alignment))
    }

    /// Consumes and concatenates a 2D sequence of boxes producing a new single box.
    pub fn grid(boxes: impl IntoIterator<Item = impl IntoIterator<Item = Self>>) -> Self {
        Self::vconcat(
            boxes
                .into_iter()
                .map(|b| Self::hconcat(b, Vertical::default())),
            Horizontal::default(),
        )
    }

    /// Produces a new box with a frame around the `self` contents.
    pub fn framed(self) -> Self {
        let (w, h) = (self.width, self.height);

        let vbar = || Self::fill('|', 1, h);
        let hbar = || Self::fill('-', w, 1);
        let corner = || Self::from('+');

        Self::grid([
            [corner(), hbar(), corner()],
            [vbar(), self, vbar()],
            [corner(), hbar(), corner()],
        ])
    }
}

impl From<String> for Box {
    fn from(value: String) -> Self {
        Self {
            width: value.len(),
            height: 1,
            contents: value,
        }
    }
}

impl From<char> for Box {
    fn from(value: char) -> Self {
        Self::from(String::from(value))
    }
}

impl From<&str> for Box {
    fn from(value: &str) -> Self {
        Self::from(value.to_owned())
    }
}

impl Display for Box {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;
        while let Some(chunk) = self.chunk(i) {
            f.write_str(chunk)?;
            f.write_char('\n')?;
            i += 1;
        }

        Ok(())
    }
}
