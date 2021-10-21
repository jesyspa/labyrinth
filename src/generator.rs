use crate::geometry::Direction;
use crate::labyrinth::Labyrinth;
use rand::seq::IteratorRandom;
use std::collections::HashSet;
use std::hash::Hash;

pub fn random_walk<R, W, L: Labyrinth<R, W>>(lab: &mut L)
where
    W: Default,
    L::RoomId: Hash,
{
    let mut rng = rand::thread_rng();
    let mut explored = HashSet::<L::RoomId>::new();
    let mut stack = vec![lab.random_room()];

    while !stack.is_empty() {
        let r = stack[stack.len() - 1];
        explored.insert(r);
        let d = Direction::iter()
            .filter(|&d| {
                if let Some(r2) = lab.get_neighbour(r, d) {
                    !explored.contains(&r2)
                } else {
                    false
                }
            })
            .choose(&mut rng);
        if let Some(d) = d {
            lab.remove_wall(r, d);
            let r2 = lab
                .get_neighbour(r, d)
                .expect("Logic error: we checked this neighbour existed.");
            stack.push(r2);
        } else {
            stack.pop();
        }
    }
}
