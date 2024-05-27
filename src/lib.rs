mod search;
mod version;
mod about;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, author)]
pub struct Cli {
    #[command(subcommand)]
    subcommand: Option<Commands>,
}


#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Search word in file, default use if not provided subcommand
    Search {
        /// Search word
        query: String,
        /// Search file path
        file_path: String,
        /// Ignore case
        #[arg(short, long, default_value = "false")]
        ignore_case: bool,
    },
    /// Say hi
    Hi {},
    /// Version
    Version {},
}

impl Cli {
    /// new a Cli instance and execute the subcommand
    pub fn new() {
        let cli = Cli::parse();
        cli.execute();
    }
    /// parse from args
    pub fn execute(&self) {
        match &self.subcommand {
            Some(Commands::Search { query, file_path, ignore_case }) => {
                search::run(query.to_string(), file_path.to_string(), *ignore_case);
            }
            Some(Commands::Hi {}) => {
                about::run();
            }
            Some(Commands::Version {}) => {
                version::run();
            }
            _ => {
                about::run();
            }
        }
    }
}


// ---------------------------------- 测试 ----------------------------------
/// 测试用的搜索套件
#[cfg(test)]
struct TestSearchSuite {
    query: String,
    file_path: String,
    ignore_case: bool,
}

#[cfg(test)]
impl TestSearchSuite {
    fn new(query: &str, ignore_case: bool) -> TestSearchSuite {
        let file_path = std::env::current_dir().unwrap().join("test.txt").to_str().unwrap().to_string();
        TestSearchSuite {
            query: query.to_string(),
            file_path,
            ignore_case,
        }
    }
    fn to_command(&self) -> Commands {
        Commands::Search {
            query: self.query.clone(),
            file_path: self.file_path.clone(),
            ignore_case: self.ignore_case,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_search() {
        let s = TestSearchSuite::new("public", false);
        let cli = Cli::parse_from(&["", "search", s.query.as_str(), s.file_path.as_str()]);
        let result = cli.subcommand.unwrap();
        assert_eq!(result, s.to_command());
    }

    #[test]
    fn test_cli_search_ignore_case() {
        let s = TestSearchSuite::new("public", true);
        let mut cli = Cli::parse_from(&["", "search", s.query.as_str(), s.file_path.as_str(), "--ignore-case"]);
        let result = cli.subcommand.unwrap();
        assert_eq!(result, s.to_command());
        cli = Cli::parse_from(&["", "search", s.query.as_str(), s.file_path.as_str(), "-i"]);
        let result = cli.subcommand.unwrap();
        assert_eq!(result, s.to_command());
    }

    #[test]
    fn test_cli_hi() {
        let cli = Cli::parse_from(&["", "hi"]);
        let result = cli.subcommand.unwrap();
        assert_eq!(result, Commands::Hi {});
    }

    #[test]
    fn test_cli_version() {
        let cli = Cli::parse_from(&["", "version"]);
        let result = cli.subcommand.unwrap();
        assert_eq!(result, Commands::Version {});
    }
}