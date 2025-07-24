mod board;
mod game;

use rand::seq::IndexedRandom;

use crate::{
    board::Cell,
    game::{Game, GameResult, GameState},
};

fn main() {
    let mut black_wins = 0;
    let mut white_wins = 0;
    let mut draws = 0;

    for _ in 0..1000 {
        let mut game = Game::new(8);

        while !game.is_over() {
            let legal_moves = game.board.get_legal_moves(game.turn);
            let chosen_move = legal_moves.choose(&mut rand::rng()).unwrap();

            game.play_move(*chosen_move);
        }

        // game.board.display();
        if let GameState::GameOver(result) = game.state {
            match result {
                GameResult::Winner(Cell::Black) => black_wins += 1,
                GameResult::Winner(Cell::White) => white_wins += 1,
                _ => draws += 1,
            }
        }
    }

    println!("After 1000 simulations:");
    println!("Black wins: {}", black_wins);
    println!("White wins: {}", white_wins);
    println!("Draws: {}", draws);
}
