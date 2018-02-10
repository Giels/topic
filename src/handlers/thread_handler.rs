use std::str;
use std::cmp;

use iron::prelude::*;
use std::collections::HashMap;
use chrono::NaiveDateTime;
use postgres_array::Array as PgArray;

use persistent::Read;

use config;

use ::router::Router;

#[derive(RustcEncodable)]
struct Page {
    num: i64,
    link: bool,
}

#[derive(RustcEncodable)]
struct Post {
    name: String,
    date: String,
    content: String,
    bump: bool,
    number: i64,
    uid: i64,
    typ: i32,
}

#[derive(RustcEncodable)]
struct PostInfo {
    posts: Vec<Post>,
    rules: Vec<String>,
    board: String,
    here: String,
    pages: Vec<Page>,
}

pub fn handle_thread(request: &mut Request) -> IronResult<Response> {
    let conn = request.get::<Read<::db::PostgresPool>>().unwrap().get().unwrap();
    let config = request.get::<Read<config::Config>>().unwrap();

    let thread = get_var!(request, "thread");
    let board = get_var!(request, "board");
    let page = str::parse::<i64>(get_var!(request, "page")).unwrap_or(-1);

    let parsed_id = thread.parse::<i64>().unwrap_or(-1);
    let nposts;

    if parsed_id >= 0 && page >= 0 {
        nposts = ::db::get_num_posts(&conn, parsed_id).unwrap().get(0).get::<_,i64>(0);
        let valid = (nposts > 0) && page <= (cmp::max(nposts - 1, 0) / (config.site.posts_per_page as i64));

        if !valid {
            return super::handle_404();
        }
    } else { return super::handle_404(); }

    let npages = (nposts - 1) / (config.site.posts_per_page as i64) + 1;

    let offset = (config.site.posts_per_page as i64) * page;
    let limit = config.site.posts_per_page as i64;

    let post_rows = ::db::get_posts(&conn, parsed_id, offset, limit).unwrap();
    let info_rows = ::db::get_board_info(&conn, &board as &str).unwrap();
    let rules: PgArray<String> = info_rows.get(0).get("rules");
    let board_name: String = info_rows.get(0).get("name");

    let mut posts: Vec<Post> = vec![];

    for r in post_rows.iter() {
        let n = r.get("name");
        let d = r.get::<_,NaiveDateTime>(2).to_string();
        let c = r.get::<_,String>("content");
        let b = r.get::<_,bool>("bump");
        let number = r.get::<_,i64>("number");
        let u = r.get::<_,i64>("uid");
        let t = r.get::<_,i32>("type");
        posts.push(Post { name: n, date: d, content: c, bump: b, number: number, uid: u, typ: t });
    }

    let payload = PostInfo { posts: posts, board: board_name, rules: rules.into_inner(), here: format!("{}/t/{}/{}", board, thread, page), pages: (0..npages).map(|x| Page { num: x, link: (x != page) }).collect() };

    let mut data = HashMap::new();
    data.insert("payload", payload);

    let mut bytes = vec![];
    let template = ::mustache::compile_path("assets/thread.tpl").unwrap();
    template.render(&mut bytes, &data).unwrap();

    Ok(Response::with(("text/html".parse::<::iron::mime::Mime>().unwrap(), ::iron::status::Ok, str::from_utf8(&bytes).unwrap())))
}
