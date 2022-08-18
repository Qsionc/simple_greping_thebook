pub mod cfg;
pub mod run;

use crate::run::Runner;

fn main() -> Result<(), &'static str> {
    let config = cfg::Config::new()?;
    Runner::run(config)
}
