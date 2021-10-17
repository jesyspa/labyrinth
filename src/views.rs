use crate::geometry::Direction;
use crate::text_display::{TextDisplayable, TextSurfaceFocus};
use enum_map::EnumMap;

const ROOM_EDGE_LENGTH: usize = 4;

struct LabyrinthView {
    width: usize,
    height: usize,
    rooms: Vec<RoomView>,
}

impl LabyrinthView {
    fn room_index(&self, x: usize, y: usize) -> usize {
        x * self.height + y
    }
}

impl TextDisplayable for LabyrinthView {
    fn size(&self) -> (usize, usize) {
        (
            self.width * ROOM_EDGE_LENGTH,
            self.height * ROOM_EDGE_LENGTH,
        )
    }

    fn display(&self, surface: &mut TextSurfaceFocus) {
        for i in 0..self.width {
            for j in 0..self.height {
                let ix = self.room_index(i, j);
                let mut focus = surface.focus(
                    i * ROOM_EDGE_LENGTH,
                    j * ROOM_EDGE_LENGTH,
                    ROOM_EDGE_LENGTH,
                    ROOM_EDGE_LENGTH,
                );
                self.rooms[ix].display(&mut focus)
            }
        }
    }
}

struct RoomView {
    walls: EnumMap<Direction, bool>,
}

impl TextDisplayable for RoomView {
    fn size(&self) -> (usize, usize) {
        (ROOM_EDGE_LENGTH, ROOM_EDGE_LENGTH)
    }

    fn display(&self, _: &mut TextSurfaceFocus) {
        todo!()
    }
}
