use crate::run::Runner;

pub mod cfg;
pub mod run;

fn main() -> Result<(), &'static str> {
    let config = cfg::Config::new()?;
    Runner::run(config)
}
