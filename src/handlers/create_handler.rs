use iron::prelude::*;
use ::sha2::Digest;
use ::sha2::Sha256;

use hoedown as md;
use hoedown::renderer::Render;

use ::params::FromValue;

use ::router::Router;

use std::str;
use std::cmp;

use regex::Regex;
use regex::Captures;

use config;

use chrono::NaiveDateTime;
use chrono::UTC;

fn make_new_post(thread: &str, date: Option<&NaiveDateTime>, params: &::params::Map, conn: &super::PostgresConnection) -> Result<(), String> {
    let mut hasher = ::sha2::Sha256::new();
    hasher.input_str(&String::from_value(&params["ip"]).unwrap() as &str);
    let ip = &*hasher.result_str();

    if super::banned(conn, ip) {
        Err(format!("Users with this IP are banned: {}", super::ban_reason(conn, ip)).to_string())
    } else {
        let bump = params.contains_key("bump");
        let name = &String::from_value(&params["name"]).unwrap() as &str;
        let mut hasher = Sha256::new();
        let password = &String::from_value(&params["password"]).unwrap() as &str;
        hasher.input_str(password);
        let password = &hasher.result_str() as &str;

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
            let post_num = x.at(1).unwrap();
            let page = cmp::max((str::parse::<u64>(post_num).unwrap() as i64) / ppp, 0);
            format!("<a href=\"{0}#{1}\">@{1}</a>", page, post_num)
        })) as &str;

        let thread = str::parse::<i64>(thread).unwrap();

        let number = ::db::get_last_post_number(conn, thread).unwrap();

        let number = if number.len() > 0 {
            1 + number.get(0).get::<_,i64>(0)
        } else {
            0
        };

        let now;
        if date.is_none() {
            now = UTC::now().naive_utc();
        } else {
            now = *date.unwrap();
        }
        let _ = ::db::insert_post(conn, thread, number, name, content, password, bump, ip, &now);

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

    let thread = &get_var!(request, "thread") as &str;

    let thread_id = str::parse::<i64>(thread).unwrap_or(-1);
    if thread_id < 0 {
        return super::handle_404();
    }

    let nposts = ::db::get_num_posts(&conn, thread_id).unwrap().get(0).get::<_,i64>(0);
    let new_page = nposts / (config.site.posts_per_page as i64);
    let ip = request.remote_addr.ip().to_string();

    params.insert("ip".to_string(), ::params::Value::String(ip.to_owned()));
    params.insert("ppp".to_string(), ::params::Value::U64(config.site.posts_per_page));

    match make_new_post(thread, None, &params, &conn) {
        Err(e) => Ok(Response::with((::iron::status::Ok, e))),
        Ok(_) => {
            let mut previous_url = request.url.clone();
            previous_url.path.pop();
            previous_url.path.pop();
            previous_url.path.push(format!("{}", new_page));
            Ok(Response::with((::iron::status::MovedPermanently, ::iron::modifiers::Redirect(previous_url))))
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

    let board = get_var!(request, "board");

    let title = String::from_value(&params["title"]).unwrap();

    let now = UTC::now().naive_utc();
    let thread = ::db::insert_thread(&conn, &title as &str, &board as &str, &now).unwrap().get(0).get::<_,i64>(0);
    let thread = &format!("{}", thread) as &str;

    match make_new_post(thread, Some(&now), &params, &conn) {
        Err(e) => Ok(Response::with((::iron::status::Ok, e))),
        Ok(_) => {
            let mut next_url = request.url.clone();
            next_url.path.pop();
            next_url.path.push("t".to_string());
            next_url.path.push(thread.to_string());
            next_url.path.push("0".to_string());
            Ok(Response::with((::iron::status::MovedPermanently, ::iron::modifiers::Redirect(next_url))))
        }
    }
}
