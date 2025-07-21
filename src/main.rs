#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Empty,
    White,
    Black,
}

#[derive(Clone, Copy)]
struct Board {
    grid: [[Cell; 8]; 8],
}

type Position = (usize, usize);

impl Board {
    fn new() -> Self {
        let mut grid: [[Cell; 8]; 8] = [[Cell::Empty; 8]; 8];

        grid[3][3] = Cell::White;
        grid[3][4] = Cell::Black;
        grid[4][3] = Cell::Black;
        grid[4][4] = Cell::White;

        Self { grid }
    }

    fn get_legal_moves(self, player: Cell) -> Vec<Position> {
        let opponent: Cell = match player {
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
            _ => return vec![], // return no legal moves if player is not valid
        };

        let directions: [(i32, i32); 8] = [(1, 1), (1, 0), (1, -1), (0, 1), (0, -1), (-1, 1), (-1, 0), (-1, -1)];

        let mut legal_moves: Vec<Position> = vec![];

        for row in 0..8 {
            for col in 0..8 {
                if self.grid[row][col] != Cell::Empty {
                    continue;
                }

                for (dx, dy) in directions {
                    let mut x: i32 = row as i32 + dx;
                    let mut y: i32 = col as i32 + dy;

                    let mut found_opponent: bool = false;

                    while x >= 0 && x < 8 && y >= 0 && y < 8 {
                        let cell: Cell = self.grid[x as usize][y as usize];
                        if cell == opponent {
                            found_opponent = true;
                        }
                        else if cell == player && found_opponent {
                            legal_moves.push((row, col));
                            break;
                        }
                        else {
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

    fn display(self) {
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


fn main() {
    let board: Board = Board::new();
    board.display();

    println!("White legal moves: {:?}", board.get_legal_moves(Cell::White));
    println!("Black legal moves: {:?}", board.get_legal_moves(Cell::Black));
}
