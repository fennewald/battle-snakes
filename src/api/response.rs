use crate::{api::{Color, Head}, Direction};

pub struct Root {
    /// Version of the Battlesnake API implemented by this Battlesnake.
    /// Currently only API version 1 is valid.
    apiversion: String,
    /// Username of the author of this Battlesnake. If provided, this will be
    /// used to verify owernship.
    author: Option<String>,
    /// Hex color code used to display this Battlesnake. Must start with "#"
    /// and be 7 characters long.
    color: Option<Color>,
    /// Displayed head of this battlesnake. See Personalization Docs for
    /// available options
    head: Option<Head>,
    /// Displayed tail of this Battlesnake. See Personalization Docs for
    /// available options
    tail: Option<usize>,
    /// A version number or tag for your snake.
    version: Option<String>,
}

pub struct Move {
    /// Your Battlesnake's move for this turn. Valid moves are up, down, left,
    /// or right.
    r#move: Direction,
    /// An optional message sent to all other Battlesnakes on the next turn.
    /// Must be 256 characters or less.
    shout: Option<String>,
}
