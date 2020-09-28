use std::ops::Deref;
use super::*;

pub async fn handle_user_message(id: UserId, msg: Message, users: &Users, games: &Games) {
    eprintln!("got message from {}: {:?}", id, msg);
    let contents = if msg.is_text() {
        msg.to_str().unwrap()
    } else {
        return;
    };
    let game_msg: ClientMessage = match serde_json::from_str(&contents) {
        Ok(msg) => msg,
        Err(e) => {
            eprintln!("Error decoding message: {}\n:{}", contents, e);
            return;
        }
    };

    match game_msg {
        ClientMessage::PreGame(PreGameClientMsg::NewGame) => {
            new_game(id, users, games).await;
        },
        ClientMessage::PreGame(PreGameClientMsg::JoinGame(game_id)) => {
            join_game(id, game_id, users, games).await;
        },
        //GameMessage::Register(reg) => handle_registration(id, reg, users, games).await,
        other => {
            eprintln!("Got message {:?}", other);
            return;
        }
    };
}

/*
pub async fn handle_registration(user_id: UserId, msg: Register, users: &Users, games: &Games) {
    let game_id = if let Some(user) = users.write().await.get_mut(&user_id) {
        match user.register(msg.display_name.clone(), msg.character) {
            Ok(_) => user.user_id().expect("registered a user not in game"),
            Err(_) => {
                eprintln!("invalid registration");
                return;
            }
        }
    } else {
        return;
    };
}
*/

#[derive(Clone, Default)]
pub struct Users {
    pub users: Arc<RwLock<HashMap<UserId, User>>>
}

impl Users {
    pub async fn user_action<F, G>(&self, user_id: UserId, action_fn: F) -> Result<G, ApiError>
    where F: Fn(&User) -> Result<G, ApiError>
    {
        match self.users.read().await.get(&user_id) {
            Some(user) => action_fn(user),
            None => Err(ApiError::NoSuchUser),
        }
    }

    pub async fn send_to_user(&self, user_id: UserId, msg: ServerMessage) -> bool {
        match self.read().await.get(&user_id) {
            Some(user) => {
                user.send(msg);
                true
            },
            None => false,
        }
    }

    pub async fn set_user_game(&self, user_id: UserId, game_id: GameId) -> bool {
        if let Some(user) = self.users.write().await.get_mut(&user_id) {
            user.set_game(game_id)
        } else {
            false
        }
    }

    pub async fn user_game(&self, user_id: UserId) -> Option<GameId> {
        self.users.read().await.get(&user_id).and_then(|user| user.game_id())
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
    character: Character,
    sender: mpsc::UnboundedSender<Result<Message, warp::Error>>
}

#[derive(Debug)]
pub enum User {
    Unregistered(UnregisteredUser),
    Registered(RegisteredUser),
}

impl User {
    pub fn character(&self) -> Result<Character, ApiError> {
        match self {
            Self::Unregistered(_) => Err(ApiError::Unregistered),
            Self::Registered(user) => Ok(user.character)
        }
    }

    pub fn game_id(&self) -> Option<GameId> {
        match self {
            Self::Unregistered(user) => user.game_id,
            Self::Registered(user) => Some(user.game_id),
        }
    }

    pub fn send(&self, msg: ServerMessage) {
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
                    self.send(PreGameServerMsg::Joined(game_id).into());
                    true
                } else {
                    self.send(ServerMessage::Error("already in game".to_owned()));
                    false
                }
            }
            _ => {
                self.send(ServerMessage::Error("game is in progress".to_owned()));
                false
            },
        }
    }

    pub fn register(&mut self, name: String, character: Character) -> Result<(), ()> {
        if let User::Unregistered(user) = self {
            if let Some(game_id) = user.game_id {
                *self = User::Registered(RegisteredUser{
                    name,
                    character,
                    game_id,
                    id: user.id,
                    sender: user.sender.clone(),
                });
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}
