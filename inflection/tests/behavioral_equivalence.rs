//! Behavioral equivalence tests.
//!
//! Every test case from the original Python `test_inflection.py` is reproduced
//! here 1:1 to prove behavioral equivalence between the Rust and Python
//! implementations.

use inflection::*;

// ═══════════════════════════════════════════════════════════════════════════════
// SINGULAR ↔ PLURAL (from SINGULAR_TO_PLURAL in Python test suite)
// ═══════════════════════════════════════════════════════════════════════════════

/// All singular/plural pairs from the Python test suite.
/// Each tuple is (singular, expected_plural).
const SINGULAR_TO_PLURAL: &[(&str, &str)] = &[
    ("search", "searches"),
    ("switch", "switches"),
    ("fix", "fixes"),
    ("box", "boxes"),
    ("process", "processes"),
    ("address", "addresses"),
    ("case", "cases"),
    ("stack", "stacks"),
    ("wish", "wishes"),
    ("fish", "fish"),
    ("jeans", "jeans"),
    ("funky jeans", "funky jeans"),
    ("category", "categories"),
    ("query", "queries"),
    ("ability", "abilities"),
    ("agency", "agencies"),
    ("movie", "movies"),
    ("archive", "archives"),
    ("index", "indices"),
    ("wife", "wives"),
    ("safe", "saves"),
    ("half", "halves"),
    ("move", "moves"),
    ("salesperson", "salespeople"),
    ("person", "people"),
    ("spokesman", "spokesmen"),
    ("man", "men"),
    ("woman", "women"),
    ("basis", "bases"),
    ("diagnosis", "diagnoses"),
    ("diagnosis_a", "diagnosis_as"),
    ("datum", "data"),
    ("medium", "media"),
    ("stadium", "stadia"),
    ("analysis", "analyses"),
    ("node_child", "node_children"),
    ("child", "children"),
    ("experience", "experiences"),
    ("day", "days"),
    ("comment", "comments"),
    ("foobar", "foobars"),
    ("newsletter", "newsletters"),
    ("old_news", "old_news"),
    ("news", "news"),
    ("series", "series"),
    ("species", "species"),
    ("quiz", "quizzes"),
    ("perspective", "perspectives"),
    ("ox", "oxen"),
    ("passerby", "passersby"),
    ("photo", "photos"),
    ("buffalo", "buffaloes"),
    ("tomato", "tomatoes"),
    ("potato", "potatoes"),
    ("dwarf", "dwarves"),
    ("elf", "elves"),
    ("information", "information"),
    ("equipment", "equipment"),
    ("bus", "buses"),
    ("status", "statuses"),
    ("status_code", "status_codes"),
    ("mouse", "mice"),
    ("louse", "lice"),
    ("house", "houses"),
    ("octopus", "octopi"),
    ("virus", "viri"),
    ("alias", "aliases"),
    ("portfolio", "portfolios"),
    ("vertex", "vertices"),
    ("matrix", "matrices"),
    ("matrix_fu", "matrix_fus"),
    ("axis", "axes"),
    ("testis", "testes"),
    ("crisis", "crises"),
    ("rice", "rice"),
    ("shoe", "shoes"),
    ("horse", "horses"),
    ("prize", "prizes"),
    ("edge", "edges"),
    ("cow", "kine"),
    ("database", "databases"),
    ("human", "humans"),
];

#[test]
fn test_pluralize_singular() {
    for (singular, expected_plural) in SINGULAR_TO_PLURAL {
        assert_eq!(
            &pluralize(singular),
            expected_plural,
            "pluralize({:?}) should be {:?}",
            singular,
            expected_plural
        );
    }
}

#[test]
fn test_pluralize_singular_capitalized() {
    for (singular, expected_plural) in SINGULAR_TO_PLURAL {
        let capitalized_singular = capitalize(singular);
        let capitalized_plural = capitalize(expected_plural);
        assert_eq!(
            pluralize(&capitalized_singular),
            capitalized_plural,
            "pluralize({:?}) should be {:?}",
            capitalized_singular,
            capitalized_plural
        );
    }
}

#[test]
fn test_singularize_plural() {
    for (expected_singular, plural) in SINGULAR_TO_PLURAL {
        assert_eq!(
            &singularize(plural),
            expected_singular,
            "singularize({:?}) should be {:?}",
            plural,
            expected_singular
        );
    }
}

#[test]
fn test_singularize_plural_capitalized() {
    for (expected_singular, plural) in SINGULAR_TO_PLURAL {
        let capitalized_plural = capitalize(plural);
        let capitalized_singular = capitalize(expected_singular);
        assert_eq!(
            singularize(&capitalized_plural),
            capitalized_singular,
            "singularize({:?}) should be {:?}",
            capitalized_plural,
            capitalized_singular
        );
    }
}

#[test]
fn test_pluralize_plural() {
    for (_singular, plural) in SINGULAR_TO_PLURAL {
        assert_eq!(
            &pluralize(plural),
            plural,
            "pluralize({:?}) should be {:?} (already plural)",
            plural,
            plural
        );
    }
}

#[test]
fn test_pluralize_plural_capitalized() {
    for (_singular, plural) in SINGULAR_TO_PLURAL {
        let capitalized = capitalize(plural);
        assert_eq!(
            pluralize(&capitalized),
            capitalized,
            "pluralize({:?}) should be {:?} (already plural, capitalized)",
            capitalized,
            capitalized
        );
    }
}

#[test]
fn test_pluralize_plurals_word() {
    assert_eq!(pluralize("plurals"), "plurals");
    assert_eq!(pluralize("Plurals"), "Plurals");
}

#[test]
fn test_pluralize_empty_string() {
    assert_eq!(pluralize(""), "");
}

// ═══════════════════════════════════════════════════════════════════════════════
// UNCOUNTABILITY
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_uncountability() {
    let uncountables = [
        "equipment",
        "fish",
        "information",
        "jeans",
        "money",
        "rice",
        "series",
        "sheep",
        "species",
    ];
    for word in uncountables {
        assert_eq!(singularize(word), word, "singularize({:?})", word);
        assert_eq!(pluralize(word), word, "pluralize({:?})", word);
        assert_eq!(
            pluralize(word),
            singularize(word),
            "pluralize == singularize for {:?}",
            word
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CAMELCASE ↔ UNDERSCORE
// ═══════════════════════════════════════════════════════════════════════════════

const CAMEL_TO_UNDERSCORE: &[(&str, &str)] = &[
    ("Product", "product"),
    ("SpecialGuest", "special_guest"),
    ("ApplicationController", "application_controller"),
    ("Area51Controller", "area51_controller"),
];

const CAMEL_TO_UNDERSCORE_WITHOUT_REVERSE: &[(&str, &str)] = &[
    ("HTMLTidy", "html_tidy"),
    ("HTMLTidyGenerator", "html_tidy_generator"),
    ("FreeBSD", "free_bsd"),
    ("HTML", "html"),
];

#[test]
fn test_camelize() {
    for (camel, under) in CAMEL_TO_UNDERSCORE {
        assert_eq!(
            &camelize(under, true),
            camel,
            "camelize({:?}, true) should be {:?}",
            under,
            camel
        );
    }
}

#[test]
fn test_camelize_with_lower() {
    assert_eq!(camelize("Capital", false), "capital");
}

#[test]
fn test_camelize_with_underscores() {
    assert_eq!(camelize("Camel_Case", true), "CamelCase");
}

#[test]
fn test_camelize_empty_string_bug_fix() {
    // Python original crashes with IndexError on camelize("", False)
    assert_eq!(camelize("", true), "");
    assert_eq!(camelize("", false), "");
}

#[test]
fn test_underscore() {
    let all_cases: Vec<(&str, &str)> = CAMEL_TO_UNDERSCORE
        .iter()
        .chain(CAMEL_TO_UNDERSCORE_WITHOUT_REVERSE.iter())
        .copied()
        .collect();

    for (camel, expected_under) in all_cases {
        assert_eq!(
            &underscore(camel),
            expected_under,
            "underscore({:?}) should be {:?}",
            camel,
            expected_under
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PARAMETERIZE
// ═══════════════════════════════════════════════════════════════════════════════

const STRING_TO_PARAMETERIZED: &[(&str, &str)] = &[
    ("Donald E. Knuth", "donald-e-knuth"),
    (
        "Random text with *(bad)* characters",
        "random-text-with-bad-characters",
    ),
    ("Allow_Under_Scores", "allow_under_scores"),
    ("Trailing bad characters!@#", "trailing-bad-characters"),
    ("!@#Leading bad characters", "leading-bad-characters"),
    ("Squeeze   separators", "squeeze-separators"),
    ("Test with + sign", "test-with-sign"),
    (
        "Test with malformed utf8 \u{00A9}",
        "test-with-malformed-utf8",
    ),
];

const STRING_TO_PARAMETERIZE_NO_SEP: &[(&str, &str)] = &[
    ("Donald E. Knuth", "donaldeknuth"),
    ("With-some-dashes", "with-some-dashes"),
    (
        "Random text with *(bad)* characters",
        "randomtextwithbadcharacters",
    ),
    ("Trailing bad characters!@#", "trailingbadcharacters"),
    ("!@#Leading bad characters", "leadingbadcharacters"),
    ("Squeeze   separators", "squeezeseparators"),
    ("Test with + sign", "testwithsign"),
    ("Test with malformed utf8 \u{00A9}", "testwithmalformedutf8"),
];

const STRING_TO_PARAMETERIZE_UNDERSCORE: &[(&str, &str)] = &[
    ("Donald E. Knuth", "donald_e_knuth"),
    (
        "Random text with *(bad)* characters",
        "random_text_with_bad_characters",
    ),
    ("With-some-dashes", "with-some-dashes"),
    ("Retain_underscore", "retain_underscore"),
    ("Trailing bad characters!@#", "trailing_bad_characters"),
    ("!@#Leading bad characters", "leading_bad_characters"),
    ("Squeeze   separators", "squeeze_separators"),
    ("Test with + sign", "test_with_sign"),
    (
        "Test with malformed utf8 \u{00A9}",
        "test_with_malformed_utf8",
    ),
];

#[test]
fn test_parameterize_default() {
    for (input, expected) in STRING_TO_PARAMETERIZED {
        assert_eq!(
            &parameterize(input, "-"),
            expected,
            "parameterize({:?}, \"-\") should be {:?}",
            input,
            expected
        );
    }
}

#[test]
fn test_parameterize_no_separator() {
    for (input, expected) in STRING_TO_PARAMETERIZE_NO_SEP {
        assert_eq!(
            &parameterize(input, ""),
            expected,
            "parameterize({:?}, \"\") should be {:?}",
            input,
            expected
        );
    }
}

#[test]
fn test_parameterize_underscore_separator() {
    for (input, expected) in STRING_TO_PARAMETERIZE_UNDERSCORE {
        assert_eq!(
            &parameterize(input, "_"),
            expected,
            "parameterize({:?}, \"_\") should be {:?}",
            input,
            expected
        );
    }
}

#[test]
fn test_parameterize_multi_char_separator() {
    for (input, expected) in STRING_TO_PARAMETERIZED {
        let expected_multi = expected.replace('-', "__sep__");
        assert_eq!(
            parameterize(input, "__sep__"),
            expected_multi,
            "parameterize({:?}, \"__sep__\") should be {:?}",
            input,
            expected_multi
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HUMANIZE
// ═══════════════════════════════════════════════════════════════════════════════

const UNDERSCORE_TO_HUMAN: &[(&str, &str)] = &[
    ("employee_salary", "Employee salary"),
    ("employee_id", "Employee"),
    ("underground", "Underground"),
];

#[test]
fn test_humanize() {
    for (input, expected) in UNDERSCORE_TO_HUMAN {
        assert_eq!(
            &humanize(input),
            expected,
            "humanize({:?}) should be {:?}",
            input,
            expected
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TITLEIZE
// ═══════════════════════════════════════════════════════════════════════════════

const MIXTURE_TO_TITLEIZED: &[(&str, &str)] = &[
    ("active_record", "Active Record"),
    ("ActiveRecord", "Active Record"),
    ("action web service", "Action Web Service"),
    ("Action Web Service", "Action Web Service"),
    ("Action web service", "Action Web Service"),
    ("actionwebservice", "Actionwebservice"),
    ("Actionwebservice", "Actionwebservice"),
    ("david's code", "David's Code"),
    ("David's code", "David's Code"),
    ("david's Code", "David's Code"),
];

#[test]
fn test_titleize() {
    for (input, expected) in MIXTURE_TO_TITLEIZED {
        assert_eq!(
            &titleize(input),
            expected,
            "titleize({:?}) should be {:?}",
            input,
            expected
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ORDINAL / ORDINALIZE
// ═══════════════════════════════════════════════════════════════════════════════

const ORDINAL_NUMBERS: &[(i64, &str)] = &[
    (-1, "-1st"),
    (-2, "-2nd"),
    (-3, "-3rd"),
    (-4, "-4th"),
    (-5, "-5th"),
    (-6, "-6th"),
    (-7, "-7th"),
    (-8, "-8th"),
    (-9, "-9th"),
    (-10, "-10th"),
    (-11, "-11th"),
    (-12, "-12th"),
    (-13, "-13th"),
    (-14, "-14th"),
    (-20, "-20th"),
    (-21, "-21st"),
    (-22, "-22nd"),
    (-23, "-23rd"),
    (-24, "-24th"),
    (-100, "-100th"),
    (-101, "-101st"),
    (-102, "-102nd"),
    (-103, "-103rd"),
    (-104, "-104th"),
    (-110, "-110th"),
    (-111, "-111th"),
    (-112, "-112th"),
    (-113, "-113th"),
    (-1000, "-1000th"),
    (-1001, "-1001st"),
    (0, "0th"),
    (1, "1st"),
    (2, "2nd"),
    (3, "3rd"),
    (4, "4th"),
    (5, "5th"),
    (6, "6th"),
    (7, "7th"),
    (8, "8th"),
    (9, "9th"),
    (10, "10th"),
    (11, "11th"),
    (12, "12th"),
    (13, "13th"),
    (14, "14th"),
    (20, "20th"),
    (21, "21st"),
    (22, "22nd"),
    (23, "23rd"),
    (24, "24th"),
    (100, "100th"),
    (101, "101st"),
    (102, "102nd"),
    (103, "103rd"),
    (104, "104th"),
    (110, "110th"),
    (111, "111th"),
    (112, "112th"),
    (113, "113th"),
    (1000, "1000th"),
    (1001, "1001st"),
];

#[test]
fn test_ordinal() {
    for (number, expected_ordinalized) in ORDINAL_NUMBERS {
        let expected_suffix = &expected_ordinalized[expected_ordinalized
            .find(|c: char| c.is_alphabetic())
            .unwrap()..];
        assert_eq!(
            ordinal(*number),
            expected_suffix,
            "ordinal({}) should be {:?}",
            number,
            expected_suffix
        );
    }
}

#[test]
fn test_ordinalize() {
    for (number, expected) in ORDINAL_NUMBERS {
        assert_eq!(
            &ordinalize(*number),
            expected,
            "ordinalize({}) should be {:?}",
            number,
            expected
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// DASHERIZE
// ═══════════════════════════════════════════════════════════════════════════════

const UNDERSCORES_TO_DASHES: &[(&str, &str)] = &[
    ("street", "street"),
    ("street_address", "street-address"),
    ("person_street_address", "person-street-address"),
];

#[test]
fn test_dasherize() {
    for (input, expected) in UNDERSCORES_TO_DASHES {
        assert_eq!(
            &dasherize(input),
            expected,
            "dasherize({:?}) should be {:?}",
            input,
            expected
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TABLEIZE
// ═══════════════════════════════════════════════════════════════════════════════

const STRING_TO_TABLEIZE: &[(&str, &str)] = &[
    ("person", "people"),
    ("Country", "countries"),
    ("ChildToy", "child_toys"),
    ("_RecipeIngredient", "_recipe_ingredients"),
];

#[test]
fn test_tableize() {
    for (input, expected) in STRING_TO_TABLEIZE {
        assert_eq!(
            &tableize(input),
            expected,
            "tableize({:?}) should be {:?}",
            input,
            expected
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TRANSLITERATE
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_transliterate() {
    assert_eq!(transliterate("älämölö"), "alamolo");
    assert_eq!(transliterate("Ærøskøbing"), "rskbing");
}

// ═══════════════════════════════════════════════════════════════════════════════
// EDGE CASES & BUG FIX TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_camelize_empty_does_not_panic() {
    // Bug fix: Python original raises IndexError on camelize("", False)
    assert_eq!(camelize("", false), "");
    assert_eq!(camelize("", true), "");
}

#[test]
fn test_singularize_oxen_with_anchor() {
    // Bug fix: Python original is missing $ anchor on the "oxen" rule
    assert_eq!(singularize("oxen"), "ox");
}

#[test]
fn test_funky_jeans_uncountable_not_greedy() {
    // "jeans" is uncountable, but "funky jeans" should also be uncountable
    assert_eq!(pluralize("funky jeans"), "funky jeans");
}

#[test]
fn test_woman_pluralize() {
    // Woman is an interesting irregular: different first letter sound
    assert_eq!(pluralize("woman"), "women");
}

#[test]
fn test_news_compound() {
    assert_eq!(pluralize("old_news"), "old_news");
    assert_eq!(pluralize("news"), "news");
}

#[test]
fn test_status_code_compound() {
    // "status" is special, but "status_code" should pluralize normally
    assert_eq!(pluralize("status_code"), "status_codes");
}

#[test]
fn test_matrix_fu_compound() {
    // "matrix" → "matrices", but "matrix_fu" → "matrix_fus"
    assert_eq!(pluralize("matrix_fu"), "matrix_fus");
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER
// ═══════════════════════════════════════════════════════════════════════════════

/// Python-compatible capitalize: uppercase first char, leave rest unchanged.
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => {
            let upper: String = c.to_uppercase().collect();
            upper + chars.as_str()
        }
        None => String::new(),
    }
}
