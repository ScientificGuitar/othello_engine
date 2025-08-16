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
                game.switch_turn();
                game.update_game_state();
                return game;
            }
            None => {
                println!("Illegal move for {:?}, {:?}!", game.current_player, mv);
                return game;
            }
        }
    }

    pub fn pass_turn(&self) -> Game {
        let mut game = self.clone();
        game.switch_turn();
        game.update_game_state();
        game
    }

    fn switch_turn(&mut self) {
        self.current_player = self.current_player.opponent();
    }

    pub fn is_over(&self) -> bool {
        matches!(self.state, GameState::Winner(_) | GameState::Draw)
    }

    fn update_game_state(&mut self) {
        let black_moves = self.board.get_legal_moves(Player::Black);
        let white_moves = self.board.get_legal_moves(Player::White);

        if black_moves.is_empty() && white_moves.is_empty() {
            let (black_count, white_count) = self.board.count_pieces();
            self.state = if black_count > white_count {
                GameState::Winner(Player::Black)
            } else if white_count > black_count {
                GameState::Winner(Player::White)
            } else {
                GameState::Draw
            };
        }
    }

    pub fn evaluate(&self, maximizing_player: Player) -> i32 {
        match self.state {
            GameState::Winner(p) => {
                if p == self.current_player {
                    1000
                } else {
                    -1000
                }
            }
            GameState::Draw => 0,
            GameState::InProgress => {
                let (black, white) = self.board.count_pieces();
                match self.current_player {
                    Player::Black => black - white,
                    Player::White => white - black,
                }
            }
        }
    }
}
