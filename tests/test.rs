#[macro_use]
extern crate quickcheck_macros;

use quickcheck::TestResult;

#[quickcheck]
fn regex_find_iter_and_str_match_indices_are_consistent(
    text: String,
    skip: usize,
    len: usize,
) -> TestResult {
    if len == 0 || skip.checked_add(len).is_none() || skip + len >= text.chars().count() {
        return TestResult::discard();
    }
    let query: String = text.chars().skip(skip).take(len).collect();
    let regex = regex::Regex::new(&regex_syntax::escape(&query)).unwrap();
    let a: Vec<_> = regex.find_iter(&text).map(|m| m.start()).collect();
    let b: Vec<_> = text.match_indices(&query).map(|(i, _)| i).collect();
    TestResult::from_bool(a == b)
}
