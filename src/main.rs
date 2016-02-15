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
use script::ExecResult;

static ERROR_PAGE: &'static str = "
PAGE NOT FOUND
PLEASE CLICK HARDER
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
    let experiments = vec![(planets::PAGE_TEXT, planets::mod_cpp, planets::process_out)];
    let mut msg = String::from(ERROR_PAGE);
    if uri_strings.len() == 1 {
        match parse_num(&uri_strings[0]) {
            Some(n) => {
                if n < experiments.len() {
                    let (page, alter, process) = experiments[n];
                    match req.method {
                        Method::Get => {
                            msg = String::from(page);
                        },
                        Method::Post => {
                            let input = String::from("foo");
                            let processed_input = alter(input);
                            let results = exec_cpp(processed_input);
                            match results {
                                ExecResult::Success(warnings, output) => {
                                    msg = process(warnings, output);
                                },
                                ExecResult::IoFail(e) => {
                                },
                                ExecResult::Timeout => {
                                    msg = String::from("{timeout:true}");
                                },
                                ExecResult::CompileFail(s) => {
                                    msg = s;
                                }
                            }
                        },
                        _ => ()
                    }
                }
            },
            None => ()
        };
    }

    res.send(msg.as_bytes()).unwrap();
}

fn main() {
    println!("server start!");

    // TODO: convert this into a test!
    let q = exec_cpp(planets::mod_cpp(String::from("
Force CalculateForces(const Body &a, const Body &b) {
    /// your code here!
    return Force{0.0, 0.0};
}")));
    println!("{:?}", q);

    Server::http("127.0.0.1:3000").unwrap().handle(handle_stuff);
}
