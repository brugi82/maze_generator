mod maze;
extern crate image;

use maze::maze::{ Maze };
use std::io;

fn main() {
    let resolution = 32;

    println!("Enter maze width (default 16):");
    let mut width = String::new();
    io::stdin().read_line(&mut width)
        .expect("Failed to read the line...");
    let width: u32 = width.trim().parse()
        .expect("Please type a number...");

    println!("Enter maze height (default 16):");
    let mut height = String::new();
    io::stdin().read_line(&mut height)
        .expect("Failed to read the line...");
    let height: u32 = height.trim().parse()
        .expect("Please type a number...");

    println!("Creating maze...");

    let mut maze = Maze::new(width, height);
    maze.generate_maze();

    maze.draw(resolution, "maze.png".parse().unwrap());
}


