use std::env;


const HELP_ARG: Vec<String> = vec![String::from("-h"), String::from("--help")];
const NEW_ARG: Vec<String> = vec![String::from("-n"), String::from("--new")];
const INFO_ARG: Vec<String> = vec![String::from("-i"), String::from("--info")];



enum AppState {
    Help,
    Info(String),
    New,
    Std,
}



struct Args {
    data: Vec<String>
}


impl Args {
    pub fn new(&self) -> Self {
       Self {
            data: env::args().collect()    
        } 
    }

    fn is_help(&self) -> bool {
        HELP_ARG.iter()
            .filter(|&arg| self.data
                .contains(arg))
            .collect::<Vec<_>>()
            .len() > 1
    }

    fn is_new(&self) -> bool {
        NEW_ARG.iter()
            .filter(|&arg| self.data
                .contains(arg))
            .collect::<Vec<_>>()
            .len() > 1
    }


    fn is_info(&self) -> bool {
        INFO_ARG.iter()
            .filter(|&arg| self.data
                .contains(arg))
            .collect::<Vec<_>>()
            .len() > 1
    }


    pub fn state(&self) -> AppState {
        if self.is_help() {
            return AppState::Help;
        } else if self.is_new() { 
            return AppState::New;            
        } else if self.is_info() {
            return AppState::Info(function_name);
        } else {
            return AppState::Std;
        }
    }
}


pub fn run(Args) -> Result<(), String> {
    match Args.state {
        AppState::Help => use_help(),
        AppState::New => use_new(),
        AppState::Info(function_name) => use_info(function_name),
        AppState::Std => use_std(),
    }
}
