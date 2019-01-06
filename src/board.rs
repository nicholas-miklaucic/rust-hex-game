//! A struct describing a Hex board of a given size. For information about hex grids in computers,
//! look at the excellent [Red Blob Games](https://www.redblobgames.com/grids/hexagons/) guide on the
//! subject.
//!
//! Under the hood, this uses a union-find structure to keep track of the game status efficiently,
//! and stores pieces in sets.

use std::collections::HashSet;
use std::fmt;

use petgraph::unionfind::UnionFind;

use crate::coord::Coord;

/// One of the two possible colors in Hex.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Color {
    /// The left-right player that goes first.
    Black,
    /// The top-bottom player that goes second.
    White,
}

/// A simple descriptor of the possible values at a Hex tile: black piece, white piece, or empty.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum HexCell {
    /// A Black piece.
    Black,
    /// A White piece.
    White,
    /// An empty cell.
    Empty
}

/// A simple descriptor of the game status: ongoing, black victory, or white victory.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum GameStatus {
    /// Black wins.
    BlackWin,
    /// White wins.
    WhiteWin,
    /// The game does not yet have a winner.
    Ongoing
}

/// A Hex board of a given size, with pieces placed. The maximum size is 26, due to the limitations of
/// the standard Hex coordinate system.
#[derive(Clone, Debug)]
pub struct Board {
    /// The size of the board: both width and length. The size should be no larger than 26 due to the
    /// limitations of the Hex coordinate system. It's a u16 because, when mapping coordinates to
    /// numbers, it will be converted to a u16 enough to just make that the case here.
    pub size: u16,
    /// Black's pieces, as a union-find of coordinates mapped to integers. Black is the player
    /// connecting the left and right edges, and Black moves first. Note that each coordinate is
    /// shifted by one in each direction: the top and left edges are used for "virtual" stones.
    black_unions: UnionFind<u16>,
    /// White's pieces, as a union-find structure of coordinates mapped to integers. White is the
    /// player connecting the top and bottom edges, and White moves second. The White player can
    /// usually opt to swap with Black after the first move, but this doesn't affect the sequence of
    /// moves played. Note that each coordinate is shifted by one in each direction: the top and
    /// left edges are used for "virtual" stones.
    white_unions: UnionFind<u16>,
    /// Black's pieces, as a set of Coords.
    black: HashSet<Coord>,
    /// White's pieces, as a set of Coords.
    white: HashSet<Coord>,
    /// The current status of the board: black win, white win, or ongoing.
    status: GameStatus,
}

impl Default for Board {
    fn default() -> Self {
        // 13x13 is default size
        Board::new(13)
    }
}

impl Board {
    /// Initializes a blank board with given size less than or equal to 26.
    pub fn new(size: u16) -> Board {
        // additional 2 rows and columns for virtual stones
        let mut black_unions = UnionFind::new(((size + 2) * (size + 2)) as usize);
        let mut white_unions = UnionFind::new(((size + 2) * (size + 2)) as usize);
        // for each stone in the left and right columns, union together with the one above it
        for y in 1..size+1 {
            // corresponding to the left edge at the given height: (0, y) maps to (size + 2) * y
            // this connects (0, y) and (0, y - 1)
            black_unions.union(y * (size + 2), (y - 1) * (size + 2));
            // the right edge: (size - 1, y) maps to (size + 2) * (y + 1) - 1
            // this connects (size - 1, y) and (size - 1, y - 1)
            black_unions.union((y + 1) * (size + 2) - 1, y * (size + 2) - 1);
        }
        // for each stone in the top and bottom rows, union together with the one to the left of it
        for x in 1..size+1 {
            // corresponding to the top edge at the given x: (x, 0) maps to x
            // this connects (x, 0) and (x - 1, 0)
            white_unions.union(x, x - 1);
            // the bottom edge: (x, size - 1) maps to (size - 1) * size + x
            // this connects (x, size - 1) and (x - 1, size - 1)
            white_unions.union((size - 1) * size + x, (size - 1) * size + x -  1);
        }
        let black = HashSet::new();
        let white = HashSet::new();
        Board {
            size,
            black_unions,
            white_unions,
            black,
            white,
            status: GameStatus::Ongoing,
        }
    }
    /// Gets the integer value that maps to a given coordinate in this board size, reading in normal
    /// left-right top-down order. However, everything is shifted down and right by one, because there
    /// are virtual stones on the top and left edges. Basically, the size of the board is increased by
    /// 2 and each coordinate is shifted.
    fn coord_to_num(&self, coord: Coord) -> u16 {
        (coord.y as u16 + 1) * (self.size + 2) + coord.x as u16 + 1
    }
    /// Gets the Coordinate that maps to a given integer in this board size, reading in normal
    /// left-right top-down order. Accounts for everything being shifted down and right one to
    /// account for virtual stones. Basically, the size of the board is increased by 2 and each
    /// coordinate is shifted. This means that attempting to use this for numbers representing the
    /// top and left edges will give bad results or panic.
    fn num_to_coord(&self, num: u16) -> Coord {
        Coord {
            y: (num / (self.size + 2)) as u8 - 1,
            x: (num % (self.size + 2)) as u8 - 1,
        }
    }
    /// Determines whether the given number is black, white, or empty, including the virtual stones.
    fn piece_at_num(&self, num: u16) -> HexCell {
        // anything divisible by the real size or one before that is on the left or right edge and is black
        if num % (self.size + 2) == 0 || (num + 1) % (self.size + 2) == 0 {
            HexCell::Black
        } else if num <= self.size + 1 || num > (self.size + 1) * (self.size) {
            // anything below size + 1, or above (size + 1) * (size), is white
            HexCell::White
        } else {
            // now num_to_coord is guaranteed to work, just test the board as normal
            self.piece(self.num_to_coord(num))
        }
    }
    /// Gets the six numbers corresponding to the neighbors of a given integer when mapped to
    /// coordinates, including virtual stones. Has undefined behavior for the top left edge and may
    /// panic.
    fn num_neighbors(&self, num: u16) -> Vec<u16> {
        let size = self.size + 2; // to account for virtual stones
        vec![
            num - size, // top left
            num - size + 1, // top right
            num + 1, // right
            num + size, // bottom right
            num + size - 1, // bottom left
            num - 1, // left
        ]        
    }
    /// Places the piece at the given spot if the placement is valid (there are no other pieces and
    /// the coordinate is within range), modifying the board's state and returning true. Otherwise,
    /// does not modify the board state and returns false.
    pub fn place_piece(&mut self, coord: Coord, color: Color) -> bool {
        // if out of bounds, return false and do nothing
        if coord.x as u16 >= self.size || coord.y as u16 >= self.size {
            false
        } else if self.piece(coord) != HexCell::Empty {
            // if existing piece, return false and do nothing
            false
        } else {
            let num = self.coord_to_num(coord);
            match color {
                Color::Black => {
                    // add to set
                    self.black.insert(coord);
                    // now update union-find
                    for neighbor in self.num_neighbors(num) {
                        // if, in the union-find representation, this coordinate is black
                        if self.piece_at_num(neighbor) == HexCell::Black {
                            // union the two
                            self.black_unions.union(num, neighbor);
                        }
                    }
                }
                Color::White => {
                    // add to set
                    self.white.insert(coord);
                    // now update union-find
                    for neighbor in self.num_neighbors(num) {
                        // if, in the union-find representation, this coordinate is white
                        if self.piece_at_num(neighbor) == HexCell::White {
                            // union the two
                            self.white_unions.union(num, neighbor);
                        }
                    }
                }
            }
            // update game status
            self.set_game_status();
            true
        }        
    }
    /// Returns a `HexCell` value describing the piece at the given location: `Empty` if no piece is
    /// there, `Black` if Black has a piece, or `White` if White has a piece. If the coordinate is out
    /// of bounds, returns `Empty`.
    pub fn piece(&self, coord: Coord) -> HexCell {
        if self.black.contains(&coord) {
            HexCell::Black
        } else if self.white.contains(&coord) {
            HexCell::White
        } else {
            HexCell::Empty
        }
    }
    /// Checks for a winner, updating the game status if a change is required and returning whatever
    /// the game status is.
    fn set_game_status(&mut self) -> GameStatus {
        // if the squares one below the top left and right corners are equivalent, black has won,
        // because the left and right are connected
        if self.black_unions.find(self.size + 3) == self.black_unions.find((self.size + 2) * 2 - 1) {
            self.status = GameStatus::BlackWin;
            GameStatus::BlackWin        
        }
        // if the squares one to the right of the top and bottom left corners are connected, white
        // has won because the top and bottom are connected
        else if self.white_unions.find(1) == self.white_unions.find((self.size + 2) * (self.size + 1) + 1) {
            self.status = GameStatus::WhiteWin;
            GameStatus::WhiteWin
        } else {
            // game is still ongoing
            self.status = GameStatus::Ongoing;
            GameStatus::Ongoing
        }            
    }
    /// Returns the current game status. This is updated automatically as the game progresses, so this
    /// function has basically no runtime cost.
    pub fn status(&self) -> GameStatus {
        self.status
    }
}
    
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for y in 0..self.size {
            for x in 0..self.size {
                let c = Coord{x: x as u8, y: y as u8};
                if self.black.contains(&c) {
                    // add a black hexagon
                    s.push('⬢');          
                } else if self.white.contains(&c) {
                    // add a white hexagon
                    s.push('⬡');
                } else {
                    // add a placeholder dot
                    s.push('⋅');
                }
                // push a space, so that the next row can fit in between these pieces
                s.push(' ');
            }
            // separate with a newline and the right number of spaces
            s.push('\n');
            for _ in 0..y+1 {
                s.push(' ');
            }
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_display() {
        let mut board = Board::new(5);
        board.place_piece(Coord{x: 0, y: 0}, Color::Black);
        board.place_piece(Coord{x: 0, y: 1}, Color::Black);
        board.place_piece(Coord{x: 1, y: 1}, Color::Black);
        board.place_piece(Coord{x: 1, y: 2}, Color::Black);
        board.place_piece(Coord{x: 2, y: 2}, Color::Black);
        board.place_piece(Coord{x: 3, y: 1}, Color::Black);
        board.place_piece(Coord{x: 4, y: 0}, Color::Black);
        board.place_piece(Coord{x: 0, y: 2}, Color::White);
        board.place_piece(Coord{x: 2, y: 4}, Color::White);
        board.place_piece(Coord{x: 3, y: 0}, Color::White);
        board.place_piece(Coord{x: 4, y: 1}, Color::White);
        board.place_piece(Coord{x: 4, y: 3}, Color::White);
        println!();
        println!("{}", board);
    }

    #[test]
    fn test_game_status() {
        let mut board = Board::new(5);
        board.place_piece(Coord{x: 0, y: 0}, Color::Black);
        board.place_piece(Coord{x: 0, y: 2}, Color::White);
        board.place_piece(Coord{x: 0, y: 1}, Color::Black);
        board.place_piece(Coord{x: 2, y: 4}, Color::White);
        board.place_piece(Coord{x: 1, y: 1}, Color::Black);
        board.place_piece(Coord{x: 4, y: 1}, Color::White);
        board.place_piece(Coord{x: 1, y: 2}, Color::Black);
        board.place_piece(Coord{x: 3, y: 0}, Color::White);
        board.place_piece(Coord{x: 2, y: 2}, Color::Black);
        board.place_piece(Coord{x: 4, y: 3}, Color::White);
        board.place_piece(Coord{x: 3, y: 1}, Color::Black);
        assert_eq!(board.status, GameStatus::Ongoing);
        board.place_piece(Coord{x: 4, y: 0}, Color::Black);
        assert_eq!(board.status, GameStatus::BlackWin);

        let mut board2 = Board::new(5);
        board2.place_piece(Coord{x: 0, y: 0}, Color::White);
        board2.place_piece(Coord{x: 2, y: 0}, Color::Black);
        board2.place_piece(Coord{x: 1, y: 0}, Color::White);
        board2.place_piece(Coord{x: 4, y: 2}, Color::Black);
        board2.place_piece(Coord{x: 1, y: 1}, Color::White);
        board2.place_piece(Coord{x: 1, y: 4}, Color::Black);
        board2.place_piece(Coord{x: 2, y: 1}, Color::White);
        board2.place_piece(Coord{x: 0, y: 3}, Color::Black);
        board2.place_piece(Coord{x: 2, y: 2}, Color::White);
        board2.place_piece(Coord{x: 3, y: 4}, Color::Black);
        board2.place_piece(Coord{x: 1, y: 3}, Color::White);
        assert_eq!(board2.status, GameStatus::Ongoing);
        board2.place_piece(Coord{x: 0, y: 4}, Color::White);
        assert_eq!(board2.status, GameStatus::WhiteWin);
        println!();
        println!("{}", board2);
    }

    #[test]
    fn test_coord_num_conversion() {
        let board = Board::new(5);
        for x in 0..5 {
            for y in 0..5 {
                assert_eq!(board.num_to_coord(board.coord_to_num(Coord{x, y})), Coord{x, y});
            }
        }
    }
}
