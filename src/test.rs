#[cfg(test)]
use super::*;

#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct.";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Trust me.";
    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_insensitive(query, contents)
    );
}
