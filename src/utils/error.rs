use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct GameInfiniteLoop;
impl fmt::Display for GameInfiniteLoop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Game ended in an infinte loop")
    }
}
impl error::Error for GameInfiniteLoop {}

#[derive(Debug, Clone)]
pub struct IndexTooHigh;
impl fmt::Display for IndexTooHigh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Index too high")
    }
}
impl error::Error for IndexTooHigh {}
