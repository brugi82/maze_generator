mod maze;
extern crate image;

use maze::maze::{ Maze, BorderType };

fn main() {
    let resolution = 32;
    let width = 96;
    let height = 96;

    let mut maze = Maze::new(width, height);
    maze.generate_maze();

    let imgx = resolution * width;
    let imgy = resolution * height;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    for row in maze.cells.iter() {
        for cell in row.iter() {
            if let BorderType::Wall = cell.up {
                for pixel_index in 0..resolution {
                    let pixel = imgbuf.get_pixel_mut(cell.row_index * resolution, pixel_index + resolution * cell.col_index);
                    //pixel = image::Rgb([212, 212, 212]);
                    let image::Rgb(_data) = *pixel;
                    *pixel = image::Rgb([212, 212, 212]);
                }
            }

            if let BorderType::Wall = cell.down {
                for pixel_index in 0..resolution {
                    let pixel = imgbuf.get_pixel_mut(cell.row_index * resolution + resolution - 1, pixel_index + resolution * cell.col_index);
                    //pixel = image::Rgb([212, 212, 212]);
                    let image::Rgb(_data) = *pixel;
                    *pixel = image::Rgb([212, 212, 212]);
                }
            }

            if let BorderType::Wall = cell.left {
                for pixel_index in 0..resolution {
                    let pixel = imgbuf.get_pixel_mut(cell.row_index * resolution + pixel_index, resolution * cell.col_index);
                    //pixel = image::Rgb([212, 212, 212]);
                    let image::Rgb(_data) = *pixel;
                    *pixel = image::Rgb([212, 212, 212]);
                }
            }

            if let BorderType::Wall = cell.right {
                for pixel_index in 0..resolution {
                    let pixel = imgbuf.get_pixel_mut(cell.row_index * resolution + pixel_index, resolution * cell.col_index + resolution - 1);
                    //pixel = image::Rgb([212, 212, 212]);
                    let image::Rgb(_data) = *pixel;
                    *pixel = image::Rgb([212, 212, 212]);
                }
            }
        }
    }
    //imgbuf.
    imgbuf.save("maze.png").unwrap();
}


