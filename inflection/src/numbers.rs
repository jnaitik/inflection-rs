//! Ordinal number formatting.
//!
//! Provides [`ordinal`] and [`ordinalize`] for converting integers to their
//! English ordinal representations (1st, 2nd, 3rd, etc.).

/// Returns the ordinal suffix for a number ("st", "nd", "rd", or "th").
///
/// # Examples
///
/// ```
/// use inflection::ordinal;
///
/// assert_eq!(ordinal(1), "st");
/// assert_eq!(ordinal(2), "nd");
/// assert_eq!(ordinal(3), "rd");
/// assert_eq!(ordinal(4), "th");
/// assert_eq!(ordinal(11), "th");
/// assert_eq!(ordinal(12), "th");
/// assert_eq!(ordinal(13), "th");
/// assert_eq!(ordinal(1002), "nd");
/// assert_eq!(ordinal(-11), "th");
/// assert_eq!(ordinal(-1021), "st");
/// ```
pub fn ordinal(number: i64) -> &'static str {
    let n = number.unsigned_abs();
    match n % 100 {
        11..=13 => "th",
        _ => match n % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    }
}

/// Converts a number into an ordinal string ("1st", "2nd", "3rd", "4th", etc.).
///
/// # Examples
///
/// ```
/// use inflection::ordinalize;
///
/// assert_eq!(ordinalize(1), "1st");
/// assert_eq!(ordinalize(2), "2nd");
/// assert_eq!(ordinalize(1002), "1002nd");
/// assert_eq!(ordinalize(1003), "1003rd");
/// assert_eq!(ordinalize(-11), "-11th");
/// assert_eq!(ordinalize(-1021), "-1021st");
/// ```
pub fn ordinalize(number: i64) -> String {
    format!("{}{}", number, ordinal(number))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordinal_basic() {
        assert_eq!(ordinal(0), "th");
        assert_eq!(ordinal(1), "st");
        assert_eq!(ordinal(2), "nd");
        assert_eq!(ordinal(3), "rd");
        assert_eq!(ordinal(4), "th");
        assert_eq!(ordinal(5), "th");
        assert_eq!(ordinal(9), "th");
        assert_eq!(ordinal(10), "th");
    }

    #[test]
    fn test_ordinal_teens() {
        assert_eq!(ordinal(11), "th");
        assert_eq!(ordinal(12), "th");
        assert_eq!(ordinal(13), "th");
        assert_eq!(ordinal(14), "th");
    }

    #[test]
    fn test_ordinal_large_numbers() {
        assert_eq!(ordinal(100), "th");
        assert_eq!(ordinal(101), "st");
        assert_eq!(ordinal(102), "nd");
        assert_eq!(ordinal(103), "rd");
        assert_eq!(ordinal(111), "th");
        assert_eq!(ordinal(112), "th");
        assert_eq!(ordinal(113), "th");
        assert_eq!(ordinal(1000), "th");
        assert_eq!(ordinal(1001), "st");
    }

    #[test]
    fn test_ordinal_negative() {
        assert_eq!(ordinal(-1), "st");
        assert_eq!(ordinal(-2), "nd");
        assert_eq!(ordinal(-3), "rd");
        assert_eq!(ordinal(-11), "th");
        assert_eq!(ordinal(-1021), "st");
    }

    #[test]
    fn test_ordinalize() {
        assert_eq!(ordinalize(0), "0th");
        assert_eq!(ordinalize(1), "1st");
        assert_eq!(ordinalize(2), "2nd");
        assert_eq!(ordinalize(3), "3rd");
        assert_eq!(ordinalize(-11), "-11th");
        assert_eq!(ordinalize(-1021), "-1021st");
        assert_eq!(ordinalize(1002), "1002nd");
        assert_eq!(ordinalize(1003), "1003rd");
    }
}
