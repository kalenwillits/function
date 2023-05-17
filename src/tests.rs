use super::*;

use std::collections::HashMap;
use std::env;


#[test]
fn load_global_functions() -> Result<(), String> {
    let mut cache: HashMap<OsString, OsString> = HashMap::new();
    load_functions(&mut cache, &mut home::home_dir().expect("Failed to load home dir"))
}


#[test]
fn load_local_functions() -> Result<(), String> {
    let mut cache: HashMap<OsString, OsString> = HashMap::new();
    load_functions(&mut cache, &mut env::current_dir().expect("Failed to load current dir"))
}

#[test]
fn exec_list() -> Result<(), String> {
    run(vec![String::from("/home/user/dummy"), String::from("--list")])
}

#[test]
fn exec_version() -> Result<(), String> {
    run(vec![String::from("/home/user/dummy"), String::from("--version")])
}


#[test]
fn exec_missing() {
    assert!(!run(vec![String::from("/home/user/dummy"), String::from("non-existant-function-1231231251")]).is_err())
}





