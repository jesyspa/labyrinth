use std::iter::FromIterator;

pub struct TextSurface {
    width: usize,
    data: Vec<Vec<u8>>,
}

impl TextSurface {
    pub fn new(width: usize, height: usize) -> Self {
        use std::iter::repeat;
        TextSurface {
            width,
            data: repeat(repeat(' ' as u8).take(width).collect())
                .take(height)
                .collect(),
        }
    }

    fn set(&mut self, x: usize, y: usize, c: char) {
        self.data[x][y] = c as u8;
    }

    pub fn get_focus<'a>(&'a mut self) -> TextSurfaceFocus<'a> {
        let x_max = self.width;
        let y_max = self.data.len();
        TextSurfaceFocus {
            surface: self,
            x_offset: 0,
            y_offset: 0,
            x_max,
            y_max,
        }
    }

    pub fn print(&self) {
        for line in self.data.iter() {
            println!("{}", String::from_iter(line.iter().map(|i| *i as char)))
        }
    }
}

pub struct TextSurfaceFocus<'surface> {
    surface: &'surface mut TextSurface,
    x_offset: usize,
    y_offset: usize,
    x_max: usize,
    y_max: usize,
}

impl<'surface> TextSurfaceFocus<'surface> {
    pub fn set(&mut self, x: usize, y: usize, c: char) {
        assert!(x < self.x_max);
        assert!(y < self.y_max);
        self.surface.set(self.x_offset + x, self.y_offset + y, c);
    }

    pub fn focus<'new_focus>(
        &'new_focus mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> TextSurfaceFocus<'new_focus> {
        assert!(width <= self.x_max);
        assert!(height <= self.y_max);
        TextSurfaceFocus {
            surface: self.surface,
            x_offset: self.x_offset + x,
            y_offset: self.y_offset + y,
            x_max: width,
            y_max: height,
        }
    }
}

pub trait TextDisplayable {
    fn size(&self) -> (usize, usize);
    fn display(&self, surface: &mut TextSurfaceFocus);
}
