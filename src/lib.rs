extern crate colored;
extern crate petgraph;

pub mod coord;
pub mod board;
pub mod game;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
