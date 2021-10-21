use crate::geometry::Direction;
use crate::views::LabyrinthView;

pub trait Labyrinth<R, W> {
    type RoomId;

    fn random_room(&self) -> Self::RoomId;
    fn get_neighbour(&self, room: Self::RoomId, dir: Direction) -> Option<Self::RoomId>;

    fn room(&self, room: Self::RoomId) -> &R;
    fn mut_room(&mut self, room: Self::RoomId) -> &mut R;

    fn has_wall(&self, room: Self::RoomId, dir: Direction) -> bool;
    fn add_wall(&mut self, room: Self::RoomId, dir: Direction)
    where
        W: Default;
    fn remove_wall(&mut self, room: Self::RoomId, dir: Direction);

    fn wall(&self, room: Self::RoomId, dir: Direction) -> Option<&W>;
    fn mut_wall(&mut self, room: Self::RoomId, dir: Direction) -> Option<&mut W>;

    fn view(&self) -> LabyrinthView;
}
