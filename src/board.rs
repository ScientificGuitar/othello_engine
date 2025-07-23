#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    White,
    Black,
}

#[derive(Clone, Copy)]
pub struct Board {
    grid: [[Cell; 8]; 8],
}

pub type Position = (usize, usize);

impl Board {
    pub fn new() -> Self {
        let mut grid: [[Cell; 8]; 8] = [[Cell::Empty; 8]; 8];

        grid[3][3] = Cell::White;
        grid[3][4] = Cell::Black;
        grid[4][3] = Cell::Black;
        grid[4][4] = Cell::White;

        Self { grid }
    }

    pub fn get_legal_moves(&self, player: Cell) -> Vec<Position> {
        let opponent: Cell = match player {
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
            _ => return vec![], // return no legal moves if player is not valid
        };

        let directions: [(i32, i32); 8] = [
            (1, 1),
            (1, 0),
            (1, -1),
            (0, 1),
            (0, -1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];

        let mut legal_moves: Vec<Position> = vec![];

        for row in 0..8 {
            for col in 0..8 {
                if self.grid[row][col] != Cell::Empty {
                    continue;
                }

                'outer: for (dx, dy) in directions {
                    let mut x: i32 = row as i32 + dx;
                    let mut y: i32 = col as i32 + dy;

                    if x < 0 || x >= 8 || y < 0 || y >= 8 {
                        continue;
                    }

                    if self.grid[x as usize][y as usize] != opponent {
                        continue;
                    }

                    x += dx;
                    y += dy;

                    while x >= 0 && x < 8 && y >= 0 && y < 8 {
                        let cell: Cell = self.grid[x as usize][y as usize];
                        if cell == opponent {
                            x += dx;
                            y += dy;
                        } else if cell == player {
                            legal_moves.push((row, col));
                            break 'outer;
                        } else {
                            break;
                        }

                        x += dx;
                        y += dy;
                    }
                }
            }
        }

        legal_moves
    }

    pub fn place_piece(&self, player: Cell, mv: Position) -> Board {
        let mut board = self.clone();
        let opponent: Cell = match player {
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
            _ => return board, // can't place a piece if it's not white or black
        };

        let directions: [(i32, i32); 8] = [
            (1, 1),
            (1, 0),
            (1, -1),
            (0, 1),
            (0, -1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];

        board.grid[mv.0][mv.1] = player;

        for (dx, dy) in directions {
            let mut x: i32 = mv.0 as i32 + dx;
            let mut y: i32 = mv.1 as i32 + dy;

            let mut to_flip: Vec<Position> = vec![];

            while x >= 0 && x < 8 && y >= 0 && y < 8 {
                let cell: Cell = self.grid[x as usize][y as usize];
                if cell == opponent {
                    to_flip.push((x as usize, y as usize))
                } else if cell == player {
                    for (fx, fy) in &to_flip {
                        board.grid[*fx][*fy] = player
                    }
                    break;
                } else {
                    break;
                }

                x += dx;
                y += dy;
            }
        }

        board
    }

    pub fn count_pieces(&self) -> (i32, i32) {
        let mut black_score = 0;
        let mut white_score = 0;

        for row in 0..8 {
            for col in 0..8 {
                if self.grid[row][col] == Cell::Black {
                    black_score += 1;
                } else if self.grid[row][col] == Cell::White {
                    white_score += 1;
                }
            }
        }

        return (black_score, white_score);
    }

    pub fn display(self) {
        println!("  0 1 2 3 4 5 6 7");
        for (i, row) in self.grid.iter().enumerate() {
            print!("{} ", i);
            for col in row.iter() {
                let symbol: &'static str = match col {
                    Cell::Empty => ".",
                    Cell::White => "W",
                    Cell::Black => "B",
                };
                print!("{} ", symbol)
            }
            println!();
        }
    }
}
