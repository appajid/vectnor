//! Indic document text stays independent of the English interface; no new font assets.
use super::*;
use kurbo::Point;
use vectnor_doc::{CharStyle, TextRun};

const SAMPLES: &[(&str, &str, &[&str])] = &[
    ("Devanagari", "कि क्ष", &["Noto Sans Devanagari", "Devanagari Sangam MN"]),
    ("Telugu", "కి క్ష", &["Noto Sans Telugu", "Telugu Sangam MN", "Anek Telugu"]),
    ("Bengali", "কি ক্ষ", &["Noto Sans Bengali", "Bangla Sangam MN"]),
    ("Tamil", "கி க்ஷ", &["Noto Sans Tamil", "Tamil Sangam MN"]),
    ("Kannada", "ಕಿ ಕ್ಷ", &["Noto Sans Kannada", "Kannada Sangam MN"]),
    ("Malayalam", "കി ക്ഷ", &["Noto Sans Malayalam", "Malayalam Sangam MN"]),
    ("Gujarati", "કિ ક્ષ", &["Noto Sans Gujarati", "Gujarati Sangam MN"]),
    ("Gurmukhi", "ਕਿ ਕ੍ਰ", &["Noto Sans Gurmukhi", "Gurmukhi Sangam MN"]),
    ("Odia", "କି କ୍ଷ", &["Noto Sans Oriya", "Oriya Sangam MN"]),
];

#[test]
fn indic_shaping_keeps_marks_clusters_and_equal_style_run_splits() {
    let db = FontDb::global();
    for (script, source, candidates) in SAMPLES {
        let face = candidates
            .iter()
            .filter_map(|name| db.face(name, "Regular"))
            .find(|face| source.chars().filter(|c| !c.is_whitespace()).all(|c| face.covers(c)));
        let Some(face) = face else {
            eprintln!("SKIPPED: {script} shaping; install a suitable font or supply CRAFT_FONTS_DIR");
            continue;
        };
        eprintln!("ASSERTIONS RUN: {script} shaping with {}", face.family);
        let st = CharStyle { font_family: face.family.clone(), font_style: face.style.clone(), size: 32.0, ..Default::default() };
        let t = TextObject::point(Point::ZERO, source, st.clone());
        let whole = layout(db, &t);
        assert!(!whole.glyphs.is_empty(), "{script}");
        assert!(whole.glyphs.iter().all(|g| g.gid != 0), "{script}: missing glyph");
        assert!(whole.glyphs.iter().any(|g| !g.outline.elements().is_empty()), "{script}: missing outlines");
        let clusters: std::collections::BTreeSet<_> = whole.glyphs.iter().map(|g| (g.byte, g.len)).collect();
        assert_eq!(clusters.iter().map(|(_, len)| len).sum::<usize>(), source.len(), "{script}: every source byte retained");
        assert!(clusters.iter().all(|(byte, len)| source.get(*byte..byte + len).is_some()), "{script}: valid UTF-8 clusters");
        assert!(
            clusters.iter().any(|(byte, len)| source.get(*byte..byte + len).is_some_and(|s| s.chars().count() > 1)),
            "{script}: a matra/conjunct stays with its base"
        );
        let mut split = t.clone();
        split.runs = source.chars().map(|c| TextRun::new(c.to_string(), st.clone())).collect();
        let split_layout = layout(db, &split);
        assert_eq!(split.plain_text(), *source);
        assert_eq!(whole.glyphs.len(), split_layout.glyphs.len(), "{script}: equal-style split must not break shaping");
        for (a, b) in whole.glyphs.iter().zip(&split_layout.glyphs) {
            assert_eq!((a.gid, a.byte, a.len, a.advance, a.origin), (b.gid, b.byte, b.len, b.advance, b.origin), "{script}");
        }
    }
}
