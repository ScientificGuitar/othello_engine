use crate::board::{Board, Move, Player};

#[derive(Clone, PartialEq)]
pub enum GameState {
    InProgress,
    Winner(Player),
    Draw,
}

#[derive(Clone)]
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

    pub fn play_move(&self, mv: Move) -> Game {
        let mut game = self.clone();
        match game.board.place_piece(game.current_player, mv) {
            Some(new_board) => {
                game.board = new_board;

                let black_moves = game.board.get_legal_moves(Player::Black);
                let white_moves = game.board.get_legal_moves(Player::White);

                if black_moves.is_empty() && white_moves.is_empty() {
                    let (black_count, white_count) = game.board.count_pieces();
                    if black_count > white_count {
                        game.state = GameState::Winner(Player::Black)
                    } else if white_count > black_count {
                        game.state = GameState::Winner(Player::White)
                    } else {
                        game.state = GameState::Draw
                    }
                    return game;
                }

                game.switch_turn();
                let next_moves = game.board.get_legal_moves(game.current_player);
                if next_moves.is_empty() {
                    game.switch_turn();
                }
                return game;
            }
            None => {
                println!("Illegal move for {:?}, {:?}!", game.current_player, mv);
                return game;
            }
        }
    }

    fn switch_turn(&mut self) {
        self.current_player = self.current_player.opponent();
    }

    pub fn is_over(&self) -> bool {
        matches!(self.state, GameState::Winner(_) | GameState::Draw)
    }

    pub fn evaluate(&self) -> i32 {
        if self.state == GameState::Winner(self.current_player) {
            return 1000;
        }
        if self.state == GameState::Winner(self.current_player.opponent()) {
            return -1000;
        }

        let (black, white) = self.board.count_pieces();

        match self.current_player {
            Player::Black => black - white,
            Player::White => white - black,
        }
    }
}
