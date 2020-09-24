use super::*;
use crate::user::*;
use std::ops::Deref;

#[derive(Clone, Default)]
pub struct Games {
    games: Arc<RwLock<HashMap<GameId, Game>>>,
}

impl Games {
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
    user_registrations: [Option<UserId>; 6],
    available: Vec<Character>,
}

impl PreGame {
    async fn add_user(&mut self, user_id: UserId, users: &Users) -> bool {
        eprintln!("{:?}", self.user_registrations);
        for id_opt in self.user_registrations.iter() {
            dbg!(id_opt);
            if let Some(id) = id_opt {
                dbg!(id);
                if let Some(user) = users.read().await.get(&id) {
                    dbg!(&user);
                    user.send(GameMessage::UserJoined(user_id));
                } else {
                    eprintln!("error: user {} not found in game {}", id, self.id);
                }
            }
        }
        for id in self.user_registrations.iter_mut() {
            if id.is_none() {
                *id = Some(user_id);
            }
        }

        let msg = GameMessage::Available(self.available.clone().into());
        if let Some(user) = users.read().await.get(&user_id) {
            user.send(msg);
            true
        } else {
            false
        }
    }
}

impl Default for PreGame {
    fn default() -> Self {
        PreGame {
            id: GameId::new(),
            user_registrations: [None; 6],
            available: vec![
                Character::Yellow,
                Character::Red,
                Character::Purple,
                Character::Green,
                Character::White,
                Character::Blue,
            ],
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
            Self::Waiting(game) => game.user_registrations.iter().all(|reg| reg.is_some()),
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
            users.send_to_user(user_id, GameMessage::Status(Status::Error("game not found".to_owned()))).await;
            return
        }
    };

    if is_full {
        users.send_to_user(user_id, GameMessage::Status(Status::Error("game is full".to_owned()))).await;
        return;
    }

    if users.set_user_game(user_id, game_id).await &&
        !games.add_user(user_id, game_id, users).await {
            users.send_to_user(user_id, GameMessage::Status(Status::Error("could not join game".to_owned()))).await;
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
    user.send(GameMessage::UserId(user_id));

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
