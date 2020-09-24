use std::ops::Deref;
use super::*;

pub async fn handle_user_message(id: UserId, msg: Message, users: &Users, games: &Games) {
    eprintln!("got message from {}: {:?}", id, msg);
    let contents = if msg.is_text() {
        msg.to_str().unwrap()
    } else {
        return;
    };
    let game_msg: GameMessage = match serde_json::from_str(&contents) {
        Ok(msg) => msg,
        Err(e) => {
            eprintln!("Error decoding message: {}\n:{}", contents, e);
            return;
        }
    };

    match game_msg {
        GameMessage::NewGame => {
            new_game(id, users, games).await;
        },
        GameMessage::JoinGame(game_id) => {
            join_game(id, game_id, users, games).await;
        },
        other => {
            eprintln!("Got message {:?}", other);
            return;
        }
    };
}

#[derive(Clone, Default)]
pub struct Users {
    pub users: Arc<RwLock<HashMap<UserId, User>>>
}

impl Users {
    pub async fn send_to_user(&self, user_id: UserId, msg: GameMessage) {
        match self.read().await.get(&user_id) {
            Some(user) => user.send(msg),
            None => (),
        }
    }

    pub async fn set_user_game(&self, user_id: UserId, game_id: GameId) -> bool {
        if let Some(user) = self.users.write().await.get_mut(&user_id) {
            user.set_game(game_id)
        } else {
            false
        }
    }
}

impl Deref for Users {
    type Target = Arc<RwLock<HashMap<UserId, User>>>;

    fn deref(&self) -> &Arc<RwLock<HashMap<UserId, User>>> {
        &self.users
    }
}


#[derive(Debug)]
pub struct UnregisteredUser {
    pub id: UserId,
    pub game_id: Option<GameId>,
    pub sender: mpsc::UnboundedSender<Result<Message, warp::Error>>
}

#[derive(Debug)]
pub struct RegisteredUser {
    name: String,
    id: UserId,
    game_id: GameId,
    character: usize,
    sender: mpsc::UnboundedSender<Result<Message, warp::Error>>
}

#[derive(Debug)]
pub enum User {
    Unregistered(UnregisteredUser),
    Registered(RegisteredUser),
}

impl User {
    pub fn send(&self, msg: GameMessage) {
        let sender = match self {
            Self::Unregistered(user) => &user.sender,
            Self::Registered(user) => &user.sender,
        };
        match sender.send(Ok(Message::text(serde_json::to_string(&msg).unwrap()))) {
            Ok(()) => (),
            Err(e) => eprintln!("error sending message: {}", e),
        }
    }

    pub fn set_game(&mut self, game_id: GameId) -> bool {
        match self {
            Self::Unregistered(user) => {
                if user.game_id.is_none() {
                    user.game_id = Some(game_id);
                    self.send(GameMessage::GameId(game_id));
                    true
                } else {
                    self.send(GameMessage::Status(Status::Error("already in game".to_owned())));
                    false
                }
            }
            _ => {
                self.send(GameMessage::Status(Status::Error("game is in progress".to_owned())));
                false
            },
        }
    }
}
