
pub mod parser;

use parser::parameters::Parameters;
use std::io::BufRead;
use std::env;



fn main() -> Result<(), String> {

    let args = env::args().skip(1);
    let param = Parameters::new_argv(args)?;
    Ok(())
}


