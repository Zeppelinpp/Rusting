use minigrep_macro::trace;

#[trace]
pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<(usize, usize, &'a str)> {
    let mut results = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        let line_cmp = if ignore_case { line.to_lowercase() } else { line.to_string() };
        let query_cmp = if ignore_case { query.to_lowercase() } else { query.to_string() };
        if let Some(pos) = line_cmp.find(&query_cmp) {
            results.push((i + 1, pos + 1, line));
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
            vec![(2, 7, "Crazy Jazz Muisc"), (3, 14, "Contemporary jazz")],
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
        assert_eq!(
            vec![(2, 7, "Crazy Jazz Muisc")],
            search(query, contents, false)
        );
    }
}
