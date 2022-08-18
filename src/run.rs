use crate::cfg;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_filter() {
        let query = "word";
        let content = "\
many
words
in this
word
example";

        assert_eq!(vec!["words", "word"], Runner::filter(content, query));
    }
}

pub struct Runner;

impl Runner {
    pub fn run(cfg: cfg::Config) -> Result<(), &'static str> {
        let file_content = match std::fs::read_to_string(cfg.file_path()) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("File read_to_string error: {:?}", e.kind());
                return Err("Error reading file.");
            }
        };
        let res = Self::filter(&file_content, cfg.query());
        match res.is_empty() {
            true => println!("No matches found for '{}' query", cfg.query()),
            false => {
                println!("Matches found for query '{}':", cfg.query());
                for line in Self::filter(&file_content, cfg.query()) {
                    println!("{}", line);
                }
            }
        }
        Ok(())
    }

    fn filter<'a>(content: &'a str, query: &str) -> Vec<&'a str> {
        let mut result = Vec::new();
        for line in content.lines() {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                result.push(line);
            }
        }
        result
    }
}