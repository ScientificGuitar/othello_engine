mod board;
mod game;

use rand::seq::IndexedRandom;
use std::cmp;

use crate::{
    board::{Move, Player},
    game::{Game, GameState},
};

fn main() {
    let mut black_wins = 0;
    let mut white_wins = 0;
    let mut draws = 0;

    for _ in 0..10 {
        let mut game = Game::new(4);

        while !game.is_over() {
            let legal_moves = game.board.get_legal_moves(game.current_player);

            let mut mv = *legal_moves.choose(&mut rand::rng()).unwrap();

            if game.current_player == Player::White {
                if let Some(best_move) = find_best_move(&game, 20) {
                    mv = best_move;
                } else {
                    println!(
                        "Something went wrong... no legal moves available for {:?}",
                        Player::Black
                    );
                }
            }
            game = game.play_move(mv);
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

fn find_best_move(game: &Game, depth: i32) -> Option<Move> {
    let mut best_eval = i32::MIN;
    let mut best_move = None;
    let legal_moves = game.board.get_legal_moves(game.current_player);

    for mv in legal_moves {
        let new_game = game.play_move(mv);
        let eval = -minmax(new_game, depth - 1);
        if eval > best_eval {
            best_eval = eval;
            best_move = Some(mv);
        }
    }

    best_move
}

fn minmax(game: Game, depth: i32) -> i32 {
    if depth == 0 || game.is_over() {
        return game.evaluate();
    }

    let mut best_eval = i32::MIN;
    let legal_moves = game.board.get_legal_moves(game.current_player);
    for mv in legal_moves {
        let new_game = game.play_move(mv);
        let eval = -minmax(new_game, depth - 1);
        best_eval = cmp::max(best_eval, eval)
    }

    best_eval
}

// Benchmarks on 8x8 board:     B      D     W
// Random moves vs Random:      4525 - 419 - 5056
// Minmax depth 1 vs Random:    5795 - 389 - 3816
// Minmax depth 2 vs Random:     727 -  23 -  250
// Minmax depth 3 vs Random:      80 -   2 -   18
