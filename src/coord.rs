//! A coordinate system suitable for Hex games. The standard setup is a parallelogram, with letters
//! a-z representing columns from left to right, and numbers 1-26 representing rows from top to
//! bottom. In the most common orientation, the top left corner is farther left than the bottom left
//! corner, and so the longest diagonal runs from the top left to bottom right.
//! 
//! These coordinates are optimized for Hex, a game that is not usually played on boards larger than
//! 26x26. Thus, these coordinates do not work for higher board sizes, as it breaks the string
//! representations and integer arithmetic.

use std::ops::Add;
use std::error;
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

/// The alphabet used for representing coordinates, in lowercase.
static ALPHABET: &str = "abcdefghjiklmnopqrstuvwxyz";

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
/// A coordinate on a Hex board, such that (x, y) = (0, 0) is the top left and (1, 0) is the hex
/// immediately to the right of that hex. Cannot support boards larger than 128x128 for performance
/// reasons.
pub struct Coord {
    /// The x-axis, starting from the left column at 0.
    pub x: u8,
    /// The y-axis, starting from the top row at 0.
    pub y: u8,
}

impl Add<Coord> for Coord {
    type Output = Coord;

    /// Adds componentwise, but does not check the addition.
    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", ALPHABET.chars().nth(self.x as usize).unwrap(), self.y + 1)
    }
}

#[derive(Debug, Clone)]
/// An error for parsing a `Coord`.
pub enum ParseCoordError {
    InvalidFormat,
    InvalidInt(ParseIntError)
}

impl fmt::Display for ParseCoordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseCoordError::InvalidFormat => write!(f, "invalid coordinate string"),
            ParseCoordError::InvalidInt(ref e) => e.fmt(f),
        }
    }
}

// This is important for other errors to wrap this one.
impl error::Error for ParseCoordError {
    fn description(&self) -> &str {
        match *self {
            ParseCoordError::InvalidFormat => "invalid coordinate string format",
            ParseCoordError::InvalidInt(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ParseCoordError::InvalidFormat => None,
            ParseCoordError::InvalidInt(ref e) => Some(e)
        }
    }
}

impl From<ParseIntError> for ParseCoordError {
    fn from(err: ParseIntError) -> ParseCoordError {
        ParseCoordError::InvalidInt(err)
    }
}

impl FromStr for Coord {
    type Err = ParseCoordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_lower = s.to_lowercase();
        if s_lower.len() < 2 {
            Err(ParseCoordError::InvalidFormat)
        }
        else if !ALPHABET.contains(s_lower.chars().nth(0).unwrap()) {
            Err(ParseCoordError::InvalidFormat)
        } else {
            let x: u8 = ALPHABET.find(|x| x == s_lower.chars().nth(0).unwrap()).unwrap() as u8;
            let y: u8 = s_lower.chars().skip(1).collect::<String>().parse()?;            
            match Coord::new(x, y - 1) {
                Some(c) => Ok(c),
                None => Err(ParseCoordError::InvalidFormat)
            }
        }
    }
}

impl Coord {
    /// Creates a new `Coord`, returning `None` if either x or y exceed 25. 
    pub fn new(x: u8, y: u8) -> Option<Coord> {
        if x > 25 || y > 25 {
            Option::None
        } else {
            Option::Some(Coord{x, y})
        }
    }
    /// Returns an iterator representing each of this hex's neighbors, clockwise from the top left. If
    /// this hex is on the first row or column, will return fewer than six coordinates.
    pub fn neighbors(self) -> Vec<Coord> {
        if self == Coord::default() {
            // hard-code: special case
            return vec![
                Coord{x: 1, y: 0},
                Coord{x: 0, y: 1},
            ]
        }
        else if self.x == 0 {
            // on the left edge but not the left corner: four neighbors
            return vec![
                Coord{x: 0, y: self.y - 1},
                Coord{x: 1, y: self.y - 1},
                Coord{x: 1, y: self.y},
                Coord{x: 0, y: self.y + 1}
            ]
        }
        else if self.y == 0 {
            // on the top edge but not the top corner: four neighbors
            return vec![
                Coord{x: self.x + 1, y: 0},
                Coord{x: self.x, y: 1},
                Coord{x: self.x - 1, y: 1},
                Coord{x: self.x - 1, y: 0}
            ]
        }
        else {
            // return the six required vectors in order
            vec![
                // top left: (0, -1)
                Coord{x: self.x, y: self.y - 1},
                // top right: (1, -1)
                Coord{x: self.x + 1, y: self.y - 1},
                // right: (1, 0)
                Coord{x: self.x + 1, y: self.y},
                // bottom right: (0, 1)
                Coord{x: self.x, y: self.y + 1},
                // bottom left: (-1, 1)
                Coord{x: self.x - 1, y: self.y + 1},
                // left: (-1, 0)
                Coord{x: self.x - 1, y: self.y},
            ]
        }
    }
    /// Returns true if the two hexes neighbor each other or equal each other, and false otherwise.
    pub fn is_neighbor(self, other: Coord) -> bool {
        (Coord::abs_sub(self.x, other.x) <= 1 &&
         Coord::abs_sub(self.y, other.y) <= 1 &&
         Coord::abs_sub(self.x + self.y, other.x + other.y) <= 1)
    }
    /// Gets the absolute difference between two unsigned u8s, in a way that avoids overflow.
    pub fn abs_sub(int1: u8, int2: u8) -> u8 {
        if int1 >= int2 {
            int1 - int2
        } else {
            int2 - int1
        }
    }
    /// Gets the distance between two coordinates, defined as the number of steps in the grid needed
    /// to connect them. A distance of 0 means equality, and a distance of 1 means the two are
    /// neighboring.
    pub fn distance(self, other: Coord) -> u8 {
        // This is computed by realizing that this is really two axes of a 3D coordinate system
        // representing the plane x + y + z = 0. Thus, what we do is just Manhattan distance in that
        (Coord::abs_sub(self.x, other.x) + Coord::abs_sub(self.y, other.y) +
         Coord::abs_sub(self.x + self.y, other.x + other.y)) / 2
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_constructor() {
        assert_eq!(Coord::new(14, 15).unwrap(), Coord{x: 14, y: 15});
        assert_eq!(Coord::new(14, 26), None);
    }
    #[test]
    fn test_neighbors() {
        assert_eq!(Coord{x: 0, y: 0}.neighbors(),
                   vec![Coord{x: 1, y: 0},
                        Coord{x: 0, y: 1}]);
        assert_eq!(Coord{x: 0, y: 5}.neighbors(),
                   vec![Coord{x: 0, y: 4},
                        Coord{x: 1, y: 4},
                        Coord{x: 1, y: 5},
                        Coord{x: 0, y: 6}]);
        assert_eq!(Coord{x: 4, y: 0}.neighbors(),
                   vec![Coord{x: 5, y: 0},
                        Coord{x: 4, y: 1},
                        Coord{x: 3, y: 1},
                        Coord{x: 3, y: 0}]);
        assert_eq!(Coord{x: 7, y: 5}.neighbors(),
                   vec![Coord{x: 7, y: 4},
                        Coord{x: 8, y: 4},
                        Coord{x: 8, y: 5},
                        Coord{x: 7, y: 6},
                        Coord{x: 6, y: 6},
                        Coord{x: 6, y: 5}]);
    }
    #[test]
    fn test_is_neighbor() {
        assert!(Coord{x: 0, y: 0}.is_neighbor(Coord{x: 0, y: 1}));
        assert!(Coord{x: 0, y: 0}.is_neighbor(Coord{x: 1, y: 0}));
        assert!(Coord{x: 3, y: 0}.is_neighbor(Coord{x: 2, y: 0}));
        assert!(Coord{x: 0, y: 5}.is_neighbor(Coord{x: 0, y: 4}));
        assert!(Coord{x: 3, y: 0}.is_neighbor(Coord{x: 2, y: 1}));
        assert!(Coord{x: 0, y: 5}.is_neighbor(Coord{x: 1, y: 4}));
        assert!(Coord{x: 6, y: 7}.is_neighbor(Coord{x: 6, y: 7}));

        assert!(!Coord{x: 0, y: 0}.is_neighbor(Coord{x: 1, y: 1}));
        assert!(!Coord{x: 0, y: 0}.is_neighbor(Coord{x: 12, y: 10}));
        assert!(!Coord{x: 3, y: 0}.is_neighbor(Coord{x: 2, y: 2}));
    }
    #[test]
    fn test_distance() {
        assert_eq!(Coord{x: 0, y: 0}.distance(Coord{x: 1, y: 1}), 2);
        assert_eq!(Coord{x: 4, y: 3}.distance(Coord{x: 1, y: 1}), 5);
        assert_eq!(Coord{x: 4, y: 3}.distance(Coord{x: 4, y: 2}), 1);
        assert_eq!(Coord{x: 4, y: 3}.distance(Coord{x: 4, y: 3}), 0);
    }

    #[test]
    fn test_display() {
        assert_eq!(&Coord{x: 0, y: 0}.to_string(), "a1");
        assert_eq!(&Coord{x: 13, y: 0}.to_string(), "n1");
        assert_eq!(&Coord{x: 13, y: 5}.to_string(), "n6");
        assert_eq!(&Coord{x: 25, y: 25}.to_string(), "z26");
        assert_eq!(&Coord{x: 25, y: 0}.to_string(), "z1");
    }

    #[test]
    fn test_parsing() {
        for string in vec!["a1", "a16", "B6", "z6", "z23", "Q25"].iter() {
            assert_eq!(string.to_lowercase(), Coord::from_str(string).unwrap().to_string());
        }
        assert!(Coord::from_str("ZZ").is_err());
        assert!(Coord::from_str("Z126").is_err());
    }
}
