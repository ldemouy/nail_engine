use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum DirectionHint {
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
    Up,
    Down,
    None,
}

pub trait MappableExit: nail_core::traits::Exit {
    fn get_direction_hint() -> DirectionHint;
}

pub trait DrawableRoom<E, I>: nail_core::traits::Room<E, I>
where
    E: nail_core::traits::Exit,
    I: nail_core::traits::Item,
{
    fn get_drawn_room();
}
