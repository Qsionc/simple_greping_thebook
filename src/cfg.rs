use std::env::args;
use std::path::Path;

pub struct Config {
    file: String,
    filter: String,
}

struct PathAnalyzer;

impl Config {
    pub fn new() -> Result<Self, &'static str> {
        let args: Vec<String> = args().collect();
        if &args.len() != &3 {
            return Err("Command line argument count mismatch. Required: 2, file and phrase");
        }
        let data = Self::find_first_path(&args[1..])?;
        Ok(Self {
            file: String::from(data.0),
            filter: String::from(data.1),
        })
    }

    pub fn file_path(&self) -> &str {
        &self.file
    }

    pub fn query(&self) -> &str {
        &self.filter
    }

    fn try_is_file(path: &str) -> bool {
        let p = Path::new(path);
        p.exists() && p.is_file()
    }

    fn find_first_path(args: &[String]) -> Result<(& str, & str), &'static str> {
        assert_eq!(args.len(), 2);
        if Self::try_is_file(&args[0]) {
            return Ok((&args[0], &args[1]));
        }
        if Self::try_is_file(&args[1]) {
            return Ok((&args[1], &args[0]));
        }
        Err("Missing file path.")
    }
}