use wasm_bindgen::prelude::*;
use yew::prelude::*;

use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

use clueless_common::GameId;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub join_game: Callback<GameId>,
    pub create_game: Callback<()>,
}

#[derive(Debug)]
pub struct JoinRoom {
    link: ComponentLink<Self>,
    props: Props,
    input: String,
}

pub enum Msg {
    CreateGame,
    JoinGame,
    Input(String),
}

impl Component for JoinRoom {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link, input: String::new() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(value) => {
                self.input = value;
                true
            }
            Msg::CreateGame => {
                self.props.create_game.emit(());
                true
            }
            Msg::JoinGame => {
                self.props.join_game.emit(GameId::from_str(&self.input).unwrap());
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props == props {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <button onclick=self.link.callback(|_| Msg::CreateGame)>{ "Create Game" }</button>
                <input placeholder="Game ID" value=&self.input oninput=self.link.callback(|e: InputData| Msg::Input(e.value))/>
                <button onclick=self.link.callback(|_| Msg::JoinGame)>{ "Join Game" }</button>
            </>
        }
    }
}
