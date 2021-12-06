// use rand::rand;
use std::cmp::Ordering;
use std::io;
use std::os;
use std::time::Duration;
fn main() {
    //TODOq
}

struct Coord {
    x : i32,
    y : i32
}
struct Walls {
    noorth : bool,
    east : bool,
    south : bool,
    west : bool
}
struct Cell {
    visited : bool,
    w : Walls
}