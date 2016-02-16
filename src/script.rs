extern crate rand;
extern crate wait_timeout;

use std::fs::File;
use std::process::Command;
use std::process::Stdio;

// TODO: Figure out why it needs to be done this way
use self::wait_timeout::ChildExt;

use std::time::Duration;

use std::io::Read;
use std::io::Write;
use std::io::Error;


#[derive(Debug)]
pub enum ExecResult {
    Success(String, String),
    IoFail(Error),
    Timeout(String),
    CompileFail(String)
}

pub fn exec_cpp(cpp: String) -> ExecResult {
    let file_num: u64 = rand::random();
    // add the source code to a tmp/*.cc file
    let file_name = format!("tmp/{}.cc", file_num);
    let output_name = format!("tmp/{}", file_num);
    {
        let mut file = match File::create(&file_name) {
            Ok(f) => f,
            Err(e) => { return ExecResult::IoFail(e); }
        };
        match file.write_all(cpp.as_bytes()) {
            Ok(_) => (),
            Err(e) => { return ExecResult::IoFail(e); }
        }
    }
    let mut warnings = String::new();
    // compile it, returning if there's any errors
    {
        let results = Command::new("g++")
                                .arg("-o")
                                .arg(&output_name)
                                .arg("-std=c++11")
                                .arg("-Wall")
                                .arg(&file_name)
                                .stdin(Stdio::piped())
                                .output();
        match results {
            Ok(output) => {
                let stderr = String::from_utf8(output.stderr.clone()).unwrap();
                let stdout = String::from_utf8(output.stdout.clone()).unwrap();
                println!("compile run: status {}, stderr: {}, stdout: {}",
                         output.status, stderr, stdout);
                if !output.status.success() {
                    return ExecResult::CompileFail(stderr);
                }
                warnings = stderr;
            },
            Err(e) => { return ExecResult::IoFail(e); }
        }
    }
    // TODO: SECURE wrapper
    // then run it with a secure wrapper
    let output = {
        let mut child = {
            let command = Command::new("sudo")
                                    .arg("-u")
                                    .arg("lunarknights")
                                    .arg("-s")
                                    .arg(output_name)
                                    .stdout(Stdio::piped())
                                    .spawn();
            match command {
                Ok(c) => c,
                Err(e) => { return ExecResult::IoFail(e); }
            }
        };
        let maybe_exited = match child.wait_timeout(Duration::from_millis(1000)) {
            Ok(results) => results,
            Err(e) => { return ExecResult::IoFail(e); }
        };
        match maybe_exited {
            Some(_) => {
                // grab data from child.stdout
                let mut ret = String::new();
                child.stdout.unwrap().read_to_string(&mut ret).unwrap();
                ret
            },
            None => {
                let kill_child = Command::new("sudo")
                                        .arg("kill")
                                        .arg(format!("{}", child.id()))
                                        .status();

                 match kill_child {
                    Ok(s) => {
                        if !s.success() {
                            println!("failed to kill child {}", child.id());
                        }
                    },
                    Err(e) => {
                        println!("io error on child kill: {:?}", e);
                        return ExecResult::Timeout(warnings);
                    }
                }
                return ExecResult::Timeout(warnings);
            }
        }
    };
    // and return the parsed standard output
    ExecResult::Success(warnings, output)
}
