#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameStatus {
    Neutral,
    Play,
    Quit,
    Waiting,
    Reset,
}
