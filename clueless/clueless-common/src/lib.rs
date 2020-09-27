#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_more;

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Serialize, Deserialize)]
#[display(fmt = "{}", _0)]
pub struct GameId(Uuid);

impl GameId {
    pub fn new() -> Self {
        GameId(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Serialize, Deserialize)]
#[display(fmt = "{}", _0)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        UserId(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameMessage {
    Status(Status),
    NewGame,
    JoinGame(GameId),
    UserId(UserId),
    GameId(GameId),
    UserJoined(UserId),

    Available(Available),
    Register(Register),
    Registration(Registration),
    Complete,


    // in game
    Witness(WitnessValue),
    Position(PlayerPosition),
    PlayerPosition,
    Move(u8),
    Suggest(Suggest),
    Suggestion(Suggestion),
    SuggestionQuery,
    SuggestionResponse(SuggestionResponse),
    SuggestionStatus(SuggestionStatus),
    Accuse(Accuse),
    Accusation(Accusation),
    Winner(Character),
    Disqualified(Character),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Accuse {
    player: Character,
    room: Room,
    suspect: Character,
    weapon: Weapon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Accusation {
    player: Character,
    room: Room,
    suspect: Character,
    weapon: Weapon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WitnessResponse {
    Denied,
    Witnessed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionStatus {
    color: Character,
    status: WitnessResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionResponse {
    Denied,
    Witness(WitnessValue),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    player: Character,
    room: Room,
    suspect: Character,
    weapon: Weapon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggest {
    room: Room,
    suspect: Character,
    weapon: Weapon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerPosition {
    Yellow(u8),
    Red(u8),
    Purple(u8),
    Green(u8),
    White(u8),
    Blue(u8),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WitnessValue {
    Room(Room),
    Character(Character),
    Weapon(Weapon),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiError {
    NoSuchUser,
    InvalidRegistration,
    Unregistered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Character {
    Yellow,
    Red,
    Purple,
    Green,
    White,
    Blue,
}

#[derive(Debug, Clone, Serialize, Deserialize, From)]
pub struct Available(Vec<Character>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Register {
    color: Character,
    display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registration {
    color: Character,
    display_name: String,
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
