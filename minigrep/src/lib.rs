pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if ignore_case {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                results.push(line);
            }
        } else {
            if line.contains(&query) {
                results.push(line);
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_insensitive() {
        let query = "Jazz";
        let contents = "\
Rust:
Crazy Jazz Muisc
Contemporary jazz
";
        assert_eq!(
            vec!["Crazy Jazz Muisc", "Contemporary jazz"],
            search(query, contents, true)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "Jazz";
        let contents = "\
Rust:
Crazy Jazz Muisc
Contemporary jazz
";
        assert_eq!(vec!["Crazy Jazz Muisc"], search(query, contents, false));
    }
}
