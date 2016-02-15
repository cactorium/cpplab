extern crate rand;

use std::fs::File;
use std::process::Command;

use std::io::Error;
use std::io::Write;

pub struct Point {
    x: f64,
    y: f64
}

pub enum ExecError {
    IoFail(Error),
    TimeOut,
    CompileFail(String)
}

pub fn exec_cpp(cpp: String) -> Result<(String, Vec<Point>), ExecError> {
    let file_num: u64 = rand::random();
    // add the source code to a tmp/*.cc file
    let file_name = format!("tmp/{}.cc", file_num);
    let output_name = format!("tmp/{}", file_num);
    {
        let mut file = match File::create(&file_name) {
            Ok(f) => f,
            Err(e) => { return Err(ExecError::IoFail(e)); }
        };
        match file.write_all(cpp.as_bytes()) {
            Ok(_) => (),
            Err(e) => { return Err(ExecError::IoFail(e)); }
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
                                .output();
        match results {
            Ok(output) => {
                let stderr = String::from_utf8(output.stderr.clone()).unwrap();
                let stdout = String::from_utf8(output.stdout.clone()).unwrap();
                println!("compile run: status {}, stderr: {}, stdout: {}",
                         output.status, stderr, stdout);
                if !output.status.success() {
                    return Err(ExecError::CompileFail(stderr));
                }
                warnings = stderr;
            },
            Err(e) => { return Err(ExecError::IoFail(e)); }
        }
    }
    // then run it with a secure wrapper
    // and return the parsed standard output
    Ok((String::new(), vec![]))
}
