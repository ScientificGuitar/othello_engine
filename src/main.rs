use rand::prelude::IndexedRandom;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Empty,
    White,
    Black,
}

#[derive(Clone, Copy)]
struct Board {
    grid: [[Cell; 8]; 8],
    black_score: usize,
    white_score: usize,
}

type Position = (usize, usize);

impl Board {
    fn new() -> Self {
        let mut grid: [[Cell; 8]; 8] = [[Cell::Empty; 8]; 8];
        let black_score = 2;
        let white_score = 2;

        grid[3][3] = Cell::White;
        grid[3][4] = Cell::Black;
        grid[4][3] = Cell::Black;
        grid[4][4] = Cell::White;

        Self {
            grid,
            black_score,
            white_score,
        }
    }

    fn get_legal_moves(self, player: Cell) -> Vec<Position> {
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

                for (dx, dy) in directions {
                    let mut x: i32 = row as i32 + dx;
                    let mut y: i32 = col as i32 + dy;

                    let mut found_opponent: bool = false;

                    while x >= 0 && x < 8 && y >= 0 && y < 8 {
                        let cell: Cell = self.grid[x as usize][y as usize];
                        if cell == opponent {
                            found_opponent = true;
                        } else if cell == player && found_opponent {
                            legal_moves.push((row, col));
                            break;
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

    fn pick_best_move(self, player: Cell, move_list: Vec<Position>) -> Position {
        let mut best_move_difference = 0;
        let mut best_move = (0, 0); // TODO: Handle this better
        for chosen_move in move_list {
            let opponent: Cell = match player {
                Cell::Black => Cell::White,
                Cell::White => Cell::Black,
                _ => return best_move, // TODO: Handle this better
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

            let mut score_diff = 0;
            for (dx, dy) in directions {
                let mut x: i32 = chosen_move.0 as i32 + dx;
                let mut y: i32 = chosen_move.1 as i32 + dy;

                let mut score_to_add = 0;

                while x >= 0 && x < 8 && y >= 0 && y < 8 {
                    let cell: Cell = self.grid[x as usize][y as usize];
                    if cell == opponent {
                        score_to_add += 1;
                    } else if cell == player {
                        score_diff += score_to_add;
                        break;
                    } else {
                        break;
                    }

                    x += dx;
                    y += dy;
                }
            }

            if score_diff > best_move_difference {
                best_move_difference = score_diff;
                best_move = chosen_move;
            }
        }

        best_move
    }

    fn place_piece(&mut self, player: Cell, row: usize, col: usize) {
        let legal_moves = self.get_legal_moves(player);

        if !legal_moves.contains(&(row, col)) {
            println!("Illegal move!");
            return;
        }

        let opponent: Cell = match player {
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
            _ => return, // can't place a piece if it's not white or black
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

        self.grid[row][col] = player;
        if player == Cell::Black {
            self.black_score += 1;
        } else {
            self.white_score += 1;
        }
        for (dx, dy) in directions {
            let mut x: i32 = row as i32 + dx;
            let mut y: i32 = col as i32 + dy;

            let mut to_flip: Vec<Position> = vec![];

            while x >= 0 && x < 8 && y >= 0 && y < 8 {
                let cell: Cell = self.grid[x as usize][y as usize];
                if cell == opponent {
                    to_flip.push((x as usize, y as usize))
                } else if cell == player {
                    if player == Cell::Black {
                        self.black_score += to_flip.len();
                        self.white_score -= to_flip.len();
                    } else {
                        self.white_score += to_flip.len();
                        self.black_score -= to_flip.len();
                    }
                    for (fx, fy) in &to_flip {
                        self.grid[*fx][*fy] = player
                    }
                    break;
                } else {
                    break;
                }

                x += dx;
                y += dy;
            }
        }
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

        println!("Score: (B) {} - {} (W)", self.black_score, self.white_score)
    }
}

fn switch_player(player: Cell) -> Cell {
    match player {
        Cell::Black => Cell::White,
        Cell::White => Cell::Black,
        _ => Cell::Empty,
    }
}

fn main() {
    let mut black_wins = 0;
    let mut white_wins = 0;
    let mut draws = 0;

    for _ in 0..1000 {
        let mut board: Board = Board::new();
        let mut current_player = Cell::Black;

        // basic game loop
        loop {
            let legal_moves = board.get_legal_moves(current_player);
            if legal_moves.is_empty() {
                current_player = switch_player(current_player);
                let opponent_moves = board.get_legal_moves(current_player);
                if opponent_moves.is_empty() {
                    if board.black_score > board.white_score {
                        black_wins += 1;
                    } else if board.white_score > board.black_score {
                        white_wins += 1;
                    } else {
                        draws += 1;
                    }
                    break;
                }
                continue;
            }

            let chosen_move: (usize, usize);
            if current_player == Cell::Black {
                chosen_move = board.pick_best_move(current_player, legal_moves)
            } else {
                chosen_move = *legal_moves.choose(&mut rand::rng()).unwrap();
            }

            board.place_piece(current_player, chosen_move.0, chosen_move.1);
            current_player = switch_player(current_player);
        }
    }

    println!("Black Wins: {}, White Wins: {}, Draws: {}", black_wins, white_wins, draws);
}
