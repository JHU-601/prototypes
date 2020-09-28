use super::*;
use crate::user::*;
use std::ops::Deref;

static ALL_CHARACTERS: [Character; 6] = [
    Character::Yellow,
    Character::Red,
    Character::Purple,
    Character::Green,
    Character::White,
    Character::Blue,
];

#[derive(Clone, Default)]
pub struct Games {
    games: Arc<RwLock<HashMap<GameId, Game>>>,
}

impl Games {
    /*
    pub async fn available_users(&self, game_id: GameId, users: &Users) -> Option<Vec<Character>> {
        if let Some(game) = self.games.read().await.get(&game_id) {
            let mut available = ALL_CHARACTERS.clone();
            while let Some(user_id) = game.user_ids().iter() {
                
            }

        } else {
            None
        }

    }
    */

    pub async fn add_user(&self, user_id: UserId, game_id: GameId, users: &Users) -> bool {
        match self.games.write().await.get_mut(&game_id) {
            Some(game) => {
                game.add_user(user_id, users).await
            },
            None => {
                false
            }
        }
    }
}

impl Deref for Games {
    type Target = Arc<RwLock<HashMap<GameId, Game>>>;

    fn deref(&self) -> &Arc<RwLock<HashMap<GameId, Game>>> {
        &self.games
    }
}


pub struct ActiveGame {
    id: GameId,
    users: [UserId; 6],
    current_player: usize,
}

pub struct PreGame {
    id: GameId,
    users: [Option<UserId>; 6],
    reg_status: RegistrationStatus,
}

impl PreGame {
    async fn add_user(&mut self, user_id: UserId, users: &Users) -> bool {
        eprintln!("{:?}", self.users);
        for id_opt in self.users.iter() {
            dbg!(id_opt);
            if let Some(id) = id_opt {
                dbg!(id);
                if let Some(user) = users.read().await.get(&id) {
                    dbg!(&user);
                    user.send(RegistrationServerMsg::UserJoined(user_id).into());
                } else {
                    eprintln!("error: user {} not found in game {}", id, self.id);
                }
            }
        }
        for id in self.users.iter_mut() {
            if id.is_none() {
                *id = Some(user_id);
                break;
            }
        }

        let msg = ServerMessage::Registration(self.reg_status.clone().into());
        users.send_to_user(user_id, msg).await
    }
}

impl Default for PreGame {
    fn default() -> Self {
        PreGame {
            id: GameId::new(),
            users: [None; 6],
            reg_status: RegistrationStatus::default(),
        }
    }
}

pub enum Game {
    Waiting(PreGame),
    Active(ActiveGame),
}

impl Game {
    fn full(&self) -> bool {
        match self {
            Self::Waiting(game) => {
                eprintln!("{:#?}", game.users);

                game.users.iter().all(|reg| reg.is_some())
            },
            _ => true,
        }
    }

    async fn add_user(&mut self, user_id: UserId, users: &Users) -> bool {
        match self {
            Self::Waiting(game) => game.add_user(user_id, users).await,
            _ => false,
        }
    }

}

impl Default for Game {
    fn default() -> Self {
        Game::Waiting(PreGame::default())
    }
}


pub async fn join_game(user_id: UserId, game_id: GameId, users: &Users, games: &Games) {

    let is_full = match games.read().await.get(&game_id) {
        Some(game) => game.full(),
        None => {
            users.send_to_user(user_id, ServerMessage::Error("game not found".to_owned())).await;
            return
        }
    };

    if is_full {
        users.send_to_user(user_id, ServerMessage::Error("game is full".to_owned())).await;
        return;
    }

    if users.set_user_game(user_id, game_id).await &&
        !games.add_user(user_id, game_id, users).await {
            users.send_to_user(user_id, ServerMessage::Error("could not join game".to_owned())).await;
            return
    }

    eprintln!("join game {} {}", user_id, game_id);
    return;

}

pub async fn user_connected(ws: WebSocket, users: Users, games: Games) {
    let user_id = UserId::new();
    let (user_tx, mut user_rx) = ws.split();

    let (tx, rx) = mpsc::unbounded_channel();

    tokio::task::spawn(rx.forward(user_tx).map(|result| {
        if let Err(e) = result {
            error!("websocket send error: {}", e);
        }
    }));

    eprintln!("User {} joined", user_id);
    let user = User::Unregistered(UnregisteredUser{ sender: tx, id: user_id, game_id: None });
    user.send(PreGameServerMsg::UserId(user_id).into());

    users.write().await.insert(user_id, user);

    while let Some(result) = user_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", user_id, e);
                break;
            }
        };
        handle_user_message(user_id, msg, &users, &games).await;
    }
}



pub async fn new_game(user_id: UserId, users: &Users, games: &Games) {
    let game_id = GameId::new();


    let game = Game::default();

    eprintln!("Created game {}", game_id);

    games.write().await.insert(game_id, game);

    join_game(user_id, game_id, users, games).await
}
