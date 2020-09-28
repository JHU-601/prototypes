use std::collections::HashMap;
use std::sync::{
    Arc,
};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate derive_more;

use clueless_common::*;
use futures::{FutureExt, StreamExt};
use tokio::sync::{mpsc, RwLock};
use warp::ws::{Message, WebSocket, Ws};
use warp::Filter;
use uuid::Uuid;

mod routes;
use routes::*;
pub(crate) mod user;
pub(crate) use user::*;
pub(crate) mod game;
pub(crate) use game::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    /*
    let users = Users::default();
    let users = warp::any().map(move || users.clone());

    let games = Games::default();

    let games = warp::any().map(move || games.clone());

    let create_game = warp::path("game")
        .and(warp::ws())
        .and(users)
        .and(games)
        .map(|ws: Ws, users, games| {
            ws.on_upgrade(move |socket| user_connected(socket, users, games))
        });

    let index = warp::path::end().map(|| warp::reply::html(INDEX_HTML));
    */

    //let routes = index.or(create_game);
    let routes = routes();

    warp::serve(routes).run(([127,0,0,1], 3030)).await;
}

/*

static INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <title>Clue-Less</title>
    </head>
    <body>
        <h1>Clue-Less</h1>
        <div id="game">
            <p><em>Connecting...</em></p>
        </div>
        <button type="button" id="create-game">Create Game</button>
        <input type="text" id="game_id" />
        <button type="button" id="join-game">Join Game</button>
        <input type="text" id="command-name" />
        <input type="text" id="command" />
        <button type="button" id="send">Send Command</button>
        <script type="text/javascript">
        const create = document.getElementById('create-game');
        const join = document.getElementById('join-game');
        const uri = 'ws://' + location.host + '/game';
        const ws = new WebSocket(uri);
        function message(data) {
            const line = document.createElement('p');
            line.innerText = data;
            game.appendChild(line);
        };
        ws.onopen = function() {
            game.innerHTML = '<p><em>Creating game</em></p>';
        };
        ws.onmessage = function(msg) {
            message(msg.data);
        };
        ws.onclose = function() {
            game.getElementsByTagName('em')[0].innerText = 'Disconnected!';
        };
        send.onclick = function() {
            const msg = command.value;
            ws.send(msg);
            command.value = '';
            message('<You>: ' + msg);
        };
        create.onclick = function() {
            ws.send('"NewGame"');
        };
        join.onclick = function() {
            const msg = game_id.value;
            ws.send('{"JoinGame": "' + msg + '"}');
            game_id.value = '';
            message("Joining game: " + msg);
        };
        </script>
    </body>
</html>
"#;
*/
