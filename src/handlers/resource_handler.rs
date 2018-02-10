use iron::prelude::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use ::router::Router;

const WHITELIST : &'static [&'static str] =
&[
 "skeleton.css",
 "main.css",
 "board.css",
 "thread.css",

 "main.js",
];

fn path_prefix(s : &str) -> &str {
    match s {
        "css" => "style",
        "png" => "img",
        "jpg" => "img",
        "js" => "js",
        _ => "misc"
    }
}

fn res_type(s : &str) -> ::iron::mime::Mime {
    let type_str = match s {
        "css" => "text/css",
        "html" => "text/html",
        "js" => "text/javascript",
        _ => "test/plain"
    };

    type_str.parse::<::iron::mime::Mime>().unwrap()
}

pub fn handle_resource(request: &mut Request) -> IronResult<Response> {
//    let params = request.get::<::params::Params>().unwrap();
    let resource = get_var!(request, "res");
    let mut res_candidates = WHITELIST.to_vec();
    res_candidates.retain(|x| x.ends_with(&resource));
    if res_candidates.len() != 0 {
        let res_path = Path::new(res_candidates[0]);
        let ext = res_path.extension().unwrap().to_str().unwrap();
        let path_prefix = Path::new(path_prefix(&ext));
        let res_type = res_type(&ext);
        let res_path = path_prefix.join(Path::new(resource));
        let status =
            File::open(res_path.to_str().unwrap()).and_then(|mut f| {
                let mut s = String::new();
                f.read_to_string(&mut s)
                    .and_then(|_|
                              Ok(Response::with((res_type, ::iron::status::Ok, s))))
            });
        match status {
            Ok(x) => Ok(x),
            Err(_) => super::handle_404()
        }
    } else {
        super::handle_404()
    }


}
