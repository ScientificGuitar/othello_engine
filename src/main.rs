#[derive(Copy, Clone, Debug)]
enum Cell {
    Empty,
    White,
    Black,
}

struct Board {
    grid: [[Cell; 8]; 8],
}


impl Board {
    fn new() -> Self {
        let mut grid: [[Cell; 8]; 8] = [[Cell::Empty; 8]; 8];

        grid[3][3] = Cell::White;
        grid[3][4] = Cell::Black;
        grid[4][3] = Cell::Black;
        grid[4][4] = Cell::White;

        Self { grid }
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
}
