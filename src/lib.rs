#[cfg(test)]
mod tests;

use home;
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

fn load_functions(cache: &mut HashMap<OsString, OsString>, dir: &mut PathBuf) -> Result<(), String> {
    dir.push("run");
    if dir.is_dir() {
        for function in fs::read_dir(dir).expect("Unable to read dir") {
            if let Ok(function_path) = function {
                if let Some(function_name) = function_path.path().file_stem() {
                    cache.insert(function_name.to_os_string(), function_path.path().into_os_string());
                }

            };
        }
    }
    Ok(())
}


fn use_arg(cache: &HashMap<OsString, OsString>, key: OsString, args: &[String]) {
    let output = Command::new("sh")
        .arg(&cache[&key])
        .args(args)
        .output()
        .expect("Failed to execute function");
    io::stdout().write_all(&output.stdout).expect("Unable to write to stdout");
    io::stderr().write_all(&output.stderr).expect("Unable to write to stderr");
}


pub fn run(args: Vec<String>) -> Result<(), String> {
    let mut cache: HashMap<OsString, OsString> = HashMap::new();
    let _ = load_functions(&mut cache, &mut home::home_dir().expect("Unable to locate home dir"));
    let _ = load_functions(&mut cache, &mut env::current_dir().expect("Unable to gather working dir"));
    if args.len() > 1 {
        if (args[1] == "-v") | (args[1] == "--version") {
            println!("function version {}", env::var("CARGO_PKG_VERSION")
                .expect("Unable to gather package version"));
        } else if (args[1] == "-l") | (args[1] == "--list") {
            for (key, value) in cache {
                println!(
                    "{}: {}", 
                    key.to_str()
                        .expect("Could not unwrap key"), 
                    value
                        .to_str()
                        .expect("Could not unwrap value"));
            }
        } else {
            let key = OsString::from(&args[1]);
            if cache.contains_key(&key) {
                use_arg(&cache, key, &args[2..]);
            }
        }
    } else {
        return Err(String::from("No arguments supplied"));
    }
    Ok(())
}
