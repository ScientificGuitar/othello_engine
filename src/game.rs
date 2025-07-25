use crate::board::{Board, Move, Player};

pub enum GameState {
    InProgress,
    Winner(Player),
    Draw,
}

pub struct Game {
    pub board: Board,
    pub current_player: Player,
    pub state: GameState,
}

impl Game {
    pub fn new(board_size: usize) -> Self {
        Game {
            board: Board::new(board_size),
            current_player: Player::Black,
            state: GameState::InProgress,
        }
    }

    pub fn play_move(&mut self, mv: Move) {
        match self.board.place_piece(self.current_player, mv) {
            Some(new_board) => {
                self.board = new_board;

                let black_moves = self.board.get_legal_moves(Player::Black);
                let white_moves = self.board.get_legal_moves(Player::White);

                if black_moves.is_empty() && white_moves.is_empty() {
                    let (black_count, white_count) = self.board.count_pieces();
                    if black_count > white_count {
                        self.state = GameState::Winner(Player::Black)
                    } else if white_count > black_count {
                        self.state = GameState::Winner(Player::White)
                    } else {
                        self.state = GameState::Draw
                    }
                    return;
                }

                self.switch_turn();
                let next_moves = self.board.get_legal_moves(self.current_player);
                if next_moves.is_empty() {
                    self.switch_turn();
                }
            }
            None => {
                println!("Illegal move for {:?}, {:?}!", self.current_player, mv)
            }
        }
    }

    fn switch_turn(&mut self) {
        self.current_player = self.current_player.opponent();
    }

    pub fn is_over(&self) -> bool {
        matches!(self.state, GameState::Winner(_) | GameState::Draw)
    }
}
