mod board;
mod game;

use rand::seq::IndexedRandom;

use crate::{
    board::Player,
    game::{Game, GameState},
};

fn main() {
    let mut black_wins = 0;
    let mut white_wins = 0;
    let mut draws = 0;

    for _ in 0..1000 {
        let mut game = Game::new(8);

        while !game.is_over() {
            let legal_moves = game.board.get_legal_moves(game.current_player);
            let mv = legal_moves.choose(&mut rand::rng()).unwrap();

            game.play_move(*mv);
        }

        match game.state {
            GameState::Winner(Player::Black) => black_wins += 1,
            GameState::Winner(Player::White) => white_wins += 1,
            GameState::Draw => draws += 1,
            _ => {}
        }
    }

    println!("After 1000 simulations:");
    println!("Black wins: {}", black_wins);
    println!("White wins: {}", white_wins);
    println!("Draws: {}", draws);
}
