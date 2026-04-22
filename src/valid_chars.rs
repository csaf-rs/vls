/// O(1) compile-time generated lookup of ASCII characters validity.
/// Creates a `[bool; 128]`, `valid_special` ASCII characters are mapped to their codepoint (0 - 127).
/// If the char is valid, the bool in the resulting `[bool; 128]` is set to true.
const fn build_lookup(valid_special: &str) -> [bool; 128] {
    let mut table = [false; 128];
    let bytes = valid_special.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        table[bytes[i] as usize] = true;
        i += 1;
    }
    table
}

/// Identifies which set of characters is considered valid in a given context.
pub enum VlsSpecialCharSet {
    /// Valid characters for a `version-string`.
    /// See [vls::Vls] for more details on the grammar.
    VersionString,
    /// Valid characters for a `vls` string
    /// See [vls::Vls] for more details on the grammar.
    VlsString,
}

impl VlsSpecialCharSet {
    fn get_lookup(&self) -> &'static [bool; 128] {
        // Generate compile-time lookups
        const VERSION_STRING: [bool; 128] = build_lookup("-._+~");
        const VLS_STRING: [bool; 128] = build_lookup("-._+~=!<>|*");
        match self {
            VlsSpecialCharSet::VersionString => &VERSION_STRING,
            VlsSpecialCharSet::VlsString => &VLS_STRING,
        }
    }
}

/// Checks if the char is 
/// a) ASCII alphanumeric and   
/// b) contained in a list of allowed special chars for either the whole VLS string or a version string.
#[inline]
fn is_valid_char(ch: char, valid_lookup: &[bool; 128]) -> bool {
    let idx = ch as usize;
    ch.is_ascii_alphanumeric() || (idx < 128 && valid_lookup[idx])
}

/// Collects characters from `input` that are **not** ASCII-alphanumeric and **not**
/// contained in `special_charset`, returning them sorted and deduplicated.
///
/// Returns `None` if every character is valid.
pub fn collect_invalid_characters(input: &str, special_charset: VlsSpecialCharSet) -> Option<Vec<char>> {
    let lookup = special_charset.get_lookup();
    let mut invalid: Vec<char> = input
        .chars()
        .filter(|ch| !is_valid_char(*ch, lookup))
        .collect();

    if invalid.is_empty() {
        return None;
    }

    invalid.sort();
    invalid.dedup();
    Some(invalid)
}
