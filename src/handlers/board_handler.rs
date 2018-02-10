use iron::prelude::*;
use std::collections::HashMap;
use chrono::NaiveDateTime;
use postgres_array::Array as PgArray;

use persistent::Read;

use std::str;
use std::cmp;

use ::router::Router;

use config;

#[derive(RustcEncodable)]
struct Page {
    num: i64,
    link: bool,
}

#[derive(RustcEncodable)]
struct ThreadInfo {
    stickies: Vec<Thread>,
    threads: Vec<Thread>,
    rules: Vec<String>,
    board: String,
    link: String,
    pages: Vec<Page>,
}

#[derive(RustcEncodable)]
struct Thread {
    uid: i64,
    title: String,
    mdate: String,
    cdate: String,
    post_count: i64
}

pub fn handle_board(request: &mut Request) -> IronResult<Response> {
    let conn = request.get::<Read<::db::PostgresPool>>().unwrap().get().unwrap();
    let config = request.get::<Read<config::Config>>().unwrap();

    let board = get_var!(request, "board");
    let page = get_var!(request, "page");
    let parsed_page = str::parse::<i64>(page).unwrap_or(-1);

    let thread_valid = ::db::get_board_info(&conn, &board as &str).unwrap().len() == 1;
    let nthreads = ::db::get_num_threads(&conn, &board as &str).unwrap().get(0).get::<_,i64>(0);
    let page_valid = parsed_page >= 0 && parsed_page <= (cmp::max(nthreads - 1, 0) / (config.site.threads_per_page as i64));
    let valid = page_valid && thread_valid;

    if !valid {
        return super::handle_404();
    }

    let npages = (nthreads - 1) / (config.site.threads_per_page as i64) + 1;

    let offset = parsed_page * (config.site.threads_per_page as i64);
    let limit = config.site.threads_per_page as i64;

    let sticky_thread_rows = ::db::get_sticky_threads(&conn, board).unwrap();
    let thread_rows = ::db::get_normal_threads(&conn, board, offset, limit).unwrap();

    let mut data = HashMap::new();

    let mut sticky_threads = vec![];
    let mut threads = vec![];

    for s in sticky_thread_rows.iter() {
        let uid = s.get("uid");
        let cdate = ::db::get_thread_cdate(&conn, uid).unwrap().get(0).get::<_,NaiveDateTime>(0).to_string();
        let mdate = ::db::get_thread_mdate(&conn, uid).unwrap().get(0).get::<_,NaiveDateTime>(0).to_string();
        let pc = ::db::get_num_posts(&conn, uid).unwrap().get(0).get(0);

        sticky_threads.push(Thread { uid: uid, title: s.get("title"), mdate: mdate, cdate: cdate, post_count: pc });
    }

    for s in thread_rows.iter() {
        let uid = s.get("uid");
        let cdate = ::db::get_thread_cdate(&conn, uid).unwrap().get(0).get::<_,NaiveDateTime>(0).to_string();
        let mdate = ::db::get_thread_mdate(&conn, uid).unwrap().get(0).get::<_,NaiveDateTime>(0).to_string();
        let pc = ::db::get_num_posts(&conn, uid).unwrap().get(0).get(0);

        threads.push(Thread { uid: uid, title: s.get("title"), mdate: mdate, cdate: cdate, post_count: pc });
    }

    let info_rows = ::db::get_board_info(&conn, board).unwrap();
    let rules: PgArray<String> = info_rows.get(0).get("rules");
    let name: String = info_rows.get(0).get("name");

    let payload = ThreadInfo { stickies: sticky_threads, threads: threads, rules: rules.into_inner(), board: name, link: board.to_string(), pages: (0..npages).map(|x| Page { num: x, link: (x != parsed_page) }).collect() };

    data.insert("payload", payload);

    let mut bytes = vec![];
    let template = ::mustache::compile_path("assets/board.tpl").unwrap();
    template.render(&mut bytes, &data).unwrap();

    Ok(Response::with(("text/html".parse::<::iron::mime::Mime>().unwrap(), ::iron::status::Ok,
    str::from_utf8(&bytes).unwrap())))
}
