use super::{Battlesnake, Board, Game};

/// Your Battlesnake will receive this request when it has been entered into
/// a new game. Every game has a unique ID that can be used to allocated
/// resources or data you may need. Your response to this request will be
/// ignored.
pub struct Start {
    /// Game Object describing the game being played.
    game: Game,
    /// Turn number of the game being played (0 for new games).
    turn: usize,
    /// Board Object describing the initial state of the game board.
    board: Board,
    /// Battlesnake Object describing your Battlesnake.
    you: Battlesnake,
}

/// This request will be sent for every turn of the game. Use the
/// information provided to determine how your Battlesnake will move on
/// that turn, either up, down, left, or right.
pub struct Move {
    /// Game Object describing the game being played.
    game: Game,
    /// Turn number of the game being played (0 for new games).
    turn: usize,
}

pub struct End {

}
