#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_more;

use std::ops::{Deref, DerefMut, Index, IndexMut};

use uuid::Uuid;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Serialize, Deserialize)]
#[display(fmt = "{}", _0)]
pub struct GameId(Uuid);

impl GameId {
    pub fn new() -> Self {
        GameId(Uuid::new_v4())
    }

    pub fn from_str(val: &str) -> Option<Self> {
        Some(GameId(Uuid::parse_str(val).ok()?))
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Serialize, Deserialize)]
#[display(fmt = "{}", _0)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        UserId(Uuid::new_v4())
    }

    pub fn from_str(val: &str) -> Option<Self> {
        Some(UserId(Uuid::parse_str(val).ok()?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreGameClientMsg {
    NewGame,
    JoinGame(GameId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreGameServerMsg {
    UserId(UserId),
    Joined(GameId),
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RegistrationClientMsg {
    Register(Register),
}

#[derive(Debug, From, Clone, PartialEq, Serialize, Deserialize)]
pub enum RegistrationServerMsg {
    RegistrationStatus(RegistrationStatus),
    UserJoined(UserId),
    Registration(CharacterRegistration),
    #[from(ignore)]
    Complete,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameServerMessage {
    Witness(WitnessValue),
    Position(PlayerPosition),
    PlayerTurn,
    Suggestion(Suggestion),
    SuggestionQuery,
    SuggestionStatus(SuggestionStatus),
    SuggestionResponse(SuggestionResponse),
    Accusation(Accusation),
    Winner(Character),
    Disqualified(Character),
    Error(String),
}

#[derive(Debug, From, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameClientMessage {
    Move(Location),
    Suggest(Suggest),
    SuggestionResponse(SuggestionResponse),
    Accuse(Accuse),
}

#[derive(Debug, From, Clone, PartialEq, Serialize, Deserialize)]
pub enum ServerMessage {
    PreGame(PreGameServerMsg),
    Registration(RegistrationServerMsg),
    InGame(GameServerMessage),
    Error(String),
}

#[derive(Debug, From, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClientMessage {
    PreGame(PreGameClientMsg),
    Registration(RegistrationClientMsg),
    InGame(GameClientMessage),
}


#[derive(Debug, Serialize, Deserialize)]
pub enum GameMessage {
    Status(Status),


    // in game
    #[serde(skip)]
    Error(anyhow::Error),
}

impl From<Result<String, anyhow::Error>> for ServerMessage {
    fn from(res: Result<String, anyhow::Error>) -> Self {
        match res {
            Ok(val) => match serde_json::from_str(&val) {
                Ok(val) => val,
                Err(e) => ServerMessage::Error(anyhow::Error::new(e).to_string()),
            }
            Err(e) => ServerMessage::Error(e.to_string()),
        }
    }
}

impl Into<Result<String, anyhow::Error>> for ClientMessage {
    fn into(self) -> Result<String, anyhow::Error> {
        match serde_json::to_string(&self) {
            Ok(s) => Ok(s),
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Accuse {
    player: Character,
    room: Room,
    suspect: Character,
    weapon: Weapon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Accusation {
    player: Character,
    room: Room,
    suspect: Character,
    weapon: Weapon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WitnessResponse {
    Denied,
    Witnessed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuggestionStatus {
    color: Character,
    status: WitnessResponse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuggestionResponse {
    Denied,
    Witness(WitnessValue),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Suggestion {
    player: Character,
    room: Room,
    suspect: Character,
    weapon: Weapon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Suggest {
    room: Room,
    suspect: Character,
    weapon: Weapon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerPosition {
    Yellow(Location),
    Red(Location),
    Purple(Location),
    Green(Location),
    White(Location),
    Blue(Location),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WitnessValue {
    Room(Room),
    Character(Character),
    Weapon(Weapon),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Hallway {
    StudyHall,
    HallLounge,
    StudyLibrary,
    HallBilliard,
    LoungeDining,
    LibraryBilliard,
    BilliardDining,
    LibraryConservatory,
    BilliardBallroom,
    DiningKitchen,
    ConservatoryBallroom,
    BallroomKitchen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, From, Serialize, Deserialize)]
pub enum Location {
    Room(Room),
    Hallway(Hallway),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Room {
    Study,
    Hall,
    Lounge,
    Library,
    Billiard,
    Dining,
    Conservatory,
    Ballroom,
    Kitchen,
}

#[derive(Debug, Clone,  Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weapon {
    Rope,
    Pipe,
    Knife,
    Wrench,
    Candlestick,
    Revolver,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    Error(String),
    Ok,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiError {
    NoSuchUser,
    InvalidRegistration,
    Unregistered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Character {
    Yellow,
    Red,
    Purple,
    Green,
    White,
    Blue,
}

impl Character {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Yellow => "Col. Mustard",
            Self::Red => "Miss Scarlet",
            Self::Purple => "Professor Plum",
            Self::Green => "Mr. Green",
            Self::White => "Mrs. White",
            Self::Blue => "Mrs. Peacock",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Self::Yellow => "yellow",
            Self::Red => "red",
            Self::Purple => "purple",
            Self::Green => "green",
            Self::White => "white",
            Self::Blue => "blue",
        }
    }

    pub fn from_color(color: &str) -> Self {
        match color {
            "yellow" => Self::Yellow,
            "red" => Self::Red,
            "purple" => Self::Purple,
            "green" => Self::Green,
            "white" => Self::White,
            "blue" => Self::Blue,
            other => panic!("invalid character: {}", other),
        }
    }

    pub fn from_name(name: &str) -> Self {
        match name {
            "Col. Mustard" => Self::Yellow,
            "Miss Scarlet" => Self::Red,
            "Professor Plum" => Self::Purple,
            "Mr. Green" => Self::Green,
            "Mrs. White" => Self::White,
            "Mrs. Peacock" => Self::Blue,
            other => panic!("invalid character: {}", other),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterRegistration {
    pub character: Character,
    pub name: String,
}

#[derive(Debug, Clone, From, PartialEq, Serialize, Deserialize)]
pub enum RegistrationState {
    Unregistered(Character),
    Registered(CharacterRegistration),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegistrationStatus([RegistrationState; 6]);

impl Deref for RegistrationStatus {
    type Target = [RegistrationState; 6];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RegistrationStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl Default for RegistrationStatus {
    fn default() -> Self {
        Self([
            RegistrationState::Unregistered(Character::Yellow),
            RegistrationState::Unregistered(Character::Red),
            RegistrationState::Unregistered(Character::Purple),
            RegistrationState::Unregistered(Character::Green),
            RegistrationState::Unregistered(Character::White),
            RegistrationState::Unregistered(Character::Blue),
        ])
    }
}

impl Index<Character> for RegistrationStatus {
    type Output = RegistrationState;

    fn index(&self, character: Character) -> &RegistrationState {
        match character {
            Character::Yellow => &self.0[0],
            Character::Red => &self.0[1],
            Character::Purple => &self.0[2],
            Character::Green => &self.0[3],
            Character::White => &self.0[4],
            Character::Blue => &self.0[5],
        }
    }
}

impl IndexMut<Character> for RegistrationStatus {
    fn index_mut(&mut self, character: Character) -> &mut RegistrationState {
        match character {
            Character::Yellow => &mut self.0[0],
            Character::Red => &mut self.0[1],
            Character::Purple => &mut self.0[2],
            Character::Green => &mut self.0[3],
            Character::White => &mut self.0[4],
            Character::Blue => &mut self.0[5],
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompleteRegistration([CharacterRegistration; 6]);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Register {
    pub color: Character,
    pub display_name: String,
}

impl Register {
    pub fn new(color: Character, display_name: String) -> Self {
        Self { color, display_name }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn messsage_serialize() {
        let msg = GameMessage::NewGame;
        eprintln!("{}", serde_json::to_string(&msg).unwrap());
        let msg = GameMessage::JoinGame(GameId::new());
        eprintln!("{}", serde_json::to_string(&msg).unwrap());
        panic!();
    }
}
