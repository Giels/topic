use iron::prelude::*;
use iron;
use ::sha2::Digest;
use ::sha2::Sha256;

use postgres::types::ToSql;
use postgres;

use std;

use hoedown as md;
use hoedown::renderer::Render;

use ::params::FromValue;

use ::router::Router;

use persistent::{State};

use std::cmp;

use regex::Regex;
use regex::Captures;

use config;

use chrono::NaiveDateTime;
use chrono::Utc;

fn make_new_post(request: &mut Request, thread: &str, date: Option<&NaiveDateTime>, params: &::params::Map, conn: &super::PostgresConnection) -> Result<(), String> {
    let mut hasher = ::sha2::Sha256::new();
    hasher.input(&String::from_value(&params["ip"]).unwrap().as_bytes());
    let ip = format!("{:?}", hasher.result().as_slice()).to_owned();

    if super::banned(conn, &ip) {
        Err(format!("Users with this IP are banned: {}", super::ban_reason(conn, &ip)).to_string())
    } else {
        let bump = params.contains_key("bump");
        let name = &String::from_value(&params["name"]).unwrap() as &str;
        let mut hasher = Sha256::new();
        let password = &String::from_value(&params["password"]).unwrap() as &str;
        hasher.input(password.as_bytes());
        let password = format!("{:?}", &hasher.result().as_slice()).to_owned();

        let content = &String::from_value(&params["content"]).unwrap() as &str;
        let mut md_options = md::Extension::empty();
        md_options.insert(md::MATH);
        md_options.insert(md::NO_INTRA_EMPHASIS);
        md_options.insert(md::SPACE_HEADERS);
        md_options.insert(md::DISABLE_INDENTED_CODE);
        md_options.insert(md::FOOTNOTES);
        md_options.insert(md::STRIKETHROUGH);
        md_options.insert(md::TABLES);
        md_options.insert(md::HIGHLIGHT);
        let mut html_options = md::renderer::html::Flags::empty();
        html_options.insert(md::renderer::html::ESCAPE);
        let mut html = md::renderer::html::Html::new(html_options, 0);
        let render_output = html.render(&md::Markdown::new(&content as &str).extensions(md_options));
        let content = render_output.to_str().unwrap();
        let re = Regex::new(r"@(?P<post>[0-9]+)").unwrap();

        let ppp = i64::from_value(&params["ppp"]).unwrap();

        let content = &(&re).replace_all(content, (|x: &Captures| {
            let post_num = x.get(1).unwrap().as_str();
            let page = cmp::max((str::parse::<u64>(post_num).unwrap() as i64) / ppp, 0);
            format!("<a href=\"{0}#{1}\">@{1}</a>", page, post_num)
        })) as &str;

        let thread = str::parse::<i32>(thread).unwrap();

        let number = ::db::get_last_post_number(conn, thread).unwrap();

        let number = if number.len() > 0 {
            1 + number.get(0).get::<_,i32>(0)
        } else {
            0
        };

        let now;
        if date.is_none() {
            now = Utc::now().naive_utc();
        } else {
            now = *date.unwrap();
        }
        let session_lock = request.get::<State<::sessions::SessionStore<String, String>>>().unwrap();
        let session = session_lock.read().unwrap();
    
        let mut hasher = ::sha2::Sha256::new();
        hasher.input(&request.remote_addr.ip().to_string().as_bytes());
        let ip = format!("{:?}", hasher.result().as_slice()).to_owned();

        let _ = session.get(&ip.to_owned()).map(|uname| {
            // Admin post type is 2, site owner post type is 1. Regular users are type 0.
            ::db::insert_special_post(conn, thread, number as i32, uname, content, &password, bump, 2, &ip, &now)
        }).unwrap_or_else(|| {
            ::db::insert_post(conn, thread, number as i32, name, content, &password, bump, &ip, &now)
        });

        if bump {
            let _ = ::db::bump_thread(conn, thread, &now);
        }

        Ok(())
    }
}

pub fn handle_new_post(request: &mut Request) -> IronResult<Response> {
    let mut params = request.get::<::params::Params>().unwrap();
    let conn = request.get::<::persistent::Read<::db::PostgresPool>>().unwrap().get().unwrap();
    let config = request.get::<::persistent::Read<config::Config>>().unwrap();

    let thread = &{ get_var!(request, "thread").to_owned() } as &str;

    let thread_id = str::parse::<i32>(thread).unwrap_or(-1);
    if thread_id < 0 {
        return super::handle_404();
    }

    let nposts = ::db::get_num_posts(&conn, thread_id).unwrap().get(0).get::<_,i64>(0);
    let new_page = nposts / (config.site.posts_per_page as i64);
    let ip = request.remote_addr.ip().to_string();

    params.insert("ip".to_string(), ::params::Value::String(ip.to_owned()));
    params.insert("ppp".to_string(), ::params::Value::U64(config.site.posts_per_page));

    match make_new_post(request, thread, None, &params, &conn) {
        Err(e) => Ok(Response::with((::iron::status::Ok, e))),
        Ok(_) => {
            let path = request.url.path();
            let board_frag = path.get(0).unwrap();
            let thread_frag = path.get(2).unwrap();
            let new_url = iron::Url::parse(&format!("{}://{}:{}/{}/t/{}/{}", request.url.scheme(), request.url.host(), request.url.port(), board_frag, thread_frag, new_page)).unwrap();
            Ok(Response::with((::iron::status::MovedPermanently, ::iron::modifiers::Redirect(new_url))))
        }
    }
}

pub fn handle_new_thread(request: &mut Request) -> IronResult<Response> {
    let mut params = request.get::<::params::Params>().unwrap();
    let conn = request.get::<::persistent::Read<::db::PostgresPool>>().unwrap().get().unwrap();
    let config = request.get::<::persistent::Read<config::Config>>().unwrap();

    params.insert("bump".to_string(), ::params::Value::String("on".to_string()));
    let ip = request.remote_addr.ip().to_string();
    params.insert("ip".to_string(), ::params::Value::String(ip.to_owned()));
    params.insert("ppp".to_string(), ::params::Value::U64(config.site.posts_per_page));

    let board = { get_var!(request, "board").to_owned() };

    let title = String::from_value(&params["title"]).unwrap();

    let now = Utc::now().naive_utc();
    let thread = ::db::insert_thread(&conn, &title as &str, &board as &str, &now).unwrap().get(0).get::<_,i32>(0);
    let thread = &format!("{}", thread) as &str;

    match make_new_post(request, thread, Some(&now), &params, &conn) {
        Err(e) => { Ok(Response::with((::iron::status::Ok, e))) },
        Ok(_) => {
            let path = request.url.path();
            let board_frag = path.get(0).unwrap();
            let new_url = iron::Url::parse(&format!("{}://{}:{}/{}/t/{}/{}", request.url.scheme(), request.url.host(), request.url.port(), board_frag, thread, 0)).unwrap();
            Ok(Response::with((::iron::status::MovedPermanently, ::iron::modifiers::Redirect(new_url))))
        }
    }
}
