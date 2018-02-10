use iron::prelude::*;

use ::params::FromValue;
use ::sha2::Digest;
use ::sha2::Sha256;

use ::router::Router;

use config;

pub fn handle_report_delete_post(request: &mut Request) -> IronResult<Response> {
    let params = request.get::<::params::Params>().unwrap();
    let conn = request.get::<::persistent::Read<::db::PostgresPool>>().unwrap().get().unwrap();
    let config = request.get::<::persistent::Read<config::Config>>().unwrap();

    let thread = str::parse::<i64>(&get_var!(request, "thread") as &str).unwrap_or(-1);
    if thread >= 0 {
        let valid = ::db::get_num_posts(&conn, thread).unwrap().get(0).get::<_,i64>(0) > 0;
        if !valid {
            return super::handle_404();
        }
    } else { return super::handle_404(); }

    let mut selected = vec![];
    for k in params.keys() {
        if k.starts_with("p_") {
            selected.push(str::parse::<i64>(&k[2..]).unwrap());
        }
    }

    let ip = &request.remote_addr.ip().to_string() as &str;
    let mut hasher = Sha256::new();
    hasher.input_str(ip);
    let ip = &hasher.result_str() as &str;

    let action = &String::from_value(&params["action"]).unwrap() as &str;

    let mut hasher = Sha256::new();
    let password = &String::from_value(&params["password"]).unwrap() as &str;
    let invalid = str::len(password) == 0;
    hasher.input_str(password);
    let password = &hasher.result_str() as &str;

    let mut thread_deleted = false;

    let nposts = ::db::get_num_posts(&conn, thread).unwrap().get(0).get::<_,i64>(0);
    let new_page = (nposts - 2) / (config.site.posts_per_page as i64);

    if action == "delete" {
        if !invalid {
            for &s in selected.iter() {
                if ::db::get_post_password(&conn, s).unwrap().get(0).get::<_,String>(0) != password {
                    continue;
                } else {
                    if ::db::get_first_post_id(&conn, thread).unwrap().get(0).get::<_,i64>(0) == s {
                        let _ = ::db::delete_thread(&conn, thread);
                        thread_deleted = true;
                        break;
                    } else {
                        let _ = ::db::delete_post(&conn, s);
                    }
                }
            }
        }
    } else { // action == "report"
        for &s in selected.iter() {
            let reason = &String::from_value(&params["reason"]).unwrap() as &str;
            let _ = ::db::report_post(&conn, ip, s, reason);
        }
    }

    let mut previous_url = request.url.clone();
    previous_url.path.pop();
    previous_url.path.pop();

    if thread_deleted {
        previous_url.path.pop();
        previous_url.path.pop();
        previous_url.path.push("0".to_string());
    } else {
        previous_url.path.push(format!("{}", new_page));
    }

    Ok(Response::with((::iron::status::MovedPermanently, ::iron::modifiers::Redirect(previous_url))))
}
