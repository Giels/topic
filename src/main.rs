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
        new_post: post "/:board/t/:thread/:page/new" => handle_new_post,
        mod_post: post "/:board/t/:thread/:page/report_delete" => handle_report_delete_post,
        view_thread: get "/:board/t/:thread/:page" => handle_thread,
        view_thread0: get "/:board/t/:thread" => redirect_page0,
        new_thread: post "/:board/new" => handle_new_thread,
        no_thread_id: get "/:board/t" => redirect_up,
        view_board: get "/:board/:page" => handle_board,
        view_res: get "/res/:res" => handle_resource,
        view_admin: get "/mod/:page" => handle_mod_page,
        view_admin_login: get "/login" => handle_login_page,
        manage_admin_login: post "/login_" => handle_login,
        no_mod_page: get "/mod" => redirect_page0,
        no_board_page: get "/:board" => redirect_page0,
        index: get "/" => handle_main,
    };

    let mut chain = iron::Chain::new(router);
    let listen_on = &format!("{}:{}", config.server.host, config.server.port) as &str;
    chain.link(persistent::Read::<db::PostgresPool>::both(pool));
    chain.link(persistent::Read::<config::Config>::both(config));
    chain.link(persistent::Read::<sessions::SessionStore<String, String>>::both(logged_mods));
    Iron::new(chain).http(listen_on).unwrap();
}
