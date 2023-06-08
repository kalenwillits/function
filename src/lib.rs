#[cfg(test)]
mod tests;


use home;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn load_functions(
    cache: &mut HashMap<OsString, OsString>,
    dir: &mut PathBuf,
) -> Result<(), String> {
    dir.push("run");
    if dir.is_dir() {
        for function in fs::read_dir(dir).expect("Unable to read dir") {
            if let Ok(function_path) = function {
                if let Some(function_name) = function_path.path().file_stem() {
                    cache.insert(
                        function_name.to_os_string(),
                        function_path.path().into_os_string(),
                    );
                }
            };
        }
    }
    Ok(())
}

fn use_arg(
    cache: &HashMap<OsString, OsString>,
    key: OsString,
    args: &[String],
) -> Result<(), Box<dyn Error>> {
    if cache.contains_key(&key) {
        std::process::Command::new ("sh")
        .args([&cache[&key]])
        .args(args)
        .status()?;
    }
    Ok(())
}

fn use_list(cache: &HashMap<OsString, OsString>) {
    for (key, value) in cache {
        println!(
            "{}: {}",
            key.to_str().expect("Could not unwrap key"),
            value.to_str().expect("Could not unwrap value")
        );
    }
}

pub fn run(args: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut cache: HashMap<OsString, OsString> = HashMap::new();
    let _ = load_functions(
        &mut cache,
        &mut home::home_dir().expect("Unable to locate home dir"),
    );
    let _ = load_functions(
        &mut cache,
        &mut env::current_dir().expect("Unable to gather working dir"),
    );
    if args.len() == 1usize {
        use_list(&cache);
    } else if args[1] == "--version" {
        println!("function version {VERSION}");
    } else {
        let key = OsString::from(&args[1]);
        use_arg(&cache, key, &args[2..])?;
    }
    Ok(())
}
