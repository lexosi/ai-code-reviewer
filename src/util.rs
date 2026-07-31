/// Returns the longest prefix of `s` that is at most `max_bytes` long and ends
/// on a UTF-8 char boundary. Never panics, never splits a multibyte character.
pub(crate) fn truncate_on_char_boundary(s: &str, max_bytes: usize) -> &str {
    if max_bytes >= s.len() {
        return s;
    }
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

#[cfg(test)]
mod tests {
    use super::truncate_on_char_boundary;

    #[test]
    fn returns_whole_string_when_max_ge_len() {
        let s = "hello";
        let result = truncate_on_char_boundary(s, 100);
        assert_eq!(result, "hello");
        // Exactly-equal boundary case must also return the whole string.
        assert_eq!(truncate_on_char_boundary(s, s.len()), "hello");
    }

    #[test]
    fn ascii_truncation_cuts_at_exactly_max_bytes() {
        let s = "abcdefgh";
        let result = truncate_on_char_boundary(s, 3);
        assert_eq!(result, "abc");
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn multibyte_accent_never_splits_char() {
        // 'á' is 2 bytes in UTF-8; 10 of them = 20 bytes.
        let s = "á".repeat(10);
        // max_bytes = 5 lands in the middle of the 3rd 'á' (boundaries at 0,2,4,6...).
        let result = truncate_on_char_boundary(&s, 5);

        // Never splits a char -> result is valid UTF-8 (guaranteed by &str type)
        // and is a proper prefix of the input.
        assert!(
            s.starts_with(result),
            "result must be a prefix of the input"
        );
        assert!(
            result.len() <= 5,
            "result length {} exceeded max_bytes 5",
            result.len()
        );
        // With a mid-char cut at byte 5, the safe boundary is byte 4 -> two 'á'.
        assert_eq!(result, "áá");
        assert_eq!(result.len(), 4);
        assert_eq!(result.chars().count(), 2);
    }

    #[test]
    fn emoji_never_splits_char() {
        // '😀' is 4 bytes in UTF-8; 5 of them = 20 bytes.
        let s = "😀".repeat(5);
        // max_bytes = 10 lands mid-char (boundaries at 0,4,8,12,16,20).
        let result = truncate_on_char_boundary(&s, 10);

        assert!(
            s.starts_with(result),
            "result must be a prefix of the input"
        );
        assert!(
            result.len() <= 10,
            "result length {} exceeded max_bytes 10",
            result.len()
        );
        // Safe boundary below 10 is byte 8 -> exactly two whole emojis.
        assert_eq!(result.chars().count(), 2);
        assert_eq!(result.len(), 8);
        assert_eq!(result, "😀😀");
    }

    #[test]
    fn zero_max_bytes_returns_empty() {
        let s = "anything at all 😀 áé";
        let result = truncate_on_char_boundary(s, 0);
        assert_eq!(result, "");
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn mixed_ascii_and_multibyte_stops_before_split() {
        // "abc" = 3 ASCII bytes, 'é' = 2 bytes (bytes 3..5). Total 5 bytes.
        let s = "abcé";
        assert_eq!(s.len(), 5);
        // max_bytes = 4 lands in the middle of 'é' (boundaries at 0,1,2,3,5).
        let result = truncate_on_char_boundary(s, 4);
        assert_eq!(result, "abc");
        assert_eq!(result.len(), 3);
        assert!(s.starts_with(result));
    }
}
