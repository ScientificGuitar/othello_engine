use crate::board::{Board, Cell, Position};

pub enum GameResult {
    Draw,
    Winner(Cell),
}

pub enum GameState {
    WaitingForMove,
    GameOver(GameResult),
}

pub struct Game {
    pub board: Board,
    pub turn: Cell,
    pub state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            turn: Cell::Black,
            state: GameState::WaitingForMove,
        }
    }

    pub fn play_move(&mut self, mv: Position) {
        if let GameState::GameOver(_) = self.state {
            println!("Game is over. No more moves allowed.");
            return;
        }

        let legal_moves = self.board.get_legal_moves(self.turn);

        if !legal_moves.contains(&mv) {
            println!("Illegal move!");
            return;
        }

        self.board = self.board.place_piece(self.turn, mv);

        let black_moves = self.board.get_legal_moves(Cell::Black);
        let white_moves = self.board.get_legal_moves(Cell::White);

        if black_moves.is_empty() && white_moves.is_empty() {
            let (black_count, white_count) = self.board.count_pieces();
            self.state = GameState::GameOver(if black_count > white_count {
                GameResult::Winner(Cell::Black)
            } else if white_count > black_count {
                GameResult::Winner(Cell::White)
            } else {
                GameResult::Draw
            });
            return;
        }

        self.switch_turn();
        let next_moves = self.board.get_legal_moves(self.turn);
        if next_moves.is_empty() {
            self.switch_turn();
        }

        self.state = GameState::WaitingForMove;
    }

    fn switch_turn(&mut self) {
        self.turn = match self.turn {
            Cell::White => Cell::Black,
            Cell::Black => Cell::White,
            _ => Cell::Empty,
        };
    }

    pub fn is_over(&self) -> bool {
        matches!(self.state, GameState::GameOver(_))
    }
}
