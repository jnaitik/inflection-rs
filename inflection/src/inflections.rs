//! English singular/plural inflection rules.
//!
//! Provides [`pluralize`], [`singularize`], and [`tableize`] using ordered
//! regex-based replacement rules ported from the Python `inflection` library
//! (which itself ports Ruby on Rails' `ActiveSupport::Inflector`).
//!
//! All rules are stored as immutable statics, making this module thread-safe
//! (unlike the Python original which uses mutable global lists).

use regex::Regex;
use std::collections::HashSet;
use std::sync::LazyLock;

use crate::underscore;

/// A compiled inflection rule: a regex pattern and its replacement string.
struct Rule {
    pattern: Regex,
    replacement: &'static str,
}

impl Rule {
    fn new(pattern: &str, replacement: &'static str) -> Self {
        Self {
            pattern: Regex::new(pattern).unwrap(),
            replacement,
        }
    }
}

/// Apply the first matching rule from a list of rules to a word.
/// Returns the transformed word, or the original word if no rule matches.
fn apply_rules(word: &str, rules: &[Rule]) -> String {
    for rule in rules {
        if rule.pattern.is_match(word) {
            return rule.pattern.replace(word, rule.replacement).into_owned();
        }
    }
    word.to_owned()
}

// ─── Pluralization rules ──────────────────────────────────────────────────────
//
// These are the exact rules from the Python original, in the exact same order.
// The irregular words (person/people, man/men, etc.) are prepended at the front
// just like `_irregular()` does at module load time.
//
// The order matters: rules are tried first-to-last, first match wins.

static PLURALS: LazyLock<Vec<Rule>> = LazyLock::new(|| {
    vec![
        // ── Irregulars (inserted by _irregular() in Python) ──────────
        // _irregular('zombie', 'zombies') — same first letter
        Rule::new(r"(?i)(z)ombies$", r"${1}ombies"),
        Rule::new(r"(?i)(z)ombie$", r"${1}ombies"),
        // _irregular('cow', 'kine') — different first letter
        // Python inserts these with insert(0), so last-inserted = first-checked.
        // Plural rules (in check order):
        Rule::new(r"k[iI][nN][eE]$", r"kine"),       // kine/kIne/etc → kine
        Rule::new(r"K[iI][nN][eE]$", r"Kine"),       // Kine/KIne/etc → Kine
        Rule::new(r"c[oO][wW]$", r"kine"),            // cow/coW/etc → kine
        Rule::new(r"C[oO][wW]$", r"Kine"),            // Cow/COW/etc → Kine
        // _irregular('move', 'moves') — same first letter
        Rule::new(r"(?i)(m)oves$", r"${1}oves"),
        Rule::new(r"(?i)(m)ove$", r"${1}oves"),
        // _irregular('sex', 'sexes') — same first letter
        Rule::new(r"(?i)(s)exes$", r"${1}exes"),
        Rule::new(r"(?i)(s)ex$", r"${1}exes"),
        // _irregular('child', 'children') — same first letter
        Rule::new(r"(?i)(c)hildren$", r"${1}hildren"),
        Rule::new(r"(?i)(c)hild$", r"${1}hildren"),
        // _irregular('human', 'humans') — same first letter
        Rule::new(r"(?i)(h)umans$", r"${1}umans"),
        Rule::new(r"(?i)(h)uman$", r"${1}umans"),
        // _irregular('man', 'men') — same first letter
        Rule::new(r"(?i)(m)en$", r"${1}en"),
        Rule::new(r"(?i)(m)an$", r"${1}en"),
        // _irregular('person', 'people') — same first letter
        Rule::new(r"(?i)(p)eople$", r"${1}eople"),
        Rule::new(r"(?i)(p)erson$", r"${1}eople"),
        // ── Base rules (from PLURALS list in Python) ─────────────────
        Rule::new(r"(?i)(quiz)$", r"${1}zes"),
        Rule::new(r"(?i)^(oxen)$", r"${1}"),
        Rule::new(r"(?i)^(ox)$", r"${1}en"),
        Rule::new(r"(?i)(m|l)ice$", r"${1}ice"),
        Rule::new(r"(?i)(m|l)ouse$", r"${1}ice"),
        Rule::new(r"(?i)(passer)s?by$", r"${1}sby"),
        Rule::new(r"(?i)(matr|vert|ind)(?:ix|ex)$", r"${1}ices"),
        Rule::new(r"(?i)(x|ch|ss|sh)$", r"${1}es"),
        Rule::new(r"(?i)([^aeiouy]|qu)y$", r"${1}ies"),
        Rule::new(r"(?i)(hive)$", r"${1}s"),
        Rule::new(r"(?i)([lr])f$", r"${1}ves"),
        Rule::new(r"(?i)([^f])fe$", r"${1}ves"),
        Rule::new(r"(?i)sis$", r"ses"),
        Rule::new(r"(?i)([ti])a$", r"${1}a"),
        Rule::new(r"(?i)([ti])um$", r"${1}a"),
        Rule::new(r"(?i)(buffal|potat|tomat)o$", r"${1}oes"),
        Rule::new(r"(?i)(bu)s$", r"${1}ses"),
        Rule::new(r"(?i)(alias|status)$", r"${1}es"),
        Rule::new(r"(?i)(octop|vir)i$", r"${1}i"),
        Rule::new(r"(?i)(octop|vir)us$", r"${1}i"),
        Rule::new(r"(?i)^(ax|test)is$", r"${1}es"),
        Rule::new(r"(?i)s$", r"s"),
        Rule::new(r"$", r"s"),
    ]
});

// ─── Singularization rules ────────────────────────────────────────────────────

static SINGULARS: LazyLock<Vec<Rule>> = LazyLock::new(|| {
    vec![
        // ── Irregulars (inserted by _irregular() in Python) ──────────
        // _irregular('zombie', 'zombies') — same first letter
        Rule::new(r"(?i)(z)ombies$", r"${1}ombie"),
        // _irregular('cow', 'kine') — different first letter
        Rule::new(r"k[iI][nN][eE]$", r"cow"),         // kine → cow
        Rule::new(r"K[iI][nN][eE]$", r"Cow"),         // Kine → Cow
        // _irregular('move', 'moves') — same first letter
        Rule::new(r"(?i)(m)oves$", r"${1}ove"),
        // _irregular('sex', 'sexes') — same first letter
        Rule::new(r"(?i)(s)exes$", r"${1}ex"),
        // _irregular('child', 'children') — same first letter
        Rule::new(r"(?i)(c)hildren$", r"${1}hild"),
        // _irregular('human', 'humans') — same first letter
        Rule::new(r"(?i)(h)umans$", r"${1}uman"),
        // _irregular('man', 'men') — same first letter
        Rule::new(r"(?i)(m)en$", r"${1}an"),
        // _irregular('person', 'people') — same first letter
        Rule::new(r"(?i)(p)eople$", r"${1}erson"),
        // ── Base rules (from SINGULARS list in Python) ───────────────
        Rule::new(r"(?i)(database)s$", r"${1}"),
        Rule::new(r"(?i)(quiz)zes$", r"${1}"),
        Rule::new(r"(?i)(matr)ices$", r"${1}ix"),
        Rule::new(r"(?i)(vert|ind)ices$", r"${1}ex"),
        Rule::new(r"(?i)(passer)sby$", r"${1}by"),
        // Bug fix: Python original is missing the $ anchor here
        Rule::new(r"(?i)^(ox)en$", r"${1}"),
        Rule::new(r"(?i)(alias|status)(es)?$", r"${1}"),
        Rule::new(r"(?i)(octop|vir)(us|i)$", r"${1}us"),
        Rule::new(r"(?i)^(a)x[ie]s$", r"${1}xis"),
        Rule::new(r"(?i)(cris|test)(is|es)$", r"${1}is"),
        Rule::new(r"(?i)(shoe)s$", r"${1}"),
        Rule::new(r"(?i)(o)es$", r"${1}"),
        Rule::new(r"(?i)(bus)(es)?$", r"${1}"),
        Rule::new(r"(?i)(m|l)ice$", r"${1}ouse"),
        Rule::new(r"(?i)(x|ch|ss|sh)es$", r"${1}"),
        Rule::new(r"(?i)(m)ovies$", r"${1}ovie"),
        Rule::new(r"(?i)(s)eries$", r"${1}eries"),
        Rule::new(r"(?i)([^aeiouy]|qu)ies$", r"${1}y"),
        Rule::new(r"(?i)([lr])ves$", r"${1}f"),
        Rule::new(r"(?i)(tive)s$", r"${1}"),
        Rule::new(r"(?i)(hive)s$", r"${1}"),
        Rule::new(r"(?i)([^f])ves$", r"${1}fe"),
        Rule::new(r"(?i)(t)he(sis|ses)$", r"${1}hesis"),
        Rule::new(r"(?i)(s)ynop(sis|ses)$", r"${1}ynopsis"),
        Rule::new(r"(?i)(p)rogno(sis|ses)$", r"${1}rognosis"),
        Rule::new(r"(?i)(p)arenthe(sis|ses)$", r"${1}arenthesis"),
        Rule::new(r"(?i)(d)iagno(sis|ses)$", r"${1}iagnosis"),
        Rule::new(r"(?i)(b)a(sis|ses)$", r"${1}asis"),
        Rule::new(r"(?i)(a)naly(sis|ses)$", r"${1}nalysis"),
        Rule::new(r"(?i)([ti])a$", r"${1}um"),
        Rule::new(r"(?i)(n)ews$", r"${1}ews"),
        Rule::new(r"(?i)(ss)$", r"${1}"),
        Rule::new(r"(?i)s$", r""),
    ]
});

// ─── Uncountable words ────────────────────────────────────────────────────────

static UNCOUNTABLES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "equipment",
        "fish",
        "information",
        "jeans",
        "money",
        "rice",
        "series",
        "sheep",
        "species",
    ])
});

/// Returns the plural form of an English word.
///
/// Handles regular and irregular plurals, uncountable words, and already-plural
/// words.
///
/// # Examples
///
/// ```
/// use inflection::pluralize;
///
/// assert_eq!(pluralize("post"), "posts");
/// assert_eq!(pluralize("octopus"), "octopi");
/// assert_eq!(pluralize("sheep"), "sheep");
/// assert_eq!(pluralize("CamelOctopus"), "CamelOctopi");
/// assert_eq!(pluralize("search"), "searches");
/// assert_eq!(pluralize(""), "");
/// ```
pub fn pluralize(word: &str) -> String {
    if word.is_empty() || UNCOUNTABLES.contains(word.to_lowercase().as_str()) {
        return word.to_owned();
    }
    apply_rules(word, &PLURALS)
}

/// Returns the singular form of an English word.
///
/// # Examples
///
/// ```
/// use inflection::singularize;
///
/// assert_eq!(singularize("posts"), "post");
/// assert_eq!(singularize("octopi"), "octopus");
/// assert_eq!(singularize("sheep"), "sheep");
/// assert_eq!(singularize("word"), "word");
/// assert_eq!(singularize("CamelOctopi"), "CamelOctopus");
/// ```
pub fn singularize(word: &str) -> String {
    // Check uncountables with word-boundary matching (like Python original)
    for uncountable in UNCOUNTABLES.iter() {
        let pattern = format!(r"(?i)\b({})\z", regex::escape(uncountable));
        if let Ok(re) = Regex::new(&pattern) {
            if re.is_match(word) {
                return word.to_owned();
            }
        }
    }
    apply_rules(word, &SINGULARS)
}

/// Creates a table name from a model class name, like Rails does.
///
/// Underscores and pluralizes the word.
///
/// # Examples
///
/// ```
/// use inflection::tableize;
///
/// assert_eq!(tableize("RawScaledScorer"), "raw_scaled_scorers");
/// assert_eq!(tableize("egg_and_ham"), "egg_and_hams");
/// assert_eq!(tableize("fancyCategory"), "fancy_categories");
/// ```
pub fn tableize(word: &str) -> String {
    pluralize(&underscore(word))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── pluralize ─────────────────────────────────────────────────────

    #[test]
    fn test_pluralize_basic() {
        assert_eq!(pluralize("search"), "searches");
        assert_eq!(pluralize("switch"), "switches");
        assert_eq!(pluralize("fix"), "fixes");
        assert_eq!(pluralize("box"), "boxes");
        assert_eq!(pluralize("process"), "processes");
        assert_eq!(pluralize("address"), "addresses");
        assert_eq!(pluralize("case"), "cases");
        assert_eq!(pluralize("stack"), "stacks");
        assert_eq!(pluralize("wish"), "wishes");
    }

    #[test]
    fn test_pluralize_uncountable() {
        assert_eq!(pluralize("fish"), "fish");
        assert_eq!(pluralize("sheep"), "sheep");
        assert_eq!(pluralize("series"), "series");
        assert_eq!(pluralize("species"), "species");
        assert_eq!(pluralize("equipment"), "equipment");
        assert_eq!(pluralize("information"), "information");
        assert_eq!(pluralize("rice"), "rice");
        assert_eq!(pluralize("money"), "money");
        assert_eq!(pluralize("jeans"), "jeans");
    }

    #[test]
    fn test_pluralize_empty() {
        assert_eq!(pluralize(""), "");
    }

    #[test]
    fn test_pluralize_y_ending() {
        assert_eq!(pluralize("category"), "categories");
        assert_eq!(pluralize("query"), "queries");
        assert_eq!(pluralize("ability"), "abilities");
        assert_eq!(pluralize("agency"), "agencies");
        assert_eq!(pluralize("day"), "days"); // vowel+y → just add s
    }

    #[test]
    fn test_pluralize_irregular() {
        assert_eq!(pluralize("person"), "people");
        assert_eq!(pluralize("man"), "men");
        assert_eq!(pluralize("child"), "children");
        assert_eq!(pluralize("sex"), "sexes");
        assert_eq!(pluralize("move"), "moves");
        assert_eq!(pluralize("cow"), "kine");
        assert_eq!(pluralize("zombie"), "zombies");
        assert_eq!(pluralize("human"), "humans");
    }

    #[test]
    fn test_pluralize_already_plural() {
        assert_eq!(pluralize("plurals"), "plurals");
        assert_eq!(pluralize("Plurals"), "Plurals");
    }

    #[test]
    fn test_pluralize_sis_endings() {
        assert_eq!(pluralize("basis"), "bases");
        assert_eq!(pluralize("diagnosis"), "diagnoses");
        assert_eq!(pluralize("analysis"), "analyses");
    }

    #[test]
    fn test_pluralize_special() {
        assert_eq!(pluralize("octopus"), "octopi");
        assert_eq!(pluralize("virus"), "viri");
        assert_eq!(pluralize("alias"), "aliases");
        assert_eq!(pluralize("status"), "statuses");
        assert_eq!(pluralize("bus"), "buses");
        assert_eq!(pluralize("buffalo"), "buffaloes");
        assert_eq!(pluralize("tomato"), "tomatoes");
        assert_eq!(pluralize("potato"), "potatoes");
        assert_eq!(pluralize("quiz"), "quizzes");
        assert_eq!(pluralize("ox"), "oxen");
        assert_eq!(pluralize("vertex"), "vertices");
        assert_eq!(pluralize("matrix"), "matrices");
        assert_eq!(pluralize("index"), "indices");
    }

    #[test]
    fn test_pluralize_f_fe_endings() {
        assert_eq!(pluralize("wife"), "wives");
        assert_eq!(pluralize("half"), "halves");
        assert_eq!(pluralize("elf"), "elves");
        assert_eq!(pluralize("dwarf"), "dwarves");
    }

    #[test]
    fn test_pluralize_compound_words() {
        assert_eq!(pluralize("salesperson"), "salespeople");
        assert_eq!(pluralize("spokesman"), "spokesmen");
        assert_eq!(pluralize("node_child"), "node_children");
        assert_eq!(pluralize("passerby"), "passersby");
    }

    #[test]
    fn test_pluralize_preserves_case() {
        assert_eq!(pluralize("CamelOctopus"), "CamelOctopi");
        assert_eq!(pluralize("Search"), "Searches");
    }

    // ── singularize ───────────────────────────────────────────────────

    #[test]
    fn test_singularize_basic() {
        assert_eq!(singularize("searches"), "search");
        assert_eq!(singularize("switches"), "switch");
        assert_eq!(singularize("fixes"), "fix");
        assert_eq!(singularize("boxes"), "box");
        assert_eq!(singularize("processes"), "process");
        assert_eq!(singularize("addresses"), "address");
        assert_eq!(singularize("cases"), "case");
        assert_eq!(singularize("stacks"), "stack");
        assert_eq!(singularize("wishes"), "wish");
    }

    #[test]
    fn test_singularize_uncountable() {
        assert_eq!(singularize("fish"), "fish");
        assert_eq!(singularize("sheep"), "sheep");
        assert_eq!(singularize("series"), "series");
        assert_eq!(singularize("species"), "species");
    }

    #[test]
    fn test_singularize_irregular() {
        assert_eq!(singularize("people"), "person");
        assert_eq!(singularize("men"), "man");
        assert_eq!(singularize("children"), "child");
        assert_eq!(singularize("sexes"), "sex");
        assert_eq!(singularize("moves"), "move");
    }

    #[test]
    fn test_singularize_preserves_singular() {
        assert_eq!(singularize("word"), "word");
    }

    #[test]
    fn test_singularize_preserves_case() {
        assert_eq!(singularize("CamelOctopi"), "CamelOctopus");
    }

    #[test]
    fn test_singularize_sis_endings() {
        assert_eq!(singularize("bases"), "basis");
        assert_eq!(singularize("diagnoses"), "diagnosis");
        assert_eq!(singularize("analyses"), "analysis");
    }

    // ── tableize ──────────────────────────────────────────────────────

    #[test]
    fn test_tableize() {
        assert_eq!(tableize("RawScaledScorer"), "raw_scaled_scorers");
        assert_eq!(tableize("egg_and_ham"), "egg_and_hams");
        assert_eq!(tableize("fancyCategory"), "fancy_categories");
    }
}
