extern crate iron;
extern crate mustache;

use std;

use iron::prelude::*;
use persistent::{Read,State};

use std::collections::HashMap;

use ::sha2::Sha256;
use ::sha2::Digest;

use ::params::FromValue;

use ::router::Router;

use hoedown as md;
use hoedown::renderer::Render;

use core::borrow::Borrow;

use std::cmp;
use core::borrow::BorrowMut;

use std::sync::Arc;

pub fn handle_login_page(request: &mut Request) -> IronResult<Response> {
    let mut bytes = vec![];
    let mut data: HashMap<String, String> = HashMap::new();
    let template = ::mustache::compile_path("assets/login.tpl").unwrap();
    template.render(&mut bytes, &data).unwrap();

    Ok(Response::with(("text/html".parse::<::iron::mime::Mime>().unwrap(), ::iron::status::Ok,
    std::str::from_utf8(&bytes).unwrap())))
}

pub fn handle_mod_page(request: &mut Request) -> IronResult<Response> {
    let conn = request.get::<Read<::db::PostgresPool>>().unwrap().get().unwrap();
    let params = request.get::<::params::Params>().unwrap();
    let mut next_url = request.url.clone();
    let session_lock = request.get::<State<::sessions::SessionStore<String, String>>>().unwrap();
    let session = session_lock.read().unwrap();
    
    let mut hasher = ::sha2::Sha256::new();
    hasher.input(&request.remote_addr.ip().to_string().as_bytes());
    let ip = format!("{:?}", hasher.result().as_slice()).to_owned();

    let mut data: HashMap<String, String> = HashMap::new();

    session.get(&ip.to_owned()).and_then(|uname| {
        let possible_powers = vec!["can_delete", "can_ban", "can_sticky", "can_edit"];
        let mod_powers = ::db::get_mod_powers(&conn, uname).unwrap();
        let mod_str = "Mod Abilities:".to_string();
        for p in possible_powers {
            data.insert(p.to_string(), format!("{}", mod_powers.get(0).get::<_,bool>(p)));
        }

        let mut bytes = vec![];
        let template = ::mustache::compile_path("assets/mod.tpl").unwrap();
        template.render(&mut bytes, &data).unwrap();

        Some(Ok(Response::with(("text/html".parse::<::iron::mime::Mime>().unwrap(), ::iron::status::Ok,
        std::str::from_utf8(&bytes).unwrap()))))
    }).unwrap_or({
        let mut url_string = String::new();
        url_string.push_str(&format!("{}", request.url.scheme()));
        url_string.push_str("://");
        url_string.push_str(&format!("{}", request.url.host()));
        url_string.push(':');
        url_string.push_str(&format!("{}", request.url.port()));

        let mut prev_url = request.url.path().clone();
        prev_url.pop();
        prev_url.pop();

        for s in prev_url {
            url_string.push('/');
            url_string.push_str(s);
        };

        let new_url = iron::Url::parse(&url_string).unwrap();
        Ok(Response::with((::iron::status::MovedPermanently, ::iron::modifiers::Redirect(new_url))))
    })
}

pub fn handle_login(request: &mut Request) -> IronResult<Response> {
    let conn = request.get::<Read<::db::PostgresPool>>().unwrap().get().unwrap();
    let params = request.get::<::params::Params>().unwrap();
    let params = request.get::<::params::Params>().unwrap();
    let uname = &String::from_value(&params["uname"]).unwrap() as &str;
    let pass = &String::from_value(&params["pass"]).unwrap() as &str;
    
    let mut hasher = ::sha2::Sha256::new();
    hasher.input(pass.as_bytes());
    let pass = format!("{:?}", &hasher.result().as_slice()).to_owned();

    let valid: i64 = ::db::valid_mod_login(&conn, uname, &pass).unwrap().get(0).get(0);
    if valid == 1 {
        let mut hasher = ::sha2::Sha256::new();
        hasher.input(&request.remote_addr.ip().to_string().as_bytes());
        let ip = format!("{:?}", hasher.result().as_slice()).to_owned();

        {
            let session_lock = request.get::<State<::sessions::SessionStore<String, String>>>().unwrap();
            let mut table = session_lock.write().unwrap();
            table.insert(ip.to_owned(), uname.to_string());
        }

        let mut url_string = String::new();
        url_string.push_str(&format!("{}", request.url.scheme()));
        url_string.push_str("://");
        url_string.push_str(&format!("{}", request.url.host()));
        url_string.push(':');
        url_string.push_str(&format!("{}", request.url.port()));

        let mut prev_url = request.url.path().clone();
        prev_url.pop();
        prev_url.push("mod");
        prev_url.push("0");

        for s in prev_url {
            url_string.push('/');
            url_string.push_str(s);
        };

        let new_url = iron::Url::parse(&url_string).unwrap();
        return Ok(Response::with((::iron::status::MovedPermanently, ::iron::modifiers::Redirect(new_url))))
    }

    Ok(Response::with(("text/html".parse::<iron::mime::Mime>().unwrap(), iron::status::Ok, "Incorrect username or password")))
}
