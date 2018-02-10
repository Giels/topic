use std::collections::HashMap;
use iron::typemap;

use core;

pub struct SessionStore<A, B> where A: 'static + core::any::Any + core::cmp::Eq + core::hash::Hash,
                                    B: 'static + core::any::Any
{
    pub store: HashMap<A, B>
}

impl<A, B> typemap::Key for SessionStore<A, B> where A: 'static + core::any::Any + core::cmp::Eq + core::hash::Hash,
                                                    B: 'static + core::any::Any
{
    type Value = HashMap<A, B>;
}

impl<A, B> SessionStore<A, B> where A: 'static + core::any::Any + core::cmp::Eq + core::hash::Hash,
                                    B: 'static + core::any::Any
{
    pub fn new() -> Self {
        SessionStore { store: HashMap::new() }
    }
}
