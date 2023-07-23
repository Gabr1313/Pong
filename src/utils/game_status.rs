#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameStatus {
    End,
    Play,
    Quit,
    Waiting,
    Reset,
}
