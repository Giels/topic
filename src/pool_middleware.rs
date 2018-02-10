//! Based on https://github.com/SkylerLipthay/iron-postgres-middleware
//!
//! The MIT License (MIT)
//! Copyright (c) 2015 Martins Polakovs
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

use iron::prelude::*;
use iron::{typemap, BeforeMiddleware};
use std::sync::Arc;

use core::marker::Reflect;

use r2d2;

/// Iron middleware that allows for r2d2 connections within requests.
pub struct PoolMiddleware<M>
    where M: r2d2::ManageConnection {

    /// A pool of database connections that are shared between requests.
    pub pool: Arc<r2d2::Pool<M>>,
}

pub struct Value<M>
    where M: r2d2::ManageConnection + Reflect
{
    value: Arc<r2d2::Pool<M>>
}

impl<M> typemap::Key for PoolMiddleware<M>
    where M: r2d2::ManageConnection + Reflect,
    <M as r2d2::ManageConnection>::Connection: Reflect
{
        type Value = Value<M>;
}

impl<M> PoolMiddleware<M>
    where M: r2d2::ManageConnection {

    /// Creates a new pooled connection to the given server.
    /// 
    /// # Panics
    ///
    /// Any errors when connecting to the database.
    pub fn new(manager: M, pool_size: u32) -> Result<PoolMiddleware<M>, r2d2::InitializationError> {
        let config = r2d2::Config::builder()
            .error_handler(Box::new(r2d2::LoggingErrorHandler))
            .pool_size(pool_size)
            .build();
        let pool = Arc::new(try!(r2d2::Pool::new(config, manager)));

        Ok(PoolMiddleware {
            pool: pool
        })
    }
}

impl<M> BeforeMiddleware for PoolMiddleware<M> 
    where M: r2d2::ManageConnection + Reflect,
    <M as r2d2::ManageConnection>::Connection: Reflect
{
    fn before(&self, request: &mut Request) -> IronResult<()> {
        request.extensions.insert::<PoolMiddleware<M>>(Value { value: self.pool.clone() });
        Ok(())
    }
}

/// Adds a method to requests to get a database connection.
///
/// # Example
///
/// ```ignore
/// fn handler(req: &mut Request) -> IronResult<Response> {
/// let conn = req.get_conn();
/// con.execute("INSERT INTO foo (bar) VALUES ($1)", &[&1i32]).unwrap();
///
/// Ok(Response::with((status::Ok, resp_str)))
/// }
/// ```
pub trait PoolOwner<M> where M: r2d2::ManageConnection {
    /// Returns a pooled connection to the postgresql database. The connection is returned to
    /// the pool when the pooled connection is dropped.
    fn get_conn(&self) -> r2d2::PooledConnection<M>;
}

impl<'a, 'b, M> PoolOwner<M> for Request<'a, 'b>
    where M: r2d2::ManageConnection + Reflect,
    <M as r2d2::ManageConnection>::Connection: Reflect
{
    fn get_conn(&self) -> r2d2::PooledConnection<M> {
        let &Value { value: ref pool } = self.extensions.get::<PoolMiddleware<M>>().unwrap();
        return pool.get().unwrap();
    }
}
