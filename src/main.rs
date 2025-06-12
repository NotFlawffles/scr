use std::error::Error;

use repl::Repl;

pub mod compiler;
pub mod repl;

fn main() -> Result<(), Box<dyn Error>> {
    let mut repl = Repl::new()?;
    repl.run()
}
