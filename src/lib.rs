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

static VERSION: &str = env!("CARGO_PKG_VERSION");


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


fn use_arg(cache: &HashMap<OsString, OsString>, args: &[String]) {
    let key = OsString::from(&args[1]);
    if cache.contains_key(&key) {
        let output = Command::new("sh")
            .arg(&cache[&key])
            .args(args)
            .output()
            .expect("Failed to execute function");
        io::stdout().write_all(&output.stdout).expect("Unable to write to stdout");
        io::stderr().write_all(&output.stderr).expect("Unable to write to stderr");
    }
}

fn use_list(cache: &HashMap<OsString, OsString>) {
    for (key, value) in cache {
        println!(
            "{}: {}", 
            key.to_str()
            .expect("Could not unwrap key"), 
            value
            .to_str()
            .expect("Could not unwrap value"));
            }

}


pub fn run(args: Vec<String>) -> Result<(), String> {
    let mut cache: HashMap<OsString, OsString> = HashMap::new();
    let _ = load_functions(&mut cache, &mut home::home_dir().expect("Unable to locate home dir"));
    let _ = load_functions(&mut cache, &mut env::current_dir().expect("Unable to gather working dir"));
    if args.len() == 1usize {
        use_list(&cache);
    }  else if args[1] == "--version" {
      println!("function version {VERSION}");
    } else {
        use_arg(&cache, &args[2..]);
    }
    Ok(())
}
