//! This file's `Game` struct represents a game of Hex.

use std::fmt;

use colored::Colorize;

use crate::board::{Board, Color, GameStatus, HexCell};
use crate::coord::Coord;

/// A game of Hex, with move history. Metadata about the game (players, ratings, etc.) comes from a
/// `GameMetadata` struct: this simply captures the actual moves and whether the players swapped.
///
/// Although colors differ between Hex implementations, this crate consistently has Black as the
/// player that moves first and who tries to connect the left and right sides, on a game board where
/// the top and bottom edges are flat and the top edge is to the left of the bottom edge.
#[derive(Clone, Debug)]
pub struct Game {
    /// The number of hexes on one edge of the board. This crate does not support hex games larger
    /// than 26x26.
    pub board_size: u8,
    /// A list of moves, such that Black goes first and on every other odd-numbered move, and White
    /// goes on every even-numbered moves. If the White player swaps on their first move, that should
    /// be indicated by the game's metadata: in `GameMetadata`, Black and White are the players as
    /// they were at the end of the game, not as at the beginning.
    ///
    /// If the list of moves is invalid for whatever reason (out-of-bounds coordinates, playing to the
    /// same square twice, etc.), undefined behavior, including possible panics, can result.
    pub moves: Vec<Coord>,
    /// The current board, given the above moves.
    board: Board,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // uses a numbering system, as is common in Hex
        let mut numbered_board = vec![];
        // initialize board as blank
        for _x in 0..self.board_size {
            for _y in 0..self.board_size {
                // push two dots so that you have enough room for 99 moves
                numbered_board.push("⋅⋅".to_string());
            }
        }
        // now go through each move and modify the corresponding number
        let mut curr_num = 1;  // first move is numbered 1, not 0
        for coord in &self.moves {
            // get index in board
            let index = coord.y * self.board_size + coord.x;
            // pad to 2 digits and write with correct color
            if curr_num % 2 == 0 {
                // White to move
                numbered_board[index as usize] = format!("{:0>2}", &curr_num.to_string().bold().black().on_bright_white());
            } else {
                // Black to move
                numbered_board[index as usize] = format!("{:0>2}", &curr_num.to_string().bold().bright_white().on_black());
            }
            curr_num += 1;
        }
        let mut output_string = String::new();
        for y in 0..self.board_size {
            for x in 0..self.board_size {
                let index = y * self.board_size + x;
                // push two dots so that you have enough room for 99 moves
                output_string.push_str(&numbered_board[index as usize]);
                // add two spaces
                output_string.push(' ');
                output_string.push(' ');
            }
            // separate with two newlines and the right number of spaces
            output_string.push('\n');
            output_string.push('\n');
            for _ in 0..y+1 {
                output_string.push(' ');
                output_string.push(' ');
            }
        }
        write!(f, "{}", output_string)
    }
}

impl Default for Game {
    fn default() -> Game {
        // 13 is default size
        Game::new(13)
    }
}

impl Game {
    /// Returns a new Game of the given size.
    pub fn new(size: u8) -> Game {
        Game {
            board_size: size,
            board: Board::new(size as u16),
            moves: vec![]
        }
    }
    /// Returns the current game's status. As this is updated on each move and stored, this function
    /// incurs almost no runtime cost.
    pub fn status(&self) -> GameStatus {
        self.board.status()
    }
    /// Returns the color of the player next to move.
    pub fn next_move_color(&self) -> Color {
        if self.moves.len() % 2 == 0 {
            // if even number of moves, last move was White, so next move is Black
            Color::Black
        } else {
            // otherwise, White to play
            Color::White
        }
    }
    /// Makes the next move of the game, using whichever color is next to play. If the given
    /// coordinate is invalid (it already has a piece or is out of bounds), returns `false` and does
    /// nothing. Otherwise, returns `true`.
    pub fn make_move(&mut self, coord: Coord) -> bool {
        if self.board.place_piece(coord, self.next_move_color()) {
            // move is valid, add to moves list and return true
            self.moves.push(coord);
            true
        } else {
            // move is invalid, do nothing and return false
            false
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_display() {
        let mut g = Game::new(7);
        g.make_move(Coord{x: 1, y: 3});
        g.make_move(Coord{x: 2, y: 0});
        g.make_move(Coord{x: 4, y: 1});
        g.make_move(Coord{x: 3, y: 4});
        g.make_move(Coord{x: 0, y: 5});
        g.make_move(Coord{x: 2, y: 6});
        g.make_move(Coord{x: 5, y: 2});
        g.make_move(Coord{x: 6, y: 1});
        g.make_move(Coord{x: 2, y: 1});
        g.make_move(Coord{x: 2, y: 2});
        g.make_move(Coord{x: 4, y: 4});
        g.make_move(Coord{x: 6, y: 6});
        println!();
        println!("{}", g);
    }
}
