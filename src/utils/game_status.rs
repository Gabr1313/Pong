#[derive(Debug, PartialEq, Eq)]
pub enum GameStatus {
    End,
    Play,
    Quit,
    Waiting,
    Reset,
}
