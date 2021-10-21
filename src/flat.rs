use crate::geometry::Direction;
use crate::labyrinth::Labyrinth;
use crate::views::{LabyrinthView, RoomView};
use enum_map::EnumMap;

struct Room<R, W> {
    room_info: R,
    walls: EnumMap<Direction, Option<W>>,
}

type RoomCoords = (usize, usize);

pub struct FlatLabyrinth<R, W> {
    height: usize,
    width: usize,
    rooms: Vec<Room<R, W>>,
}

impl<R, W> FlatLabyrinth<R, W> {
    fn coord_to_index(&self, (x, y): RoomCoords) -> usize {
        y * self.width + x
    }
}

impl<R, W> Labyrinth<R, W> for FlatLabyrinth<R, W>
where
    R: Default,
    W: Default,
{
    type RoomId = RoomCoords;

    fn get_neighbour(&self, (x, y): RoomCoords, dir: Direction) -> Option<RoomCoords> {
        match dir {
            Direction::North => {
                if y + 1 == self.height {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Direction::East => {
                if x + 1 == self.width {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            Direction::South => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Direction::West => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
        }
    }

    fn room(&self, room: RoomCoords) -> &R {
        &self.rooms[self.coord_to_index(room)].room_info
    }

    fn mut_room(&mut self, room: RoomCoords) -> &mut R {
        let ix = self.coord_to_index(room);
        &mut self.rooms[ix].room_info
    }

    fn has_wall(&self, room: RoomCoords, dir: Direction) -> bool {
        let ix = self.coord_to_index(room);
        self.rooms[ix].walls[dir].is_some()
    }

    fn add_wall(&mut self, room_from: RoomCoords, dir: Direction) {
        if let Some(room_to) = self.get_neighbour(room_from, dir) {
            let ix_from = self.coord_to_index(room_from);
            let ix_to = self.coord_to_index(room_to);

            // Do these one by one so the borrow checker is happy.
            let wall_from = &mut self.rooms[ix_from].walls[dir];
            assert!(
                wall_from.is_none(),
                "Room {:?} already has a wall to the {:?}",
                room_from,
                dir
            );
            *wall_from = Some(W::default());

            let wall_to = &mut self.rooms[ix_to].walls[dir.flip()];
            assert!(
                wall_to.is_none(),
                "Room {:?} already has a wall to the {:?}",
                room_to,
                dir
            );
            *wall_to = Some(W::default());
        } else {
            panic!("Room {:?} has no neighbour to the {:?}", room_from, dir);
        }
    }

    fn remove_wall(&mut self, room_from: RoomCoords, dir: Direction) {
        if let Some(room_to) = self.get_neighbour(room_from, dir) {
            let ix_from = self.coord_to_index(room_from);
            let ix_to = self.coord_to_index(room_to);

            // Do these one by one so the borrow checker is happy.
            let wall_from = &mut self.rooms[ix_from].walls[dir];
            assert!(
                wall_from.is_some(),
                "Room {:?} already has no wall to the {:?}",
                room_from,
                dir
            );
            *wall_from = None;

            let wall_to = &mut self.rooms[ix_to].walls[dir.flip()];
            assert!(
                wall_to.is_some(),
                "Room {:?} already has no wall to the {:?}",
                room_to,
                dir
            );
            *wall_to = None;
        } else {
            panic!("Room {:?} has no neighbour to the {:?}", room_from, dir);
        }
    }

    fn wall(&self, room: RoomCoords, dir: Direction) -> Option<&W> {
        let ix = self.coord_to_index(room);
        self.rooms[ix].walls[dir].as_ref()
    }

    fn mut_wall(&mut self, room: RoomCoords, dir: Direction) -> Option<&mut W> {
        let ix = self.coord_to_index(room);
        self.rooms[ix].walls[dir].as_mut()
    }

    fn view(&self) -> LabyrinthView {
        let mut rooms = Vec::with_capacity(self.width * self.height);
        for r in self.rooms.iter() {
            rooms.push(RoomView::new(r.walls.iter().filter_map(|(d, w)| {
                w.as_ref().map(|_| d)
            })));
        }
        LabyrinthView::new(self.width, self.height, rooms)
    }
}