#![feature(time2)]
extern crate hyper;

mod script;
mod planets;

use std::time::SystemTime;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;

use hyper::uri::RequestUri::*;
use hyper::method::Method;

use script::exec_cpp;

static ERROR_PAGE: &'static [u8] = b"
PAGE NOT FOUND
CLICK HARDER
";

fn parse_num(s: &String) -> Option<usize> {
    let mut n = 0;
    for c in s.chars() {
        if c >= '0' && c <= '9' {
            n = 10*n + ((c as usize)-('0' as usize));
        } else {
            return None;
        }
    }
    Some(n)
}

// TODO: Restructure so that we can do something other than panicking on failure
fn handle_stuff(req: Request, res: Response) {
    let uri_str = match req.uri {
        AbsolutePath(s) => vec![s],
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
    let pages = vec![planets::PAGE_TEXT];
    let alter_fns = vec![planets::mod_cpp];
    let mut msg = ERROR_PAGE;
    if uri_strings.len() == 1 {
        match parse_num(&uri_strings[0]) {
            Some(n) => {
                if n >= 0 && n < pages.len() {
                    msg = pages[n];
                }
            },
            None => {},
        }
    } else {
        assert!(req.method == Method::Post);
        match parse_num(&uri_strings[0]) {
            Some(n) => {
                if n >= 0 && n < alter_fns.len() {
                    exec_cpp(alter_fns[n](String::from("foo")));
                    msg = ERROR_PAGE;
                }
            },
            None => {},
        }
    }

    res.send(msg).unwrap();
}

fn main() {
    println!("server start!");
    Server::http("127.0.0.1:3000").unwrap().handle(handle_stuff);
}
