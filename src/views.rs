use crate::geometry::{Direction, Offset, Point, Transform};
use crate::text_display::{TextDisplayable, TextSurfaceFocus};
use enum_map::EnumMap;

const ROOM_EDGE_LENGTH: usize = 4;
const ROOM_BOUNDARY: Point = Point(ROOM_EDGE_LENGTH, ROOM_EDGE_LENGTH);

pub struct LabyrinthView {
    width: usize,
    height: usize,
    rooms: Vec<RoomView>,
}

impl LabyrinthView {
    pub fn new(width: usize, height: usize, rooms: Vec<RoomView>) -> Self {
        assert_eq!(width * height, rooms.len());
        LabyrinthView {
            width,
            height,
            rooms,
        }
    }

    fn room_index(&self, Point(x, y): Point) -> usize {
        y * self.width + x
    }
}

impl TextDisplayable for LabyrinthView {
    fn size(&self) -> Point {
        ROOM_EDGE_LENGTH * Point(self.width, self.height)
    }

    fn display(&self, surface: &mut TextSurfaceFocus<'_>) {
        for i in 0..self.width {
            for j in 0..self.height {
                let ix = self.room_index(Point(i, j));
                let mut focus = surface.focus(
                    Offset(i * ROOM_EDGE_LENGTH, j * ROOM_EDGE_LENGTH),
                    self.rooms[ix].size(),
                );
                self.rooms[ix].display(&mut focus)
            }
        }
    }
}

pub struct RoomView {
    walls: EnumMap<Direction, bool>,
}

impl RoomView {
    pub fn new<T>(wall_list: T) -> Self
    where
        T: Iterator<Item = Direction>,
    {
        let mut walls = EnumMap::default();
        for direction in wall_list {
            walls[direction] = true;
        }
        RoomView { walls }
    }
}

impl TextDisplayable for RoomView {
    fn size(&self) -> Point {
        ROOM_BOUNDARY
    }

    fn display(&self, focus: &mut TextSurfaceFocus<'_>) {
        for d in Direction::iter() {
            let t = Transform::new(ROOM_EDGE_LENGTH, d);
            focus.set(t * Point(0, 0), '#');
            if self.walls[d] {
                focus.set(t * Point(1, 0), '#');
                focus.set(t * Point(2, 0), '#');
            }
        }
    }
}
