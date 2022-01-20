#![feature(path_try_exists)]
use std::cell::Cell;
use std::{fs, cell};

// use rand::rand;
use std::cmp::Ordering;
use std::fs::OpenOptions;
use std::io;
use std::io::BufReader;
use std::os;
use std::path::Path;
use std::time::Duration;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    mazeFile(1, 10);
}

fn mazeFile(width: u32, height: u32) {
    let mut path = Path::new("D:\\Users\\Egor\\Rust\\projects\\generatorlab\\path.txt");

    if path
        .try_exists()
        .expect("Can't check existence of file does_not_exist.txt")
    {
        fs::remove_file(path)
            .expect("could not remove file");
        println!("file is removed");
    }

    let mut file = File::create("path.txt") // Создание файла
        .expect("Не удалось создать файл");

    // let maze = genMaze(width, height);

    for y in 0..height {
        let mut northWalls = "";
        let mut sideWalls = "";
        let mut southWalls = "";
        

        for x in 0..width {
            let walls = Walls {
                north: true,
                east: true, 
                south: true,
                west: true,
            };

            if walls.north {
                northWalls = "+--+"
            } else {
                northWalls = "+  +"
            }

            if walls.west {
                sideWalls = "|"
            } else {
                sideWalls = " "
            }

            if walls.east {
                sideWalls = "  |"
            } else {
                sideWalls = "   "
            }

            if walls.south {
                southWalls = "+--+"
            } else {
                southWalls = "+  +"
            }
        }
        
        file.write((northWalls.to_string() + "\n").as_bytes()).expect("esfs");
        file.write((sideWalls.to_string() + "\n").as_bytes()).expect("esfs");
        file.write((southWalls.to_string() + "\n").as_bytes()).expect("esfs");
        
    }
}

fn genMaze(w: u32, h: u32) -> Cell {
    let mut maze = (Cell { visited, w }:Cell, w) ;
    for i in maze  {
        let mut col = Cell { visited}
        for j in col {
            let walls = Walls {
                north: true,
                east: true, 
                south: true,
                west: true,
            };
            col[j] = walls.north;
            col[j] = walls.east;
            col[j] = walls.south;
            col[j] = walls.west;
        }
        maze[i] = col;
    }

    let mut backtrack = Coord {x: , y: };
    // let maze = Cell {visited: , w: };
    // return maze;
    todo!()
}

// fn unvisitedNeighbors(){
//     let mut width = 
// }

struct Coord {
    x: i32,
    y: i32,
}
#[derive(Default)]
struct Walls {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}
struct Cell {
    visited: bool,
    w: Walls,
}

fn unvisitedCellsIn(Cell { visited, w }: Cell, maze) -> bool { //
    for col in maze{
        for j in 0..10  {
            if !visited{
                return true;
            }
        }
    }
    // let mut array = [[0u8; 4]; 6];
    // let maze = Cell {visited: , w: };
    return false;
}
