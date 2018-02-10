extern crate iron;
extern crate mustache;

use iron::prelude::*;

use std::collections::HashMap;

use topic::utils;
use topic::db;

pub fn handle_new_thread(pool: &utils::PostgresPool) -> utils::RouteHandler {
    let pool = pool.clone();

    move |request: &mut Request|
    {
        let ref board = utils::get_var!(request, "board");
        let ref thread = utils::get_var!(request, "thread");
        Ok(Response::with((iron::status::Ok, format!("Hello from POST /{}/{}/new", board, thread))))
    }
}

pub fn handle_new_post(pool: &utils::PostgresPool) -> utils::RouteHandler {
    let pool = pool.clone();

    move |request: &mut Request|
    {
        let ref board = utils::get_var!(request, "board");
        let ref thread = utils::get_var!(request, "thread");
        Ok(Response::with((iron::status::Ok, format!("Hello from POST /{}/{}/new", board, thread))))
    }
}
