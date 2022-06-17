pub mod request;
pub mod response;

/// A 24-bit rgb color code
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Default for Color {
    fn default() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
        }
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color{ r, g, b }
    }
}

pub enum RequestType {
    Start,
    Move,
    End
}

pub struct Request {
    /// Game Object describing the game being played.
    game: Game,
    /// Turn number of the game being played (0 for new games).
    turn: usize,
    /// Board Object describing the game board on this turn
    board: Board,
    /// Battlesnake Object describing your Battlesnake.
    you: Battlesnake,
}

pub enum Head {
    Default,
    Beluga,
    Bendr,
    Dead,
    Evil,
    Fang,
    Pixel,
    Safe,
    Sandworm,
    Shades,
    Silly,
    Smile,
    Tounge,
    Bowler,
    Mark,
    AllSeeing,
    SmartCaterpillar,
    TransRights,
    Bonhomme,
    Earmuffs,
    Rudolph,
    Scarf,
    Ski,
    Snowman,
    Snowworm,
    Caffine,
    Gamer,
    TigerKing,
    Workout
}

impl TryFrom<&str> for Head {
    type Error = &'static str;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        match name {
            "default"            => Ok(Head::Default),
            "beluga"             => Ok(Head::Beluga),
            "bendr"              => Ok(Head::Bendr),
            "dead"               => Ok(Head::Dead),
            "evil"               => Ok(Head::Evil),
            "fang"               => Ok(Head::Fang),
            "pixel"              => Ok(Head::Pixel),
            "safe"               => Ok(Head::Safe),
            "sand-worm"          => Ok(Head::Sandworm),
            "shades"             => Ok(Head::Shades),
            "silly"              => Ok(Head::Silly),
            "smile"              => Ok(Head::Smile),
            "tounge"             => Ok(Head::Tounge),
            "rbc-bowler"         => Ok(Head::Bowler),
            "replit-mark"        => Ok(Head::Mark),
            "all-seeing"         => Ok(Head::AllSeeing),
            "smart-caterpillar"  => Ok(Head::SmartCaterpillar),
            "trans-rights-scarf" => Ok(Head::TransRights),
            "bonhomme"           => Ok(Head::Bonhomme),
            "earmuffs"           => Ok(Head::Earmuffs),
            "rudolph"            => Ok(Head::Rudolph),
            "scarf"              => Ok(Head::Scarf),
            "ski"                => Ok(Head::Ski),
            "snowman"            => Ok(Head::Snowman),
            "snow-worm"          => Ok(Head::Snowworm),
            "caffine"            => Ok(Head::Caffine),
            "gamer"              => Ok(Head::Gamer),
            "tiger-king"         => Ok(Head::TigerKing),
            "workout"            => Ok(Head::Workout),
            _ => Err("Could not match name"),
        }
    }
}

impl Head {
    fn name(&self) -> &'static str {
        match self {
            Head::Default          => "default",
            Head::Beluga           => "beluga",
            Head::Bendr            => "bendr",
            Head::Dead             => "dead",
            Head::Evil             => "evil",
            Head::Fang             => "fang",
            Head::Pixel            => "pixel",
            Head::Safe             => "safe",
            Head::Sandworm         => "sand-worm",
            Head::Shades           => "shades",
            Head::Silly            => "silly",
            Head::Smile            => "smile",
            Head::Tounge           => "tounge",
            Head::Bowler           => "rbc-bowler",
            Head::Mark             => "replit-mark",
            Head::AllSeeing        => "all-seeing",
            Head::SmartCaterpillar => "smart-caterpillar",
            Head::TransRights      => "trans-rights-scarf",
            Head::Bonhomme         => "bonhomme",
            Head::Earmuffs         => "earmuffs",
            Head::Rudolph          => "rudolph",
            Head::Scarf            => "scarf",
            Head::Ski              => "ski",
            Head::Snowman          => "snowman",
            Head::Snowworm         => "snow-worm",
            Head::Caffine          => "caffine",
            Head::Gamer            => "gamer",
            Head::TigerKing        => "tiger-king",
            Head::Workout          => "workout",
        }
    }
}

pub struct Game {
    /// A unique identifier for this Game.
    /// Example: "totally-unique-game-id"
    id: String,
    /// Information about the ruleset being used to run this game.
    /// Example: {"name": "standard", "version": "v1.2.3"}
    ruleset: Ruleset,
    /// The name of the map used to populate the game board with snakes, food,
    /// and hazards.
    /// Example: "standard"
    map: String,
    /// How much time your snake has to respond to requests for this Game.
    /// Example: 500
    timeout: usize,
    /// The source of this game. One of:
    /// * tournament
    /// * league (for League Arenas)
    /// * arena (for all other Arenas)
    /// * challenge
    /// * custom (for all other game sources)
    /// The values for this field may change in the near future
    source: String,
}

pub struct Ruleset {
    /// Name of the ruleset being used to run this game. Possible values include:
    /// standard, solo, royale, squad, constrictor, wrapped. See Game Modes for
    /// more information on each ruleset
    /// Example: "standard"
    name: String,
    /// The release version of the Rules modules used in this game
    /// Example: "version": "v1.2.3"
    version: String,
    /// A collection of specific settings being used by the current game that
    /// control how the rules are applied.
    settings: RulesetSettings
}

pub struct RulesetSettings {
    /// Percentage chance of spawning a new food every round.
    foodSpawnChance: i32,
    /// Minimum food to keep on the board every turn.
    minimumFood: i32,
    /// Health damage a snake will take when ending its turn in a hazard. This
    /// stacks on top of the regular 1 damage a snake takes per turn.
    hazardDamagePerTurn: i32,
    royale: RoyaleRulesetSettings,
    squard: SquadRulesetSetting,
}

pub struct RoyaleRulesetSettings {
    /// In Royale mode, the number of turns between generating new hazards
    /// (shrinking the safe board space).
    shrinkEveryNTurns: usize,
}

pub struct SquadRulesetSetting {
    /// In Squad mode, allow members of the same squad to move over each other
    /// without dying.
    allowBodyCollisions: bool,
    /// In Squad mode, all squad members are eliminated when one is eliminated.
    sharedElimination: bool,
    /// In Squad mode, all squad members share health.
    sharedHealth: bool,
    /// In Squad mode, all squad members share length.
    sharedLength: bool,
}

pub struct Battlesnake {
    /// Unique identifier for this Battlesnake in the context of the current Game.
    /// Example: "totally-unique-snake-id"
    id: String,
    /// Name given to this Battlesnake by its author.
    /// Example: "Sneky McSnek Face"
    name: String,
    /// Health value of this Battlesnake, between 0 and 100 inclusively.
    /// Example: 54
    health: u8,
    /// Array of coordinates representing this Battlesnake's location on the
    /// game board. This array is ordered from head to tail.
    /// Example: [{"x": 0, "y": 0}, ..., {"x": 2, "y": 0}]
    body: Vec<Point>,
    /// The previous response time of this Battlesnake, in milliseconds. "0"
    /// means the Battlesnake timed out and failed to respond.
    /// Example: "450"
    latency: String,
    /// Coordinates for this Battlesnake's head. Equivalent to the first
    /// element of the body array.
    /// Example: {"x": 0, "y": 0}
    head: Point,
    /// Length of this Battlesnake from head to tail. Equivalent to the length
    /// of the body array.
    /// Example: 3
    length: usize,
    /// Message shouted by this Battlesnake on the previous turn.
    /// Example: "why are we shouting??"
    shout: String,
    /// The squad that the Battlesnake belongs to. Used to identify squad
    /// members in Squad Mode games.
    /// Example: "1"
    squad: String,
    /// The collection of customizations applied to this Battlesnake that
    /// represent how it is viewed. Follows the same rules as in the GET
    /// request.
    /// Example: {"color":"#888888", "head":"default", "tail":"default" }
    customizations: BattlesnakeCustomizations,
}

pub struct BattlesnakeCustomizations {
    color: Color,
    head: Head,
    tail: usize,
}

pub struct Point {
    x: u8,
    y: u8,
}

/// The game board is represented by a standard 2D grid, oriented with (0,0) in
/// the bottom left. The Y-Axis is positive in the up direction, and X-Axis is
/// positive to the right. Coordinates begin at zero, such that a board that is
/// 11x11 will have coordinates ranging from [0, 10].
pub struct Board {
    /// The number of rows in the y-axis of the game board.
    /// Example: 11
    height: usize,
    /// The number of columns in the x-axis of the game board.
    /// Example: 11
    width: usize,
    /// Array of coordinates representing food locations on the game board.
    /// Example: [{"x": 5, "y": 5}, ..., {"x": 2, "y": 6}]
    food: Vec<Point>,
    /// Array of coordinates representing hazardous locations on the game board.
    /// These will only appear in some game modes.
    /// Example: [{"x": 0, "y": 0}, ..., {"x": 0, "y": 1}]
    hazards: Vec<Point>,
    /// Array of Battlesnake Objects representing all Battlesnakes remaining on
    /// the game board (including yourself if you haven't been eliminated).
    /// Example: [{"id": "snake-one", ...}, ...]
    snakes: Vec<Battlesnake>,
}

/// Game maps are defined in the BattlesnakeOfficial/rules repo, inside the
/// maps package. Known maps currently include:
pub enum Map {

}
