use r2d2;
use r2d2_postgres::{PostgresConnectionManager};

use iron::prelude::*;
use iron;

use db;

use iron::response::Response as IronResponse;

fn handle_404() -> IronResult<IronResponse> {
    Ok(IronResponse::with((iron::status::NotFound, "Page not found")))
}

macro_rules! get_var {
    ( $req:ident, $var:expr ) => ( $req.extensions.get::<Router>().unwrap().find($var).unwrap(); );
}

fn banned(conn: &PostgresConnection, ip: &str) -> bool {
    db::is_banned(conn, ip).unwrap().get(0).get::<_,bool>(0)
}

fn ban_reason(conn: &PostgresConnection, ip: &str) -> String {
    let rows = db::get_ban_reason(conn, ip).unwrap();
    if rows.len() > 0 {
        rows.get(0).get(0)
    } else {
        "No ban on record, please contact an admin.".to_string()
    }
}

pub type PostgresConnection = r2d2::PooledConnection<PostgresConnectionManager>;

mod main_handler;
mod board_handler;
mod thread_handler;
mod create_handler;
mod delete_handler;
mod resource_handler;
mod admin_handler;

pub use self::main_handler::*;
pub use self::board_handler::*;
pub use self::thread_handler::*;
pub use self::create_handler::*;
pub use self::delete_handler::*;
pub use self::resource_handler::*;
pub use self::admin_handler::*;
