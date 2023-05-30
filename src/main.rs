use function;
use std::env;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    function::run(&args)
}
