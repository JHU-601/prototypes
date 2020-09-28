#![recursion_limit="512"]
#[macro_use]
extern crate serde;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use yew::services::ConsoleService;
use yew::format::Json;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

use clueless_common::*;

use serde::{Serialize, Deserialize};

mod join_room;
use join_room::*;
mod register;
use register::*;

#[derive(Debug)]
enum GameState {
    Connecting,
    Connected,
    Joining,
    Registration(PlayerRegistrationState),
}

#[derive(Debug)]
pub struct PlayerRegistrationState {
    game: GameId,
    status: RegistrationStatus,
}

impl PlayerRegistrationState {
    fn new(game: GameId) -> Self {
        Self{ game, status: RegistrationStatus::default() }
    }

    fn handle_msg(&mut self, msg: RegistrationServerMsg) {
        match msg {
            RegistrationServerMsg::RegistrationStatus(status) => self.status = status,
            RegistrationServerMsg::Registration(reg) => {
                let character = reg.character;
                self.status[character] = RegistrationState::Registered(reg);
            }
            _ => (),
        }
    }
}

impl GameState {
    pub fn is_pregame(&self) -> bool {
        match self {
            Self::Connecting | Self::Connected | Self::Joining => true,
            _ => false,
        }
    }

    pub fn is_registration(&self) -> bool {
        match self {
            Self::Registration(_) => true,
            _ => false,
        }
    }
}

struct GameView {
    link: ComponentLink<Self>,
    state: GameState,
    socket: WebSocketTask,
    user_id: UserId,
}

/*
impl GameView {
    fn handle_game_message(&mut self, message: GameMessage) -> ShouldRender {
        match message {
            GameMessage::UserId(id) => {
                ConsoleService::log(&format!("got user id: {}", id));
                self.user_id = id;
                false
            }
            msg => {
                ConsoleService::log(&format!("got msg: {:?}", msg));
                false
            }
        }
    }
}
*/

#[derive(Debug)]
enum Msg {
    WsNotify(WebSocketStatus),
    CreateGame,
    JoinGame(GameId),
    WsFailed,
    GameMessage(ServerMessage),
    UserMessage(ClientMessage),
}

impl Component for GameView {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|msg| Msg::GameMessage(msg));
        let notify = link.batch_callback(|status| vec![Msg::WsNotify(status)]);
        Self {
            link,
            state: GameState::Connecting,
            socket: WebSocketService::connect_text("ws://localhost:3030/game", callback, notify).unwrap(),
            user_id: UserId::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        ConsoleService::log(&format!("got msg: {:?}", msg));
        match msg {
            Msg::GameMessage(msg) => match msg {
                ServerMessage::InGame(msg) => {
                    ConsoleService::log(&format!("got msg: {:?}", msg));
                    false
                }
                ServerMessage::PreGame(msg) if self.state.is_pregame() => {
                    match msg {
                        PreGameServerMsg::UserId(id) => self.user_id = id,
                        PreGameServerMsg::Joined(id) => {
                            self.state = GameState::Registration(PlayerRegistrationState::new(id));
                        }
                    }
                    true
                }
                ServerMessage::Registration(msg) => match &mut self.state {
                    GameState::Registration(reg) => {
                        reg.handle_msg(msg);
                        true
                    }
                    _ => {
                        ConsoleService::log(&format!("invalid state for message {:?}", msg));
                        false
                    }
                }
                other => {
                    ConsoleService::log(&format!("Invalid message for current state: {:?}", other));
                    false
                }
            },
            Msg::UserMessage(msg @ ClientMessage::Registration(_)) if self.state.is_registration() => {
                self.socket.send(msg);
                true
            }

            Msg::WsNotify(WebSocketStatus::Opened) => {
                ConsoleService::log("connected");
                self.state = GameState::Connected;
                true
            },
            Msg::CreateGame if self.state.is_pregame() => {
                self.socket.send(ClientMessage::PreGame(PreGameClientMsg::NewGame));
                self.state = GameState::Joining;
                true
            },
            Msg::JoinGame(id)  if self.state.is_pregame() => {
                self.socket.send(ClientMessage::PreGame(PreGameClientMsg::JoinGame(id)));
                self.state = GameState::Joining;
                true
            }

            _ => {
                ConsoleService::log(&format!("Msg: {:#?}", msg));
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
// <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
        match &self.state {
            GameState::Connecting => {
                html! {
                    <div>
                        <p> { "Connecting..." } </p>
                        </div>
                }
            },
            GameState::Connected => {
                html! {
                    <div>
                        <JoinRoom create_game=self.link.callback(|_| Msg::CreateGame) join_game=self.link.callback(|id| Msg::JoinGame(id)) />
                    </div>
                }
            }
            GameState::Joining => {
                html! {
                    <div>
                        <p> { "Joining game..." } </p>
                    </div>
                }
            }
            GameState::Registration(state) => {
                html! {
                    <div>
                        <RegistrationView on_register=self.link.callback(|msg| Msg::UserMessage(ClientMessage::Registration(msg))) status=state.status.clone() />
                    </div>
                }
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<GameView>::new().mount_to_body();
}
