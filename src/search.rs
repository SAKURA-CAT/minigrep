/// Search for a query in a content and return a list of lines that match.
/// print the result to the console.
fn filter<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    if ignore_case {
        return contents.lines().filter(|line| line.to_lowercase().contains(query)).collect();
    }
    contents.lines().filter(|line| line.contains(query)).collect()
}

/// run [filter] with the given query and file_path, printing the result to the console.
pub fn run(query: String, file_path: String, ignore_case: bool) {
    let contents = std::fs::read_to_string(file_path).unwrap();
    let result = filter(&query, &contents, ignore_case);
    if result.is_empty() {
        println!("Oops! No result found.");
        return;
    }
    println!("Result:");
    for line in result {
        println!("* {}", line);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Test the filter function with ignore_case set to false.
    #[test]
    fn test_filter() {
        let query = "public";
        let contents = "public struct Test {}";
        let result = filter(query, contents, false);
        assert_eq!(result, vec!["public struct Test {}"]);
    }


    /// Test the filter function with ignore_case set to false.
    /// The query is in uppercase, but the content is in lowercase.
    #[test]
    fn test_filter_case_sensitive() {
        let query = "public";
        let contents = "PUBLIC struct Test {}";
        let result = filter(query, contents, false);
        assert_eq!(result, Vec::<&str>::new());
    }


    /// Test the filter function with ignore_case set to true.
    #[test]
    fn test_filter_ignore_case() {
        let query = "public";
        let contents = "PUBLIC struct Test {}";
        let result = filter(query, contents, true);
        assert_eq!(result, vec!["PUBLIC struct Test {}"]);
    }


    /// Test the filter function with no result.
    /// The query is not in the content.
    #[test]
    fn test_filter_no_result() {
        let query = "public";
        let contents = "struct Test {}";
        let result = filter(query, contents, false);
        assert_eq!(result, Vec::<&str>::new());
    }
}

