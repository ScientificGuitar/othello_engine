use std::cmp;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn opponent(self) -> Self {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

#[derive(Clone)]
pub struct Board {
    grid: Vec<Vec<Option<Player>>>,
    size: usize,
}

pub type Move = (usize, usize);

impl Board {
    pub fn new(mut board_size: usize) -> Self {
        if board_size % 2 == 1 {
            board_size += 1;
        }
        let size = cmp::min(cmp::max(board_size, 2), 8);
        let upper = size / 2;
        let lower = upper - 1;

        let mut grid = vec![vec![None; board_size]; board_size];

        grid[lower][lower] = Some(Player::White);
        grid[lower][upper] = Some(Player::Black);
        grid[upper][lower] = Some(Player::Black);
        grid[upper][upper] = Some(Player::White);

        Self { grid, size }
    }

    pub fn get_legal_moves(&self, player: Player) -> Vec<Move> {
        let opponent = player.opponent();

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

        let mut legal_moves: Vec<Move> = vec![];

        for row in 0..self.size {
            for col in 0..self.size {
                if self.grid[row][col] != None {
                    continue;
                }

                'outer: for (dx, dy) in directions {
                    let mut x: i32 = row as i32 + dx;
                    let mut y: i32 = col as i32 + dy;

                    if x < 0 || x >= self.size as i32 || y < 0 || y >= self.size as i32 {
                        continue;
                    }

                    if self.grid[x as usize][y as usize] != Some(opponent) {
                        continue;
                    }

                    x += dx;
                    y += dy;

                    while x >= 0 && x < self.size as i32 && y >= 0 && y < self.size as i32 {
                        match self.grid[x as usize][y as usize] {
                            Some(p) if p == opponent => {
                                x += dx;
                                y += dy;
                            }
                            Some(p) if p == player => {
                                legal_moves.push((row, col));
                                break 'outer;
                            }
                            Some(_) => break, // catch-all for other players, not currently possible
                            None => break,
                        }
                    }
                }
            }
        }

        legal_moves
    }

    pub fn place_piece(&mut self, player: Player, mv: Move) -> Option<Board> {
        if self.is_legal_move(player, mv) {
            Some(self.place_piece_unchecked(player, mv))
        } else {
            None
        }
    }

    pub fn place_piece_unchecked(&mut self, player: Player, mv: Move) -> Board {
        let mut board = self.clone();
        let opponent = player.opponent();

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

        board.grid[mv.0][mv.1] = Some(player);

        for (dx, dy) in directions {
            let mut x: i32 = mv.0 as i32 + dx;
            let mut y: i32 = mv.1 as i32 + dy;

            let mut to_flip: Vec<Move> = vec![];

            while x >= 0 && x < self.size as i32 && y >= 0 && y < self.size as i32 {
                match board.grid[x as usize][y as usize] {
                    Some(p) if p == opponent => to_flip.push((x as usize, y as usize)),
                    Some(p) if p == player => {
                        for (fx, fy) in &to_flip {
                            board.grid[*fx][*fy] = Some(player)
                        }
                        break;
                    }
                    Some(_) => break, // catch-all for other players, not currently possible
                    None => break,
                }

                x += dx;
                y += dy;
            }
        }

        board
    }

    pub fn is_legal_move(&self, player: Player, mv: Move) -> bool {
        self.get_legal_moves(player).contains(&mv)
    }

    pub fn count_pieces(&self) -> (i32, i32) {
        let mut black_score = 0;
        let mut white_score = 0;

        for row in 0..self.size {
            for col in 0..self.size {
                match self.grid[row][col] {
                    Some(Player::Black) => black_score += 1,
                    Some(Player::White) => white_score += 1,
                    None => {}
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
                    Some(Player::White) => "W",
                    Some(Player::Black) => "B",
                    None => ".",
                };
                print!("{} ", symbol)
            }
            println!();
        }
    }
}
