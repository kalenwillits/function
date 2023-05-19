use std::env;
use function;


fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    function::run(&args)
}
