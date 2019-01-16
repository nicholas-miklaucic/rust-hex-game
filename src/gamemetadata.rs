//! This file describes a struct to hold metadata about a game or set of games, loosely based on the
//! Smart Game Format.

/// Describes how and if a game ended: resignation, forfeit, or neither. Neither can mean either
/// direct loss or that the given game is a partial game.  If a resignation or forfeit, includes the
/// move on which the resignation or forfeit happened. This is numbered by move pair, not by
/// move. Thus, the 3rd move by Black is really the 5th move of play.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameEnding {
    /// The game did not finish or finished by direct loss.
    NotApplicable,
    /// Black resigned on the given move.
    BlackResignation(u8),
    /// Black forfeited on the given move (by time loss, disqualification, etc.)
    BlackForfeit(u8),
    /// White resigned on the given move.)
    WhiteResignation(u8),
    /// White forfeited on the given move (by time loss, disqualification, etc.)
    WhiteForfeit(u8),
}

/// A set of properties and metadata relating to games of Hex, including resigns, forfeits, piece or
/// color swaps, player names, and other notes.
#[derive(Clone, Debug)]
pub struct GameMetadata {
    /// Indicates whether White swapped colors on the second move. This crate does not handle the
    /// variant of Hex that has players swap pieces: it's equivalent to swapping colors and flipping
    /// on the long diagonal, and adding it unnecessarily complicates game serialization.
    pub swapped: bool,
    /// Black's name. Black is the player who makes the first move, or the player who decides to swap
    /// colors on the second move. Black tries to connect the left and right edges.
    pub black_name: String,
    /// White's name. White is the player who makes the second move, unless that player elects to swap
    /// colors. White tries to connect the top and bottom edges.
    pub white_name: String,
    /// Any comments on the game, as a string.
    pub comment: String,
    /// The year of the match, as an integer.
    pub year: u8,
    /// The month of the match, as an integer 1-12.
    pub month: u8,
    /// The day of the match, from 0 to 31.
    pub day: u8,
    /// Indicates how the game ended and if either player resigned or forfeited. The game will still
    /// be read in even if the board has a win for either player or if either player resigned or
    /// forfeited, to indicate possible future variations.
    pub ending: GameEnding,
}
