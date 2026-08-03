//! String case conversion functions.
//!
//! Provides [`camelize`], [`underscore`], [`dasherize`], [`humanize`], and
//! [`titleize`] for converting between various string casing conventions.

use regex::Regex;
use std::sync::LazyLock;

// Pre-compiled regexes for underscore()
static RE_UNDERSCORE_1: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([A-Z]+)([A-Z][a-z])").unwrap());
static RE_UNDERSCORE_2: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([a-z\d])([A-Z])").unwrap());

// Pre-compiled regexes for camelize()
static RE_CAMELIZE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?:^|_)(.)").unwrap());

// Pre-compiled regexes for humanize()
static RE_HUMANIZE_ID: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"_id$").unwrap());
static RE_HUMANIZE_FIRST: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\w").unwrap());

// Pre-compiled regexes for titleize()
static RE_TITLEIZE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b('?\w)").unwrap());

/// Converts a string to CamelCase.
///
/// If `uppercase_first_letter` is `true`, produces UpperCamelCase (PascalCase).
/// If `false`, produces lowerCamelCase.
///
/// # Examples
///
/// ```
/// use inflection::camelize;
///
/// assert_eq!(camelize("device_type", true), "DeviceType");
/// assert_eq!(camelize("device_type", false), "deviceType");
/// ```
///
/// Note that `camelize` is not always the inverse of [`underscore`]:
///
/// ```
/// use inflection::{camelize, underscore};
///
/// // "IOError" → "io_error" → "IoError" (not "IOError")
/// assert_eq!(camelize(&underscore("IOError"), true), "IoError");
/// ```
///
/// # Bug fix vs. Python original
///
/// The Python original raises `IndexError` on `camelize("", False)`.
/// This implementation safely returns an empty string.
pub fn camelize(string: &str, uppercase_first_letter: bool) -> String {
    if uppercase_first_letter {
        RE_CAMELIZE
            .replace_all(string, |caps: &regex::Captures| caps[1].to_uppercase())
            .into_owned()
    } else {
        // Fix for Python Bug #1: empty string no longer panics
        if string.is_empty() {
            return String::new();
        }
        let camelized = camelize(string, true);
        // Lowercase only the first character
        let mut chars = camelized.chars();
        match chars.next() {
            Some(first) => {
                let lower: String = first.to_lowercase().collect();
                lower + chars.as_str()
            }
            None => String::new(),
        }
    }
}

/// Converts CamelCase to snake_case.
///
/// Replaces `::` with `/`, inserts underscores before case transitions,
/// replaces hyphens with underscores, and lowercases the result.
///
/// # Examples
///
/// ```
/// use inflection::underscore;
///
/// assert_eq!(underscore("DeviceType"), "device_type");
/// assert_eq!(underscore("HTMLTidy"), "html_tidy");
/// assert_eq!(underscore("Area51Controller"), "area51_controller");
/// ```
pub fn underscore(word: &str) -> String {
    // Step 1: Insert underscore between consecutive uppercase and uppercase+lowercase
    // e.g., "HTMLTidy" → "HTML_Tidy"
    let result = RE_UNDERSCORE_1.replace_all(word, "${1}_${2}");
    // Step 2: Insert underscore between lowercase/digit and uppercase
    // e.g., "HTML_Tidy" → "HTML_Tidy" (no change), "camelCase" → "camel_Case"
    let result = RE_UNDERSCORE_2.replace_all(&result, "${1}_${2}");
    // Step 3: Replace hyphens with underscores and lowercase
    result.replace('-', "_").to_lowercase()
}

/// Replaces underscores with dashes in the string.
///
/// # Examples
///
/// ```
/// use inflection::dasherize;
///
/// assert_eq!(dasherize("puni_puni"), "puni-puni");
/// assert_eq!(dasherize("street_address"), "street-address");
/// ```
pub fn dasherize(word: &str) -> String {
    word.replace('_', "-")
}

/// Capitalizes the first word, turns underscores into spaces, and strips
/// a trailing `"_id"` suffix.
///
/// # Examples
///
/// ```
/// use inflection::humanize;
///
/// assert_eq!(humanize("employee_salary"), "Employee salary");
/// assert_eq!(humanize("author_id"), "Author");
/// assert_eq!(humanize("underground"), "Underground");
/// ```
pub fn humanize(word: &str) -> String {
    // Strip trailing _id
    let word = RE_HUMANIZE_ID.replace(word, "");
    // Replace underscores with spaces
    let word = word.replace('_', " ");
    // Lowercase everything
    let word = word.to_lowercase();
    // Capitalize first character
    RE_HUMANIZE_FIRST
        .replace(&word, |caps: &regex::Captures| caps[0].to_uppercase())
        .into_owned()
}

/// Capitalizes all words and replaces some characters to create a title.
///
/// Processes the string through [`underscore`] then [`humanize`] first,
/// then capitalizes each word.
///
/// # Examples
///
/// ```
/// use inflection::titleize;
///
/// assert_eq!(titleize("man from the boondocks"), "Man From The Boondocks");
/// assert_eq!(titleize("x-men: the last stand"), "X Men: The Last Stand");
/// assert_eq!(titleize("TheManWithoutAPast"), "The Man Without A Past");
/// assert_eq!(titleize("raiders_of_the_lost_ark"), "Raiders Of The Lost Ark");
/// ```
pub fn titleize(word: &str) -> String {
    // First: underscore → humanize → Python's str.title()
    let humanized = humanize(&underscore(word));
    let titled = title_case(&humanized);

    // Then apply the regex to capitalize after word boundaries (including after apostrophes)
    RE_TITLEIZE
        .replace_all(&titled, |caps: &regex::Captures| {
            // Capitalize the captured character (which may include a leading apostrophe)
            let s = &caps[1];
            capitalize_first(s)
        })
        .into_owned()
}

/// Python-compatible `str.title()` implementation.
///
/// Capitalizes the first letter of each "word" (sequence after a non-alpha character).
fn title_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut capitalize_next = true;

    for c in s.chars() {
        if !c.is_alphabetic() {
            capitalize_next = true;
            result.push(c);
        } else if capitalize_next {
            for upper in c.to_uppercase() {
                result.push(upper);
            }
            capitalize_next = false;
        } else {
            for lower in c.to_lowercase() {
                result.push(lower);
            }
        }
    }
    result
}

/// Python-compatible `str.capitalize()`: uppercase first char, lowercase the rest.
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => {
            let upper: String = c.to_uppercase().collect();
            let rest: String = chars.as_str().to_lowercase();
            upper + &rest
        }
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── camelize ──────────────────────────────────────────────────────

    #[test]
    fn test_camelize_basic() {
        assert_eq!(camelize("device_type", true), "DeviceType");
        assert_eq!(camelize("product", true), "Product");
        assert_eq!(camelize("special_guest", true), "SpecialGuest");
        assert_eq!(
            camelize("application_controller", true),
            "ApplicationController"
        );
        assert_eq!(camelize("area51_controller", true), "Area51Controller");
    }

    #[test]
    fn test_camelize_lower() {
        assert_eq!(camelize("device_type", false), "deviceType");
        assert_eq!(camelize("Capital", false), "capital");
    }

    #[test]
    fn test_camelize_with_underscores() {
        assert_eq!(camelize("Camel_Case", true), "CamelCase");
    }

    #[test]
    fn test_camelize_empty_string() {
        // Bug fix: Python original crashes with IndexError on camelize("", False)
        assert_eq!(camelize("", true), "");
        assert_eq!(camelize("", false), "");
    }

    // ── underscore ────────────────────────────────────────────────────

    #[test]
    fn test_underscore_basic() {
        assert_eq!(underscore("Product"), "product");
        assert_eq!(underscore("SpecialGuest"), "special_guest");
        assert_eq!(
            underscore("ApplicationController"),
            "application_controller"
        );
        assert_eq!(underscore("Area51Controller"), "area51_controller");
    }

    #[test]
    fn test_underscore_acronyms() {
        assert_eq!(underscore("HTMLTidy"), "html_tidy");
        assert_eq!(underscore("HTMLTidyGenerator"), "html_tidy_generator");
        assert_eq!(underscore("FreeBSD"), "free_bsd");
        assert_eq!(underscore("HTML"), "html");
    }

    // ── dasherize ─────────────────────────────────────────────────────

    #[test]
    fn test_dasherize() {
        assert_eq!(dasherize("street"), "street");
        assert_eq!(dasherize("street_address"), "street-address");
        assert_eq!(dasherize("person_street_address"), "person-street-address");
    }

    // ── humanize ──────────────────────────────────────────────────────

    #[test]
    fn test_humanize() {
        assert_eq!(humanize("employee_salary"), "Employee salary");
        assert_eq!(humanize("employee_id"), "Employee");
        assert_eq!(humanize("underground"), "Underground");
    }

    // ── titleize ──────────────────────────────────────────────────────

    #[test]
    fn test_titleize() {
        assert_eq!(titleize("active_record"), "Active Record");
        assert_eq!(titleize("ActiveRecord"), "Active Record");
        assert_eq!(titleize("action web service"), "Action Web Service");
        assert_eq!(titleize("Action Web Service"), "Action Web Service");
        assert_eq!(titleize("Action web service"), "Action Web Service");
        assert_eq!(titleize("actionwebservice"), "Actionwebservice");
        assert_eq!(titleize("Actionwebservice"), "Actionwebservice");
    }

    #[test]
    fn test_titleize_with_apostrophes() {
        assert_eq!(titleize("david's code"), "David's Code");
        assert_eq!(titleize("David's code"), "David's Code");
        assert_eq!(titleize("david's Code"), "David's Code");
    }
}
