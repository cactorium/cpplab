#![feature(time2, io, custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate hyper;

extern crate serde;
extern crate serde_json;

mod script;
mod planets;

use std::collections::HashMap;
use std::io::Read;
use std::time::SystemTime;

use hyper::Server;
use hyper::net::Fresh;
use hyper::server::{Request, Response, Handler};

use hyper::uri::RequestUri::*;
use hyper::method::Method;

use script::exec_cpp;
use script::ExecResult;

static HOME_PAGE: &'static str = "
HELLO
THERE IS NO HOME PAGE YET
THIS WILL BE FIXED EVENTUALLY
";

static ERROR_PAGE: &'static str = "
PAGE NOT FOUND
PLEASE CLICK HARDER
";

#[derive(Serialize, Deserialize)]
struct IoFail {
    io_err: bool,
    msgs: String
}

#[derive(Serialize, Deserialize)]
struct Timeout {
    timeout: bool,
    msgs: String
}

impl Timeout {
    fn new(s: String) -> Timeout {
        Timeout { timeout: true, msgs: s }
    }
}

#[derive(Serialize, Deserialize)]
struct CompileFail {
    failed: bool,
    msgs: String
}

type AlterFn = Box<Fn(String) -> String>;
type RespFn = Box<Fn(String, String) -> String>;
type RouteMap = HashMap<String, (&'static str, AlterFn, RespFn)>;

fn route_map() -> RouteMap {
    let mut tmp: RouteMap = HashMap::new();
    tmp.insert(String::from("planets"), 
                 (planets::PAGE_TEXT, Box::new(planets::mod_cpp), Box::new(planets::process_out)));
    tmp
}

fn route_stuff(map: &RouteMap, req: Request) -> String {
    let uri_str = match req.uri {
        AbsolutePath(ref s) => vec![s.clone()],
        AbsoluteUri(ref url) => Vec::from(url.path().unwrap()),
        _ => vec![String::from("BADPATH")]
    };
    let uri_strings: Vec<String> = uri_str.iter()
                                          .fold(String::from(""), |a, ref b| a + b)
                                          .split("/")
                                          .filter(|&s| s.len() > 0)
                                          .map(|s| String::from(s))
                                          .collect();
    println!("{:?}: {}: {} {:?}", SystemTime::now(), req.remote_addr, req.method, uri_strings);
    if uri_strings.len() < 1 {
        return String::from(HOME_PAGE);
    }
    match map.get(&uri_strings[0]) {
        Some (&(get_string, ref alter_fn, ref resp_fn)) => {
            match req.method {
                Method::Get => {
                    String::from(get_string)
                },
                Method::Post => {
                    let input = req.chars().map(|c| c.unwrap()).collect();
                    let processed_input = alter_fn(input);
                    let results = exec_cpp(processed_input);
                    match results {
                        ExecResult::Success(warnings, output) => {
                            resp_fn(warnings, output)
                        },
                        ExecResult::IoFail(e) => {
                            let payload = IoFail {
                                io_err: true,
                                msgs: format!("{:?}", e)
                            };
                            serde_json::to_string(&payload).unwrap()
                        },
                        ExecResult::Timeout(s) => {
                            serde_json::to_string(&Timeout::new(s)).unwrap()
                        },
                        ExecResult::CompileFail(s) => {
                            let payload = CompileFail {
                                failed: true,
                                msgs: s
                            };
                            serde_json::to_string(&payload).unwrap()
                        }
                    }
                },
                _ => String::from(ERROR_PAGE),
            }
        },
        None => String::from(ERROR_PAGE),
    }
}

// TODO: Restructure so that we can do something other than panicking on failure
fn handle_stuff(routes: &RouteMap, req: Request, res: Response) {
    // println!("{}", msg);
    let msg = route_stuff(routes, req);
    res.send(msg.as_bytes()).unwrap();
}

struct StuffHandler {
    routes: RouteMap
}

unsafe impl Send for StuffHandler {}
unsafe impl Sync for StuffHandler {}

impl Handler for StuffHandler {
    fn handle<'a, 'k> (&'a self, req: Request<'a, 'k>, resp: Response<'a, Fresh>) {
        handle_stuff(&self.routes, req, resp)
    }
}

fn main() {
    println!("server start!");

    /*
    // TODO: convert this into a test!
    let q = exec_cpp(planets::mod_cpp(String::from("
Force CalculateForces(const Body &a, const Body &b) {
    /// your code here!
    return Force{0.0, 0.0};
}")));
    println!("{:?}", q);
    */

    let handler = StuffHandler{ routes: route_map() };
    let _ = Server::http("localhost:3000").unwrap().handle(handler);
}
