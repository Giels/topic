extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate iron;

use iron::typemap;

use chrono::NaiveDateTime;

use db::r2d2_postgres::{PostgresConnectionManager, SslMode};

use db::postgres::Connection;

use db::postgres::Result as PgResult;
use db::postgres::rows::Rows as PgRows;

use db::postgres::types::ToSql;

use config::DbConfig;

pub struct PostgresPool {
    pub pool: r2d2::Pool<PostgresConnectionManager>
}

pub fn postgres_pool(conf: &DbConfig) -> r2d2::Pool<r2d2_postgres::PostgresConnectionManager> {
    let manager = PostgresConnectionManager::new(
        &format!("postgres://{}:{}@{}:{}/{}",
                 conf.user,
                 conf.password,
                 conf.host,
                 conf.port,
                 conf.db) as &str,
                 SslMode::None).unwrap();
    let config = r2d2::Config::builder()
        .error_handler(Box::new(r2d2::LoggingErrorHandler))
        .pool_size(10)
        .build();
    (r2d2::Pool::new(config, manager)).unwrap()
}

impl typemap::Key for PostgresPool { type Value = r2d2::Pool<PostgresConnectionManager>; }

//pub fn postgres_middleware(conf: &ServerConf) -> Result<PoolMiddleware<PostgresConnectionManager>, r2d2::InitializationError> {
//    PoolMiddleware::new(PostgresConnectionManager::new(
//                            &format!("postgres://{}:{}@{}:{}/{}",
//                                conf.user,
//                                conf.password,
//                                conf.host,
//                                conf.port,
//                                conf.db) as &str,
//                            SslMode::None).unwrap(),
//                            10)
//}
//
macro_rules! def_queries {
    ( $($prep_name:ident ($($arg_name:ident : $args:ty),*) => $prep_val:expr);*; ) => (
        $(pub fn $prep_name<'a>(conn: &'a Connection, $($arg_name : $args),*) -> PgResult<PgRows<'a>> {
            let data = [$(&$arg_name as &ToSql),*];
            conn.query($prep_val, &data[..])
        })*
    );
}

macro_rules! def_updates {
    ( $($prep_name:ident ($($arg_name:ident : $args:ty),*) => $prep_val:expr);*; ) => (
        $(pub fn $prep_name(conn: &Connection, $($arg_name : $args),*) -> PgResult<u64> {
            let data = [$(&$arg_name as &ToSql),*];
            conn.execute($prep_val, &data[..])
        })*
    );
}

def_updates! {
    insert_post(tid: i64, number: i64, name: &str, content: &str, password: &str, bump: bool, ip: &str, date: &NaiveDateTime) => "INSERT INTO posts (tid, number, name, content, password, bump, ip, cdate) VALUES($1, $2, $3, $4, $5, $6, $7, $8)";
    insert_special_post(tid: i64, number: i64, name: &str, content: &str, password: &str, bump: bool, typ: i64, ip: &str) => "INSERT INTO posts (tid, number, name, content, password, bump, type, ip) VALUES($1, $2, $3, $4, $5, $6, $7, $8)";

    bump_thread(tid: i64, bdate: &NaiveDateTime) => "UPDATE threads SET bdate = $2 WHERE uid = $1";

    report_post(ip: &str, uid: i64, reason: &str) => "INSERT INTO reports (ip, post, reason) VALUES ($1, $2, $3)";

    delete_post(uid: i64) => "UPDATE posts SET deleted = 'true' WHERE uid = $1";
    delete_thread(uid: i64) => "DELETE FROM threads WHERE uid = $1";
}

def_queries! {
    valid_mod_login(uname: &str, pass: &str) => "SELECT count(*) FROM mods WHERE uname = $1 AND pass = $2";

    get_mod_powers(uname: &str) => "SELECT can_delete, can_ban, can_sticky, can_edit FROM mods WHERE unmae = $1";

    insert_thread(title: &str, link: &str, bdate: &NaiveDateTime) => "INSERT INTO threads (title, bid, bdate) VALUES($1, (SELECT uid FROM boards WHERE link = $2 LIMIT 1), $3) RETURNING uid";

    get_categories() => "SELECT DISTINCT ON (name) name FROM categories ORDER BY name DESC";
    get_boards_in_cat(name: &str) => "SELECT link, name FROM boards WHERE uid IN (SELECT bid FROM categories WHERE name = $1) ORDER BY uid";

    get_boards() => "SELECT link, name FROM boards ORDER BY uid";
    get_board_info(link: &str) => "SELECT name, notice, rules FROM boards WHERE link = $1";

    get_num_threads(link: &str) => "SELECT count(*) FROM threads WHERE bid IN (SELECT uid FROM boards WHERE link = $1)";

    get_threads(link: &str, offset: i64, limit: i64) => "SELECT title, uid FROM threads WHERE bid IN (SELECT uid FROM boards WHERE link = $1) ORDER BY uid OFFSET $2 LIMIT $3";
    get_sticky_threads(link: &str) => "SELECT title, uid FROM threads WHERE bid IN (SELECT uid FROM boards WHERE link = $1) AND sticky = 'true' ORDER BY uid";
    get_normal_threads(link: &str, offset: i64, limit: i64) => "SELECT uid, title FROM threads WHERE bid IN (SELECT uid FROM boards WHERE link = $1) AND sticky = 'f' ORDER BY bdate DESC OFFSET $2 LIMIT $3";

    get_thread_cdate(tid: i64) => "SELECT DATE_TRUNC('second', cdate) FROM posts WHERE tid = $1 ORDER BY cdate LIMIT 1";
    get_thread_mdate(tid: i64) => "SELECT DATE_TRUNC('second', cdate) FROM posts WHERE tid = $1 ORDER BY cdate DESC LIMIT 1";

    get_posts(tid: i64, offset: i64, limit: i64) => "SELECT uid, name, DATE_TRUNC('second', cdate), content, password, bump, number, type FROM posts WHERE tid = $1 AND deleted = 'false' ORDER BY cdate OFFSET $2 LIMIT $3";
    get_first_post_id(tid: i64) => "SELECT uid FROM posts WHERE tid = $1 LIMIT 1";
    get_last_post_number(tid: i64) => "SELECT number FROM posts WHERE tid = $1 ORDER BY number DESC LIMIT 1";
    get_post_password(uid: i64) => "SELECT password FROM posts WHERE uid = $1";

    get_num_posts(tid: i64) => "SELECT count(*) FROM posts WHERE tid = $1";

    is_banned(ip: &str) => "SELECT exists(SELECT 1 FROM bans WHERE ip = $1)";
    get_ban_reason(ip: &str) => "SELECT reason FROM bans WHERE ip = $1";
}
