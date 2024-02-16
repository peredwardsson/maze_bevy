use bevy::prelude::*;

pub trait MainState<T: States> {
    fn main() -> T;
}