use crate::view_geometry::{Offset, Point};
use std::fmt::{Display, Formatter};
use std::result::Result;

pub struct TextSurface {
    width: usize,
    data: Vec<Vec<u8>>,
}

impl TextSurface {
    pub fn new(width: usize, height: usize) -> Self {
        TextSurface {
            width,
            data: vec![vec![b' '; width]; height],
        }
    }

    fn set(&mut self, Point(x, y): Point, c: char) {
        self.data[y][x] = c as u8;
    }

    pub fn get_focus(&mut self) -> TextSurfaceFocus<'_> {
        let x_max = self.width;
        let y_max = self.data.len();
        TextSurfaceFocus {
            surface: self,
            offset: Offset::default(),
            bound: Point(x_max, y_max),
        }
    }

    pub fn create_and_draw<T: TextDisplayable>(obj: &T) -> Self {
        let Point(w, h) = obj.size();
        let mut surface = Self::new(w, h);
        obj.display(&mut surface.get_focus());
        surface
    }
}

impl Display for TextSurface {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for line in self.data.iter().rev() {
            writeln!(fmt, "{}", String::from_utf8_lossy(line))?;
        }
        Ok(())
    }
}

pub struct TextSurfaceFocus<'surface> {
    surface: &'surface mut TextSurface,
    offset: Offset,
    bound: Point,
}

impl<'surface> TextSurfaceFocus<'surface> {
    pub fn set(&mut self, p: Point, c: char) {
        assert!(p.within(self.bound));
        self.surface.set(p + self.offset, c);
    }

    pub fn focus(&mut self, start: Offset, bound: Point) -> TextSurfaceFocus<'_> {
        let offset = self.offset + start;
        assert!((bound + offset).bounded_by(self.bound));
        TextSurfaceFocus {
            surface: self.surface,
            offset,
            bound,
        }
    }
}

pub trait TextDisplayable {
    fn size(&self) -> Point;
    fn display(&self, surface: &mut TextSurfaceFocus<'_>);
}
