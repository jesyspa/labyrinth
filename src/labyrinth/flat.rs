use super::Labyrinth;
use crate::geometry::{Direction, Point};
use crate::views::{LabyrinthView, RoomView};
use enum_map::EnumMap;

struct Room<R, W> {
    room_info: R,
    walls: EnumMap<Direction, Option<W>>,
}

impl<R, W> Room<R, W>
where
    R: Default,
    W: Default,
{
    pub fn new(all_walls: bool) -> Self {
        let mut walls = EnumMap::default();
        if all_walls {
            for v in walls.values_mut() {
                *v = Some(W::default());
            }
        }
        Self {
            room_info: R::default(),
            walls,
        }
    }
}

pub struct FlatLabyrinth<R, W> {
    width: usize,
    height: usize,
    rooms: Vec<Room<R, W>>,
}

impl<R, W> FlatLabyrinth<R, W> {
    pub fn new(width: usize, height: usize, all_walls: bool) -> Self
    where
        R: Default,
        W: Default,
    {
        let size = width * height;
        Self {
            width,
            height,
            rooms: (0..size).map(|_| Room::new(all_walls)).collect(),
        }
    }

    fn coord_to_index(&self, Point(x, y): Point) -> usize {
        y * self.width + x
    }
}

impl<R, W> Labyrinth<R, W> for FlatLabyrinth<R, W> {
    type RoomId = Point;

    fn random_room(&self) -> Point {
        Point(0, 0)
    }

    fn get_neighbour(&self, Point(x, y): Point, dir: Direction) -> Option<Point> {
        match dir {
            Direction::North => {
                if y + 1 == self.height {
                    None
                } else {
                    Some(Point(x, y + 1))
                }
            }
            Direction::East => {
                if x + 1 == self.width {
                    None
                } else {
                    Some(Point(x + 1, y))
                }
            }
            Direction::South => {
                if y == 0 {
                    None
                } else {
                    Some(Point(x, y - 1))
                }
            }
            Direction::West => {
                if x == 0 {
                    None
                } else {
                    Some(Point(x - 1, y))
                }
            }
        }
    }

    fn room(&self, room: Point) -> &R {
        &self.rooms[self.coord_to_index(room)].room_info
    }

    fn mut_room(&mut self, room: Point) -> &mut R {
        let ix = self.coord_to_index(room);
        &mut self.rooms[ix].room_info
    }

    fn has_wall(&self, room: Point, dir: Direction) -> bool {
        let ix = self.coord_to_index(room);
        self.rooms[ix].walls[dir].is_some()
    }

    fn add_wall(&mut self, room_from: Point, dir: Direction)
    where
        W: Default,
    {
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

    fn remove_wall(&mut self, room_from: Point, dir: Direction) {
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

    fn wall(&self, room: Point, dir: Direction) -> Option<&W> {
        let ix = self.coord_to_index(room);
        self.rooms[ix].walls[dir].as_ref()
    }

    fn mut_wall(&mut self, room: Point, dir: Direction) -> Option<&mut W> {
        let ix = self.coord_to_index(room);
        self.rooms[ix].walls[dir].as_mut()
    }

    fn view(&self) -> LabyrinthView {
        let mut rooms = Vec::with_capacity(self.width * self.height);
        for r in self.rooms.iter() {
            rooms.push(RoomView::new(
                r.walls.iter().filter_map(|(d, w)| w.as_ref().map(|_| d)),
            ));
        }
        LabyrinthView::new(self.width, self.height, rooms)
    }
}
