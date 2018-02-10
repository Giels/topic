extern crate iron;
extern crate mustache;

extern crate rustc_serialize;

use iron::prelude::*;
use persistent::Read;

use std::collections::HashMap;

use ::sha2::Sha256;
use ::sha2::Digest;

use ::params::FromValue;

use ::router::Router;

use hoedown as md;
use hoedown::renderer::Render;

use core::borrow::Borrow;

use std::str;
use std::cmp;
use core::borrow::BorrowMut;

use std::sync::Arc;

pub fn handle_login_page(request: &mut Request) -> IronResult<Response> {
    let mut bytes = vec![];
    let mut data: HashMap<String, String> = HashMap::new();
    let template = ::mustache::compile_path("assets/mod.tpl").unwrap();
    template.render(&mut bytes, &data).unwrap();

    Ok(Response::with(("text/html".parse::<::iron::mime::Mime>().unwrap(), ::iron::status::Ok,
    str::from_utf8(&bytes).unwrap())))
}

pub fn handle_mod_page(request: &mut Request) -> IronResult<Response> {
    let conn = request.get::<Read<::db::PostgresPool>>().unwrap().get().unwrap();
    let params = request.get::<::params::Params>().unwrap();
    let mut next_url = request.url.clone();
    let session = request.get::<::persistent::Read<::sessions::SessionStore<String, String>>>().unwrap();
    
    let mut hasher = ::sha2::Sha256::new();
    hasher.input_str(&String::from_value(&params["ip"]).unwrap() as &str);
    let ip = hasher.result_str();

    let mut data: HashMap<String, String> = HashMap::new();

    session.get(&ip).and_then(|uname| {
        let possible_powers = vec!["can_delete", "can_ban", "can_sticky", "can_edit"];
        let mod_powers = ::db::get_mod_powers(&conn, uname).unwrap();
        let mod_str = "Mod Abilities:".to_string();
        for p in possible_powers {
            data.insert(p.to_string(), mod_powers.get(0).get(p));
        }

        let mut bytes = vec![];
        let template = ::mustache::compile_path("assets/login.tpl").unwrap();
        template.render(&mut bytes, &data).unwrap();

        Some(Ok(Response::with(("text/html".parse::<::iron::mime::Mime>().unwrap(), ::iron::status::Ok,
        str::from_utf8(&bytes).unwrap()))))
    }).unwrap_or({
        next_url.path.pop();
        Ok(Response::with((::iron::status::MovedPermanently, ::iron::modifiers::Redirect(next_url))))
    })
}

pub fn handle_login(request: &mut Request) -> IronResult<Response> {
    let conn = request.get::<::persistent::Read<::db::PostgresPool>>().unwrap().get().unwrap();
    let params = request.get::<::params::Params>().unwrap();
    let mut next_url = request.url.clone();
    let params = request.get::<::params::Params>().unwrap();
    let uname = &String::from_value(&params["uname"]).unwrap() as &str;
    let pass = &String::from_value(&params["pass"]).unwrap() as &str;
    let page = str::parse::<i64>(&get_var!(request, "page") as &str);
    
    let mut hasher = ::sha2::Sha256::new();
    hasher.input_str(pass);
    let pass = &hasher.result_str() as &str;

    if ::db::valid_mod_login(&conn, uname, pass).unwrap().get(0).get(0) {
        let mut hasher = ::sha2::Sha256::new();
        hasher.input_str(&String::from_value(&params["ip"]).unwrap() as &str);
        let ip = hasher.result_str();

        request.get_mut::<::persistent::Read<::sessions::SessionStore<String, String>>>().map(|mut session| {
            let ref mut table = *Arc::get_mut(&mut session).unwrap();
            table.insert(ip, uname.to_string());
        });

        next_url.path.pop();
        next_url.path.push("mod/0".to_string());
        return Ok(Response::with((::iron::status::MovedPermanently, ::iron::modifiers::Redirect(next_url))))
    }

    Ok(Response::with(("text/html".parse::<iron::mime::Mime>().unwrap(), iron::status::Ok, "Incorrect password or username")))
}
