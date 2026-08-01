//! URL-friendly string parameterization and transliteration.
//!
//! Provides [`transliterate`] for converting Unicode to ASCII approximations,
//! and [`parameterize`] for creating URL-friendly slugs.

use regex::Regex;
use std::sync::LazyLock;
use unicode_normalization::UnicodeNormalization;

// Pre-compiled regex for parameterize: matches non-alphanumeric, non-dash, non-underscore
static RE_PARAMETERIZE_UNWANTED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)[^a-z0-9\-_]+").unwrap());

/// Replaces non-ASCII characters with an ASCII approximation using Unicode
/// NFKD decomposition. Characters with no ASCII decomposition are dropped.
///
/// # Examples
///
/// ```
/// use inflection::transliterate;
///
/// assert_eq!(transliterate("älämölö"), "alamolo");
/// assert_eq!(transliterate("Ærøskøbing"), "rskbing");
/// ```
///
/// # Known Limitation
///
/// Characters like `Æ` and `ø` that decompose to non-ASCII codepoints
/// are silently dropped. This matches the Python original's behavior.
/// For more comprehensive transliteration, consider a dedicated library.
pub fn transliterate(string: &str) -> String {
    string
        .nfkd()
        .filter(|c| c.is_ascii())
        .collect()
}

/// Replaces special characters in a string so that it may be used as part
/// of a "pretty" URL.
///
/// The string is first transliterated to ASCII, then non-alphanumeric
/// characters (except hyphens and underscores) are replaced with the
/// `separator`. Consecutive separators are squeezed, and leading/trailing
/// separators are removed.
///
/// # Examples
///
/// ```
/// use inflection::parameterize;
///
/// assert_eq!(parameterize("Donald E. Knuth", "-"), "donald-e-knuth");
/// assert_eq!(parameterize("Allow_Under_Scores", "-"), "allow_under_scores");
/// assert_eq!(parameterize("Donald E. Knuth", "_"), "donald_e_knuth");
/// assert_eq!(parameterize("Donald E. Knuth", ""), "donaldeknuth");
/// ```
pub fn parameterize(string: &str, separator: &str) -> String {
    // Step 1: Transliterate to ASCII
    let mut result = transliterate(string);

    // Step 2: Replace unwanted characters with the separator
    result = RE_PARAMETERIZE_UNWANTED
        .replace_all(&result, separator)
        .into_owned();

    // Step 3: If separator is non-empty, squeeze consecutive separators
    // and strip leading/trailing separators
    if !separator.is_empty() {
        let escaped = regex::escape(separator);

        // Squeeze: no more than one separator in a row
        let re_squeeze = Regex::new(&format!("{}{{2,}}", escaped)).unwrap();
        result = re_squeeze.replace_all(&result, separator).into_owned();

        // Strip leading/trailing separator
        let re_trim = Regex::new(&format!("(?i)^{sep}|{sep}$", sep = escaped)).unwrap();
        result = re_trim.replace_all(&result, "").into_owned();
    }

    result.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── transliterate ─────────────────────────────────────────────────

    #[test]
    fn test_transliterate_basic() {
        assert_eq!(transliterate("älämölö"), "alamolo");
    }

    #[test]
    fn test_transliterate_drops_non_decomposable() {
        // Æ and ø have no ASCII decomposition — they're dropped
        assert_eq!(transliterate("Ærøskøbing"), "rskbing");
    }

    #[test]
    fn test_transliterate_ascii_passthrough() {
        assert_eq!(transliterate("hello world"), "hello world");
    }

    #[test]
    fn test_transliterate_empty() {
        assert_eq!(transliterate(""), "");
    }

    // ── parameterize ──────────────────────────────────────────────────

    #[test]
    fn test_parameterize_default_separator() {
        assert_eq!(parameterize("Donald E. Knuth", "-"), "donald-e-knuth");
    }

    #[test]
    fn test_parameterize_bad_characters() {
        assert_eq!(
            parameterize("Random text with *(bad)* characters", "-"),
            "random-text-with-bad-characters"
        );
    }

    #[test]
    fn test_parameterize_underscores_preserved() {
        assert_eq!(
            parameterize("Allow_Under_Scores", "-"),
            "allow_under_scores"
        );
    }

    #[test]
    fn test_parameterize_trailing_bad() {
        assert_eq!(
            parameterize("Trailing bad characters!@#", "-"),
            "trailing-bad-characters"
        );
    }

    #[test]
    fn test_parameterize_leading_bad() {
        assert_eq!(
            parameterize("!@#Leading bad characters", "-"),
            "leading-bad-characters"
        );
    }

    #[test]
    fn test_parameterize_squeeze() {
        assert_eq!(
            parameterize("Squeeze   separators", "-"),
            "squeeze-separators"
        );
    }

    #[test]
    fn test_parameterize_plus_sign() {
        assert_eq!(parameterize("Test with + sign", "-"), "test-with-sign");
    }

    #[test]
    fn test_parameterize_no_separator() {
        assert_eq!(parameterize("Donald E. Knuth", ""), "donaldeknuth");
        assert_eq!(
            parameterize("Random text with *(bad)* characters", ""),
            "randomtextwithbadcharacters"
        );
    }

    #[test]
    fn test_parameterize_underscore_separator() {
        assert_eq!(
            parameterize("Donald E. Knuth", "_"),
            "donald_e_knuth"
        );
        assert_eq!(
            parameterize("With-some-dashes", "_"),
            "with-some-dashes"
        );
    }

    #[test]
    fn test_parameterize_multi_char_separator() {
        assert_eq!(
            parameterize("Donald E. Knuth", "__sep__"),
            "donald__sep__e__sep__knuth"
        );
    }
}
