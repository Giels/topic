use std::str;
use iron::prelude::*;
use std::collections::HashMap;

use ::persistent::Read;

use config;

#[derive(RustcEncodable)]
struct Board {
    link: String,
    name: String,
    thread_count: i64,
}

#[derive(RustcEncodable)]
struct BoardCat {
    name: String,
    boards: Vec<Board>,
}

#[derive(RustcEncodable)]
struct SiteInfo {
    cats: Vec<BoardCat>,
    name: String,
}

pub fn redirect_page0(request: &mut Request) -> IronResult<Response> {
    let mut url = request.url.clone();
    url.path.push("0".to_string());
    Ok(Response::with((::iron::status::MovedPermanently, ::iron::modifiers::Redirect(url))))
}

pub fn redirect_up(request: &mut Request) -> IronResult<Response> {
    let mut url = request.url.clone();
    url.path.pop();
    Ok(Response::with((::iron::status::MovedPermanently, ::iron::modifiers::Redirect(url))))
}

pub fn handle_main(request: &mut Request) -> IronResult<Response> {
    let conn = request.get::<Read<::db::PostgresPool>>().unwrap().get().unwrap();
    let config = request.get::<Read<config::Config>>().unwrap();
    let cat_rows = ::db::get_categories(&conn).unwrap();

    let mut cats: Vec<String> = vec![];
    for r in cat_rows.iter() {
        cats.push(r.get(0)); //the category name
    }

    let mut board_rows = vec![];
    for c in &cats {
        board_rows.push(::db::get_boards_in_cat(&conn, &c).unwrap());
    }

    let mut data = HashMap::new();
    let mut board_cats = vec![];

    for (b, c) in board_rows.iter().zip(&cats) {
        board_cats.push(BoardCat { name: c.to_owned(), boards: vec![] });
        let last_idx = (&board_cats).len() - 1;
        let current_cat = &mut board_cats[last_idx];
        for r in b.iter() {
            let tc = ::db::get_num_threads(&conn, (&r.get::<_,String>("link") as &str)).unwrap().get(0).get(0);
            current_cat.boards.push(Board { link: r.get::<_,String>("link").to_owned(), name: r.get::<_,String>("name").to_owned(), thread_count: tc });
        }
    };

    data.insert("payload", SiteInfo { cats: board_cats, name: config.site.name.to_owned() });

    let mut bytes = vec![];
    let template = ::mustache::compile_path("assets/main.tpl").unwrap();
    template.render(&mut bytes, &data).unwrap();

    Ok(Response::with(("text/html".parse::<::iron::mime::Mime>().unwrap(), ::iron::status::Ok,
    str::from_utf8(&bytes).unwrap())))
}
