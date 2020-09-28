use warp::{
    self,
    filters::BoxedFilter,
    reply::Reply,
    http::Response,
    Filter,
};

use crate::{Users, Games};

static INDEX: &'static str = "static/index.html";
static APP_JS: &'static str = "static/clueless-client.js";
static APP_WASM: &'static str = "static/clueless-client_bg.wasm";


pub fn routes() -> BoxedFilter<(impl Reply,)> {
    static_files()
        .or(game_websocket())
        .boxed()
}

pub fn game_websocket() -> BoxedFilter<(impl Reply,)> {
    let users = Users::default();
    let users = warp::any().map(move || users.clone());
    let games = Games::default();
    let games = warp::any().map(move || games.clone());

    let user_connect = warp::path("game")
        .and(warp::ws())
        .and(users)
        .and(games)
        .map(|ws: warp::ws::Ws, users, games| ws.on_upgrade(move |socket| crate::user_connected(socket, users, games)));

    user_connect.boxed()
}

pub fn static_files() -> BoxedFilter<(impl Reply,)> {
    let files = index().or(app_wasm()).or(app_js());

    warp::any().and(files).boxed()
}

fn index() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(INDEX))
        .boxed()

    //warp::path::end().map(|| warp::reply::html(INDEX)).boxed()
}

fn app_wasm() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path::path("clueless-client_bg.wasm"))
        .and(warp::path::end())
        .and(warp::fs::file(APP_WASM))
        .boxed()
    //warp::path("clueless_client.js").map(|| Response::builder)
}

fn app_js() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path::path("clueless-client.js"))
        .and(warp::path::end())
        .and(warp::fs::file(APP_JS))
        .boxed()
    //warp::path("clueless_client.js").map(|| Response::builder)
}
