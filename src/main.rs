mod maze;
extern crate image;

use maze::maze::{ Maze, BorderType };
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

    let imgx = resolution * width;
    let imgy = resolution * height;


    let mut imgbuf = image::ImageBuffer::from_fn(imgx, imgy, |_x, _y| {
        image::Rgb([248, 248, 248])
    });

    for row in maze.cells.iter() {
        for cell in row.iter() {
            if let BorderType::Wall = cell.up {
                for pixel_index in 0..resolution {
                    let pixel = imgbuf.get_pixel_mut(cell.row_index * resolution, pixel_index + resolution * cell.col_index);
                    let image::Rgb(_data) = *pixel;
                    *pixel = image::Rgb([128, 128, 128]);
                }
            }

            if let BorderType::Wall = cell.down {
                for pixel_index in 0..resolution {
                    let pixel = imgbuf.get_pixel_mut(cell.row_index * resolution + resolution - 1, pixel_index + resolution * cell.col_index);
                    let image::Rgb(_data) = *pixel;
                    *pixel = image::Rgb([128, 128, 128]);
                }
            }

            if let BorderType::Wall = cell.left {
                for pixel_index in 0..resolution {
                    let pixel = imgbuf.get_pixel_mut(cell.row_index * resolution + pixel_index, resolution * cell.col_index);
                    let image::Rgb(_data) = *pixel;
                    *pixel = image::Rgb([128, 128, 128]);
                }
            }

            if let BorderType::Wall = cell.right {
                for pixel_index in 0..resolution {
                    let pixel = imgbuf.get_pixel_mut(cell.row_index * resolution + pixel_index, resolution * cell.col_index + resolution - 1);
                    let image::Rgb(_data) = *pixel;
                    *pixel = image::Rgb([128, 128, 128]);
                }
            }
        }
    }

    let start_row = maze.start_position.0;
    let start_col = maze.start_position.1;

    for pixel_row in 1..(resolution - 1) {
        for pixel_col in 1..(resolution - 1) {
            if ((pixel_row + pixel_col) % 4 == 0) || ((pixel_row + pixel_col + 1) % 4 == 0) {
                update_start_cells(start_row * resolution + pixel_row, start_col * resolution + pixel_col, &mut imgbuf);
                update_start_cells(pixel_row, pixel_col, &mut imgbuf);
            }
        }
    }

    imgbuf.save("maze.png").unwrap();
}

fn update_start_cells(pixel_row: u32, pixel_col: u32, imgbuf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
    let start_pixel = imgbuf.get_pixel_mut(pixel_row, pixel_col);
    let image::Rgb(_data) = *start_pixel;
    *start_pixel = image::Rgb([252, 3, 94]);
}


