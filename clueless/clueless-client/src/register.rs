use yew::prelude::*;

use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

use clueless_common::*;

fn render_registration_state(state: &RegistrationState) -> Html {
    match state {
        RegistrationState::Unregistered(character) => {
            html! {
                <option value={ character.color() }>{ character.name() }</option>
            }
        }
        _ => html! {<></>}
    }
}

fn render_reg_state_table(state: &RegistrationState) -> Html {
    match state {
        RegistrationState::Unregistered(character) => html! {
            <tr>
                <td>{ character.name() }</td>
                <td></td>
            </tr>
        },
        RegistrationState::Registered(reg) => html! {
            <tr>
                <td>{ reg.character.name() }</td>
                <td>{ &reg.name }</td>
                </tr>
        }
    }
}

fn render_registration_status(status: &RegistrationStatus) -> Html {
    html! {
        <table>
            <thead>
            <tr>{ "Character" }</tr>
            <tr>{ "Player" }</tr>
            </thead>
            <tbody>
            {for status.iter().map(render_reg_state_table) }
            </tbody>
        </table>
    }
}

#[derive(Debug)]
pub struct RegistrationView {
    link: ComponentLink<Self>,
    props: Props,
    character: Character,
    name: String,
}


fn select_user(val: ChangeData) -> Option<Msg> {
    match val {
        ChangeData::Value(_) | ChangeData::Files(_) => None,
        ChangeData::Select(element) => Some(Msg::SelectUser(Character::from_color(&element.value()))),
    }
}


#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Props {
    pub on_register: Callback<RegistrationClientMsg>,
    pub status: RegistrationStatus,
}

#[derive(Debug)]
pub enum Msg {
    SelectUser(Character),
    NameInput(String),
    Submit,
}

impl Component for RegistrationView {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props, name: String::new(), character: Character::Yellow }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::SelectUser(c) => self.character = c,
            Msg::NameInput(name) => self.name = name,
            Msg::Submit => self.props.on_register.emit(RegistrationClientMsg::Register(Register::new(self.character, std::mem::take(&mut self.name)))),
        }
        true
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
                <fieldset>
                <legend>{ "Registration" }</legend>
                <label for="display">{ "Display Name" }</label>
                <input type="text" id="display" placeholder="display name" oninput=self.link.callback(|e: InputData| Msg::NameInput(e.value))/>
                <label for="character">{ "Select Character" }</label>
                <select name="character" id="character" onchange=self.link.callback(|val: ChangeData| select_user(val).unwrap_or_else(|| Msg::NameInput(String::new()))) >
            { for self.props.status.iter().map(render_registration_state) }
            </select>
                <button onclick=self.link.callback(|_| Msg::Submit)>{ "Register" }</button>
                </fieldset>
                { render_registration_status(&self.props.status) }
            </>
        }
    }

}
