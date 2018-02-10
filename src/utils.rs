extern crate r2d2;
extern crate r2d2_postgres;

extern crate iron;

use db::r2d2_postgres::{PostgresConnectionManager};

use iron::prelude::*;

macro_rules! get_var {
    ( $req:ident, $var:expr ) => ( $req.extensions.get::<Router>().unwrap().find($var).unwrap(); );
}

pub fn handle_404(_: Error) -> IronResult<IronResponse> {
    Ok(Response::with((iron::status::NotFound, "Page not found")));
}

type RouteHandler = (Fn(&mut Request) -> IronResult<IronResponse>);
type PostgresPool = r2d2::Pool<PostgresConnectionManager>;


