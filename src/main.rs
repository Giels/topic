#[macro_use]
extern crate router;
extern crate topic;
extern crate iron;
extern crate persistent;

use topic::db;
use topic::handlers::{handle_main, handle_board, handle_thread, redirect_page0, redirect_up};
use topic::handlers::{handle_new_thread, handle_new_post, handle_report_delete_post};
use topic::handlers::handle_resource;
use topic::handlers::{handle_mod_page, handle_login_page, handle_login};

use iron::prelude::*;

use topic::config;
use topic::sessions;

use std::collections::HashMap;

fn main() {
    let config = config::parse("config.toml");
    let pool = db::postgres_pool(&config.database);
    let logged_mods = HashMap::new();

    let router = router! {
        post "/:board/t/:thread/:page/new" => handle_new_post,
        post "/:board/t/:thread/:page/report_delete" => handle_report_delete_post,
        get "/:board/t/:thread/:page" => handle_thread,
        get "/:board/t/:thread" => redirect_page0,
        post "/:board/new" => handle_new_thread,
        get "/:board/t" => redirect_up,
        get "/:board/:page" => handle_board,
        get "/res/:res" => handle_resource,
        get "/mod/:page" => handle_mod_page,
        get "/login" => handle_login_page,
        get "/login_" => handle_login,
        get "/mod" => redirect_page0,
        get "/:board" => redirect_page0,
        get "/" => handle_main,
    };

    let mut chain = iron::Chain::new(router);
    let listen_on = &format!("{}:{}", config.server.host, config.server.port) as &str;
    chain.link(persistent::Read::<db::PostgresPool>::both(pool));
    chain.link(persistent::Read::<config::Config>::both(config));
    chain.link(persistent::Read::<sessions::SessionStore<String, String>>::both(logged_mods));
    Iron::new(chain).http(listen_on).unwrap();
}
