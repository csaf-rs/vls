/// Collects characters from `input` that are **not** ASCII-alphanumeric and **not**
/// contained in `valid_special`, returning them sorted and deduplicated.
///
/// Returns `None` when every character is valid.
pub fn collect_invalid_characters(input: &str, valid_special: &str) -> Option<Vec<char>> {
    let mut invalid: Vec<char> = input
        .chars()
        .filter(|ch| !ch.is_ascii_alphanumeric() && !valid_special.contains(*ch))
        .collect();

    if invalid.is_empty() {
        return None;
    }

    invalid.sort();
    invalid.dedup();
    Some(invalid)
}
