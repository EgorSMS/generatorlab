// use rand::rand;
use std::cmp::Ordering;
use std::io;
use std::os;
use std::time::Duration;


use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    //TODO
    
}



fn MazeFile(width: i32, height: i32){
    let mut file = File::create("labirint.txt")
    .expect("Не удалось создать файл");

    
    let x = 0;
    let y = 0;

    let maze = genMaze(width, height);

    for y in y..height
    {
        let northWalls = "";
        let sideWalls = "";
        let southWalls = "";


        for x in x..width
        {
            if 
            let walls = Walls {}
            if Walls.north {
                northWalls += "+--+"
            } else {
                northWalls += "+  +"
            }

            if(Walls)

        }
        
    }

}

fn genMaze(w:i32, h:i32) -> Cell {
    let mut array = [[0u8; 4]; 6];
    let maze = Cell {visited: , w: };
    return maze;
}

struct Coord {
    x : i32,
    y : i32
}
struct Walls {
    north : bool,
    east : bool,
    south : bool,
    west : bool
}
struct Cell {
    visited : bool,
    w : Walls
}

fn unvisitedCellsIn(maze [][]cell) -> Cell {
    let mut array = [[0u8; 4]; 6];
    let maze = Cell {visited: , w: };
    return maze;
}