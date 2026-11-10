//! Language-aware hyphenation. English breaks long words between syllables at vowel–consonant
//! boundaries (V-CV: "ty-po", and VC-CV: "hap-pen"), never leaving fewer than [`MIN_BEFORE`]
//! letters before or [`MIN_AFTER`] after the hyphen.
//!
//! Preferences › Hyphenation › Exceptions (comma separated) override the points for named words:
//! `typography` never breaks, `ty-pog-ra-phy` breaks only where the hyphens mark. See
//! [`set_hyphenation_exceptions`]. Eleven Indic languages use licensed bundled patterns;
//! other languages use explicit exceptions. All breaks retain complete graphemes.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock};

mod indic;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Language {
    None,
    English,
    Indic(u8),
}
impl Language {
    pub(crate) fn from_tag(tag: Option<&str>) -> Self {
        if tag.is_some_and(|s| s.is_empty() || s.len() > 35 || !s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')) {
            return Self::None;
        }
        let primary = tag.unwrap_or("en").split('-').next().unwrap_or_default();
        if primary.eq_ignore_ascii_case("en") {
            Self::English
        } else {
            indic::TAGS.iter().position(|t| primary.eq_ignore_ascii_case(t)).and_then(|i| u8::try_from(i).ok()).map_or(Self::None, Self::Indic)
        }
    }
}

/// Letters, attached marks and joiners belong to one hyphenation word.
pub(crate) fn is_word_char(ch: char) -> bool {
    use unicode_general_category::{GeneralCategory, get_general_category};
    ch.is_alphabetic()
        || matches!(ch, '\'' | '’' | '\u{200c}' | '\u{200d}')
        || matches!(get_general_category(ch), GeneralCategory::NonspacingMark | GeneralCategory::SpacingMark | GeneralCategory::EnclosingMark)
}

/// Whether a document-text language has automatic patterns; unsupported languages use exceptions.
pub fn language_has_patterns(language: Option<&str>) -> bool {
    !matches!(Language::from_tag(language), Language::None)
}

/// Language-specific character-index breaks, using the existing 6/3/3 paragraph limits.
pub fn hyphen_points_for_language(word: &str, language: Option<&str>) -> Vec<usize> {
    points_for(word, Language::from_tag(language))
}

pub(crate) fn points_for(word: &str, language: Language) -> Vec<usize> {
    if let Some(ex) = exception_for(word) {
        let mut points = match ex {
            Exception::Never => vec![],
            Exception::Only(points) => points,
        };
        indic::retain_boundaries(word, &mut points);
        return points;
    }
    match language {
        Language::None => vec![],
        Language::English => english_points(word),
        Language::Indic(index) => {
            let n = word.chars().count();
            if n < MIN_WORD || !word.chars().all(is_word_char) {
                return vec![];
            }
            let mut points = indic::points(word, usize::from(index));
            points.retain(|p| *p >= MIN_BEFORE && n.saturating_sub(*p) >= MIN_AFTER);
            indic::retain_boundaries(word, &mut points);
            points
        }
    }
}

/// Only words at least this long are hyphenated (Illustrator's default: longer than 5 letters).
pub const MIN_WORD: usize = 6;
pub const MIN_BEFORE: usize = 3;
pub const MIN_AFTER: usize = 3;

/// An exception for one word: never break it, or break only at the listed character indices.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Exception {
    Never,
    Only(Vec<usize>),
}

#[derive(Default)]
struct Exceptions {
    source: String,
    map: HashMap<String, Exception>,
}

fn store() -> MutexGuard<'static, Exceptions> {
    static LOCK: OnceLock<Mutex<Exceptions>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(Exceptions::default())).lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn parse_list(source: &str) -> Exceptions {
    let mut map = HashMap::new();
    for entry in source.split(',') {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }
        let (word, ex) = parse_exception(entry);
        if word.is_empty() {
            continue;
        }
        map.insert(word, ex);
    }
    Exceptions { source: source.to_string(), map }
}

/// Preferences › Hyphenation › Exceptions: comma-separated words. A word with no hyphens is never
/// hyphenated; hyphens mark the only allowed break points (`ty-pog-ra-phy`). Matching is
/// case-insensitive. Empty clears the list.
pub fn set_hyphenation_exceptions(source: &str) {
    *store() = parse_list(source);
}

/// The last string passed to [`set_hyphenation_exceptions`] (empty when none).
pub fn hyphenation_exceptions() -> String {
    store().source.clone()
}

fn parse_exception(entry: &str) -> (String, Exception) {
    // Illustrator: bare word = never break; hyphens mark the only allowed breaks. No other syntax.
    if !entry.contains('-') {
        return (normalize_word(entry), Exception::Never);
    }
    let mut word = String::new();
    let mut points = Vec::new();
    for ch in entry.chars() {
        if ch == '-' {
            let at = word.chars().count();
            if at > 0 {
                points.push(at);
            }
        } else {
            for c in ch.to_lowercase() {
                word.push(c);
            }
        }
    }
    let n = word.chars().count();
    points.retain(|&p| p > 0 && p < n);
    points.sort_unstable();
    points.dedup();
    (word, Exception::Only(points))
}

fn normalize_word(word: &str) -> String {
    word.chars().flat_map(|c| c.to_lowercase()).collect()
}

/// Look the word up under the preferences lock. Only the matched [`Exception`] is cloned, never
/// the whole list.
#[cfg(not(test))]
fn exception_for(word: &str) -> Option<Exception> {
    store().map.get(&normalize_word(word)).cloned()
}

/// Unit tests read only their own thread's list: tests running in parallel set the process-wide
/// one.
#[cfg(test)]
fn exception_for(word: &str) -> Option<Exception> {
    TEST_OVERRIDE.with(|c| c.borrow().as_ref().and_then(|e| e.map.get(&normalize_word(word)).cloned()))
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' | 'y') || (c.is_alphabetic() && !c.is_ascii())
}

/// Consonant pairs that stay together at the start of a syllable ("gra-phy", "tea-cher").
fn onset_pair(a: char, b: char) -> bool {
    let (a, b) = (a.to_ascii_lowercase(), b.to_ascii_lowercase());
    matches!(
        (a, b),
        ('c' | 's' | 't' | 'p' | 'w' | 'g', 'h')
            | ('b' | 'c' | 'd' | 'f' | 'g' | 'k' | 'p' | 't', 'r')
            | ('b' | 'c' | 'f' | 'g' | 'k' | 'p' | 's', 'l')
            | ('q', 'u')
    )
}

/// Allowed hyphenation points in `word`, as char indices (a hyphen goes *before* the char).
/// Words containing non-letters (other than a trailing apostrophe) are not hyphenated.
/// Preferences exceptions override the pattern for a matching word.
pub fn hyphen_points(word: &str) -> Vec<usize> {
    points_for(word, Language::English)
}

fn english_points(word: &str) -> Vec<usize> {
    let c: Vec<char> = word.chars().collect();
    let n = c.len();
    if n < MIN_WORD || !c.iter().all(|c| c.is_alphabetic() || matches!(c, '\'' | '’')) {
        return vec![];
    }
    // A capitalised run like "NASA" or a proper name stays whole only when all caps.
    if c.iter().all(|c| c.is_uppercase()) {
        return vec![];
    }
    let v: Vec<bool> = c.iter().map(|&x| is_vowel(x)).collect();
    let mut out = vec![];
    for i in MIN_BEFORE..=n.saturating_sub(MIN_AFTER) {
        if !c[i].is_alphabetic() || !c[i - 1].is_alphabetic() {
            continue;
        }
        let ok = if v[i - 1] && !v[i] {
            // V-CV: a single consonant (or an onset pair) followed by a vowel starts the syllable.
            (i + 1 < n && v[i + 1]) || (i + 2 < n && onset_pair(c[i], c[i + 1]) && v[i + 2])
        } else if !v[i - 1] && !v[i] {
            // VC-CV: split a consonant cluster after the first consonant unless it's an onset pair.
            i >= 2 && v[i - 2] && i + 1 < n && v[i + 1] && !onset_pair(c[i - 1], c[i])
        } else {
            false
        };
        if ok {
            out.push(i);
        }
    }
    out
}

/// `word` with `-` at every hyphenation point (for tests and diagnostics).
pub fn hyphenate_word(word: &str) -> String {
    let pts = hyphen_points(word);
    let mut s = String::with_capacity(word.len() + pts.len());
    for (i, ch) in word.chars().enumerate() {
        if pts.contains(&i) {
            s.push('-');
        }
        s.push(ch);
    }
    s
}

#[cfg(test)]
thread_local! {
    /// Per-thread list so unit tests don't race the process-wide preferences store.
    static TEST_OVERRIDE: std::cell::RefCell<Option<Exceptions>> = const { std::cell::RefCell::new(None) };
}

/// Run `f` with Preferences › Hyphenation › Exceptions applied on this thread only (tests).
#[cfg(test)]
pub(crate) fn with_hyphenation_exceptions_for_test(source: &str, f: impl FnOnce()) {
    TEST_OVERRIDE.with(|c| *c.borrow_mut() = Some(parse_list(source)));
    f();
    TEST_OVERRIDE.with(|c| *c.borrow_mut() = None);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_exceptions(source: &str, f: impl FnOnce()) {
        with_hyphenation_exceptions_for_test(source, f);
    }

    /// #394 Preferences › Hyphenation › Exceptions contract (parse + override).
    #[test]
    fn hyphenation_exceptions_override_the_pattern() {
        let pattern = hyphen_points("typography");
        assert_eq!(pattern, vec![4, 7], "pattern: typo-gra-phy");

        // Bare word: never break (case-insensitive).
        with_exceptions("typography", || {
            assert!(hyphen_points("typography").is_empty());
            assert!(hyphen_points("Typography").is_empty(), "case-insensitive");
        });

        // Hyphens mark the only allowed breaks (stricter than the pattern).
        with_exceptions("typo-graphy", || {
            assert_eq!(hyphen_points("typography"), vec![4], "only the marked break");
            assert_eq!(hyphenate_word("typography"), "typo-graphy");
        });

        // Comma-separated list; empty / whitespace entries ignored.
        with_exceptions("  , typography , hap-pen ,  ", || {
            assert!(hyphen_points("typography").is_empty());
            assert_eq!(hyphen_points("happen"), vec![3]);
            assert!(!hyphen_points("hyphenation").is_empty(), "unlisted words still use the pattern");
        });

        assert_eq!(hyphen_points("typography"), pattern, "override cleared");
    }

    /// Process-wide store used by `prefs.set` / restore.
    #[test]
    fn set_hyphenation_exceptions_round_trips_the_source_string() {
        set_hyphenation_exceptions("typography, hap-pen");
        assert_eq!(hyphenation_exceptions(), "typography, hap-pen");
        set_hyphenation_exceptions("");
        assert!(hyphenation_exceptions().is_empty());
    }

    #[test]
    fn indic_patterns_preserve_syllables_and_current_margins() {
        for (tag, syllable) in [
            ("hi", "कि"),
            ("mr", "कि"),
            ("gu", "કિ"),
            ("bn", "কি"),
            ("pa", "ਕਿ"),
            ("ta", "கி"),
            ("te", "కి"),
            ("kn", "ಕಿ"),
            ("ml", "കി"),
            ("or", "କି"),
            ("as", "কি"),
        ] {
            let word = syllable.repeat(4);
            assert_eq!(hyphen_points_for_language(&word, Some(tag)), [4], "{tag}");
            assert_eq!(hyphen_points_for_language(&word, Some(&format!("{}-IN", tag.to_uppercase()))), [4]);
            assert!(language_has_patterns(Some(tag)));
            assert!(hyphen_points_for_language(&word, Some("zz")).is_empty());
        }
        assert!(!language_has_patterns(Some("sa")));
        assert!(hyphen_points_for_language("typography", Some("sa")).is_empty());
    }

    #[test]
    fn indic_marks_conjuncts_joiners_and_exceptions_are_never_split() {
        use unicode_segmentation::UnicodeSegmentation;
        for word in ["अक्षरअक्षर", "क्\u{200d}षक्\u{200d}ष", "क्\u{200c}षक्\u{200c}ष", "किंकिंकिं", "क़िक़िक़ि", "किकि123कि"]
        {
            let boundaries: Vec<_> = word.grapheme_indices(true).map(|(b, _)| word[..b].chars().count()).collect();
            assert!(hyphen_points_for_language(word, Some("hi")).iter().all(|p| boundaries.contains(p)), "{word}");
        }
        with_exceptions("क-ि-क्-ष-क-ि", || {
            assert_eq!(hyphen_points_for_language("किक्षकि", Some("hi")), [2, 5]);
            assert_eq!(hyphen_points_for_language("किक्षकि", Some("sa")), [2, 5]);
        });
        with_exceptions("कि-किकिकि", || {
            assert_eq!(hyphen_points_for_language("किकिकिकि", Some("sa")), [2]);
        });
    }
}
