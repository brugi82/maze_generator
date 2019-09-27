pub mod maze {
    use rand::Rng;
    extern crate image;

    pub enum BorderType {
        Passage,
        Wall,
    }

    #[derive(Copy, Clone)]
    pub enum MoveDirection {
        Left,
        Up,
        Right,
        Down,
    }

    pub struct Cell {
        pub left: BorderType,
        pub up: BorderType,
        pub right: BorderType,
        pub down: BorderType,
        pub visited: bool,
        pub row_index: u32,
        pub col_index: u32,
    }

    pub struct Maze {
        pub width: u32,
        pub height: u32,
        pub cells: Vec<Vec<Cell>>,
        pub current_position: (u32, u32),
        pub moves: Vec<(u32, u32)>,
        pub visited_count: u32,
        pub start_position: (u32, u32),
        total_cell_count: u32,
        max_len: u32,
    }

    impl Cell {
        pub fn new(row_index: u32, col_index: u32) -> Cell {
            Cell {
                row_index: row_index,
                col_index: col_index,
                visited: false,
                up: BorderType::Wall,
                left: BorderType::Wall,
                down: BorderType::Wall,
                right: BorderType::Wall,
            }
        }
    }

    impl Maze {
        pub fn new(width: u32, height: u32) -> Maze {
            if width < 1 || height < 1 {
                panic!("Dimensions of the Maze have to be positive numbers.");
            }

            let mut cells: Vec<Vec<Cell>> = vec![];
            for row_index in 0..height {
                let mut row: Vec<Cell> = vec![];
                for col_index in 0..width {
                    let cell = Cell::new(row_index, col_index);
                    row.push(cell);
                }

                cells.push(row);
            }

            cells[0][0].visited = true;

            let mut moves: Vec<(u32, u32)> = vec![];
            moves.push((0,0));

            Maze {
                cells,
                current_position: (0, 0),
                width,
                height,
                moves,
                visited_count: 1,
                total_cell_count: width * height,
                start_position: (0, 0),
                max_len: 1,
            }
        }

        pub fn generate_maze(&mut self) {
            while self.visited_count < self.total_cell_count {
                let available_moves = &self.get_available_moves();
                let next_move = &self.choose_move(&available_moves);
                match next_move {
                    Some(dir) => self.make_move(&dir),
                    None => {
                        let trackback_move = self.moves.pop().unwrap();
                        self.current_position = trackback_move;
                    }
                }
            }
        }

        pub fn draw(&self, resolution: u32, path: String) {
            let imgx = resolution * self.width;
                let imgy = resolution * self.height;


                let mut imgbuf = image::ImageBuffer::from_fn(imgx, imgy, |_x, _y| {
                    image::Rgb([248, 248, 248])
                });

                for row in self.cells.iter() {
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

                let start_row = self.start_position.0;
                let start_col = self.start_position.1;

                for pixel_row in 1..(resolution - 1) {
                    for pixel_col in 1..(resolution - 1) {
                        if ((pixel_row + pixel_col) % 4 == 0) || ((pixel_row + pixel_col + 1) % 4 == 0) {
                            self.update_start_cells(start_row * resolution + pixel_row, start_col * resolution + pixel_col, &mut imgbuf);
                            self.update_start_cells(pixel_row, pixel_col, &mut imgbuf);
                        }
                    }
                }

                imgbuf.save(path).unwrap();
        }

        fn get_available_moves(&self) -> Vec<MoveDirection> {
            let mut available_moves: Vec<MoveDirection> = vec![];

            if (self.current_position.1 > 0) && (self.get_cell(self.current_position.0, self.current_position.1 - 1).visited == false) {
                available_moves.push(MoveDirection::Left);
            }
            if (self.current_position.1 < self.width - 1) && (self.get_cell(self.current_position.0, self.current_position.1 + 1).visited == false) {
                available_moves.push(MoveDirection::Right);
            }
            if (self.current_position.0 > 0) && (self.get_cell(self.current_position.0 - 1, self.current_position.1).visited == false) {
                available_moves.push(MoveDirection::Up);
            }
            if (self.current_position.0 < self.height - 1) && (self.get_cell(self.current_position.0 + 1, self.current_position.1).visited == false) {
                available_moves.push(MoveDirection::Down);
            }

            available_moves
        }

        fn choose_move(&self, moves: &Vec<MoveDirection>) -> Option<MoveDirection> {
            match moves.len() {
                1 => Option::Some(moves[0].clone()),
                l if l > 0 => {
                    let move_index = rand::thread_rng().gen_range(0, moves.len());
                    Option::Some(moves[move_index].clone())
                },
                _ => None,
            }
        }

        fn make_move(&mut self, direction: &MoveDirection) {
            let next_index = self.get_next_index(direction);

            match direction {
                MoveDirection::Left => {
                    self.set_cell_passage(self.current_position, &MoveDirection::Left);
                    self.set_cell_passage(next_index, &MoveDirection::Right);
                },
                MoveDirection::Up => {
                    self.set_cell_passage(self.current_position, &MoveDirection::Up);
                    self.set_cell_passage(next_index, &MoveDirection::Down);
                },
                MoveDirection::Right => {
                    self.set_cell_passage(self.current_position, &MoveDirection::Right);
                    self.set_cell_passage(next_index, &MoveDirection::Left);
                },
                MoveDirection::Down => {
                    self.set_cell_passage(self.current_position, &MoveDirection::Down);
                    self.set_cell_passage(next_index, &MoveDirection::Up);
                }
            }

            let next_cell = self.get_cell_mut(next_index.0, next_index.1);
            next_cell.visited = true;
            self.visited_count += 1;
            self.moves.push(next_index);
            self.current_position = next_index;
            self.update_start_position();
        }

        fn set_cell_passage(&mut self, cell_index: (u32, u32), side: &MoveDirection) {
            let cell = self.get_cell_mut(cell_index.0, cell_index.1);
            match side {
                MoveDirection::Left => cell.left = BorderType::Passage,
                MoveDirection::Up => cell.up = BorderType::Passage,
                MoveDirection::Right => cell.right = BorderType::Passage,
                MoveDirection::Down => cell.down = BorderType::Passage,
            }
        }

        fn get_next_index(&self, direction: &MoveDirection) -> (u32, u32) {
            match direction {
                MoveDirection::Left => (self.current_position.0, self.current_position.1 - 1),
                MoveDirection::Up => (self.current_position.0 - 1, self.current_position.1),
                MoveDirection::Right => (self.current_position.0, self.current_position.1 + 1),
                MoveDirection::Down => (self.current_position.0 + 1, self.current_position.1),
            }
        }

        pub fn get_cell(&self, row_index: u32, col_index: u32) -> &Cell {
            &self.cells[row_index as usize][col_index as usize]
        }

        fn get_cell_mut(&mut self, row_index: u32, col_index: u32) -> &mut Cell {
            &mut self.cells[row_index as usize][col_index as usize]
        }

        fn update_start_position(&mut self) {
            if self.max_len < self.moves.len() as u32 {
                self.start_position = (self.current_position.0, self.current_position.1);
                self.max_len += 1;
            }
        }

        fn update_start_cells(&self, pixel_row: u32, pixel_col: u32, imgbuf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
            let start_pixel = imgbuf.get_pixel_mut(pixel_row, pixel_col);
            let image::Rgb(_data) = *start_pixel;
            *start_pixel = image::Rgb([252, 3, 94]);
        }
    }


}