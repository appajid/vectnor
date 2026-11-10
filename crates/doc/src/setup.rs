//! Document Setup (File → Document Setup): the bleed around the artboards, view options
//! (transparency grid, simulated paper, images in outline mode, substitution highlights), output
//! options and the document's typographic defaults. Every field is defaulted, so documents saved
//! before Document Setup existed load unchanged.

use serde::{Deserialize, Serialize};
use vectnor_color::Color;
use vectnor_geom::Rect;

use crate::text::ScriptMetrics;

/// Transparency grid cell size.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GridSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl GridSize {
    pub const ALL: [GridSize; 3] = [GridSize::Small, GridSize::Medium, GridSize::Large];
    pub fn id(self) -> &'static str {
        match self {
            GridSize::Small => "small",
            GridSize::Medium => "medium",
            GridSize::Large => "large",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            GridSize::Small => "Small",
            GridSize::Medium => "Medium",
            GridSize::Large => "Large",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|g| g.id().eq_ignore_ascii_case(s))
    }
    /// Edge of one grid cell in screen points (the grid does not scale with the zoom).
    pub fn cell(self) -> f32 {
        match self {
            GridSize::Small => 4.0,
            GridSize::Medium => 8.0,
            GridSize::Large => 16.0,
        }
    }
}

const fn hex(v: u32) -> Color {
    Color::Rgb { r: ((v >> 16) & 0xff) as f32 / 255.0, g: ((v >> 8) & 0xff) as f32 / 255.0, b: (v & 0xff) as f32 / 255.0 }
}

/// Transparency grid colour presets (Vectnor's own pairs). Any other pair is "Custom".
pub const GRID_COLOR_PRESETS: &[(&str, [Color; 2])] = &[
    ("Light", [hex(0xffffff), hex(0xcccccc)]),
    ("Medium", [hex(0xb3b3b3), hex(0x8c8c8c)]),
    ("Dark", [hex(0x666666), hex(0x4d4d4d)]),
    ("Red", [hex(0xffffff), hex(0xffc7c7)]),
    ("Orange", [hex(0xffffff), hex(0xffdcb8)]),
    ("Green", [hex(0xffffff), hex(0xc6efc6)]),
    ("Blue", [hex(0xffffff), hex(0xc4dcff)]),
    ("Purple", [hex(0xffffff), hex(0xe2c9ff)]),
];

/// How text is written to formats without live type (Document Setup → Type → Export).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExportText {
    /// Keep text as text (editable).
    #[default]
    Editable,
    /// Write glyph outlines (exact appearance without the fonts).
    Appearance,
}

impl ExportText {
    pub fn id(self) -> &'static str {
        match self {
            ExportText::Editable => "editable",
            ExportText::Appearance => "appearance",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        [ExportText::Editable, ExportText::Appearance].into_iter().find(|e| e.id().eq_ignore_ascii_case(s))
    }
}

/// Quote characters (Document Setup → Type): opening and closing double and single quotes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Quotes {
    pub double: [char; 2],
    pub single: [char; 2],
}

impl Default for Quotes {
    fn default() -> Self {
        Self::ENGLISH
    }
}

impl Quotes {
    pub const ENGLISH: Self = Self { double: ['“', '”'], single: ['‘', '’'] };
    /// Double-quote pairs offered by Document Setup.
    pub const DOUBLE_CHOICES: &[[char; 2]] = &[['“', '”'], ['„', '“'], ['„', '”'], ['”', '”'], ['«', '»'], ['»', '«'], ['「', '」'], ['"', '"']];
    /// Single-quote pairs offered by Document Setup.
    pub const SINGLE_CHOICES: &[[char; 2]] = &[['‘', '’'], ['‚', '‘'], ['‚', '’'], ['’', '’'], ['‹', '›'], ['›', '‹'], ['『', '』'], ['\'', '\'']];

    /// Is `c` one of these opening quotes?
    fn is_open(&self, c: char) -> bool {
        c == self.double[0] || c == self.single[0]
    }

    /// A quote opens after nothing, whitespace, an opening bracket, a dash or another opening quote.
    fn opens(&self, prev: Option<char>) -> bool {
        prev.is_none_or(|p| p.is_whitespace() || matches!(p, '(' | '[' | '{' | '—' | '–') || self.is_open(p))
    }

    /// `text` with straight quotes (`"` and `'`) replaced by these quotes: opening or closing by
    /// context (an apostrophe is a closing single quote). `prev` is the character before `text`,
    /// and is updated to its last character, so text can be converted in pieces.
    pub fn apply(&self, text: &str, prev: &mut Option<char>) -> String {
        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            let r = match c {
                '"' => self.double[usize::from(!self.opens(*prev))],
                '\'' => self.single[usize::from(!self.opens(*prev))],
                c => c,
            };
            out.push(r);
            *prev = Some(r);
        }
        out
    }
}

/// Document and character languages: the name, the BCP 47 tag text is shaped in (so that fonts use
/// their language's forms, OpenType `locl`) and the quote characters each uses (choosing a
/// language in Document Setup also picks its quotes). The first is the default.
pub const LANGUAGES: &[(&str, &str, Quotes)] = &[
    ("English: USA", "en-US", Quotes::ENGLISH),
    ("English: UK", "en-GB", Quotes { double: ['“', '”'], single: ['‘', '’'] }),
    ("German", "de", Quotes { double: ['„', '“'], single: ['‚', '‘'] }),
    ("German: Swiss", "de-CH", Quotes { double: ['«', '»'], single: ['‹', '›'] }),
    ("French", "fr", Quotes { double: ['«', '»'], single: ['‹', '›'] }),
    ("Italian", "it", Quotes { double: ['«', '»'], single: ['‘', '’'] }),
    ("Spanish", "es", Quotes { double: ['«', '»'], single: ['‘', '’'] }),
    ("Catalan", "ca", Quotes { double: ['«', '»'], single: ['‘', '’'] }),
    ("Portuguese", "pt", Quotes { double: ['«', '»'], single: ['‘', '’'] }),
    ("Dutch", "nl", Quotes { double: ['“', '”'], single: ['‘', '’'] }),
    ("Danish", "da", Quotes { double: ['»', '«'], single: ['›', '‹'] }),
    ("Swedish", "sv", Quotes { double: ['”', '”'], single: ['’', '’'] }),
    ("Finnish", "fi", Quotes { double: ['”', '”'], single: ['’', '’'] }),
    ("Norwegian", "nb", Quotes { double: ['«', '»'], single: ['‘', '’'] }),
    ("Polish", "pl", Quotes { double: ['„', '”'], single: ['‚', '’'] }),
    ("Czech", "cs", Quotes { double: ['„', '“'], single: ['‚', '‘'] }),
    ("Russian", "ru", Quotes { double: ['«', '»'], single: ['„', '“'] }),
    ("Turkish", "tr", Quotes { double: ['“', '”'], single: ['‘', '’'] }),
    ("Japanese", "ja", Quotes { double: ['「', '」'], single: ['『', '』'] }),
    ("Chinese", "zh", Quotes { double: ['“', '”'], single: ['‘', '’'] }),
    ("Hindi", "hi", Quotes::ENGLISH),
    ("Marathi", "mr", Quotes::ENGLISH),
    ("Gujarati", "gu", Quotes::ENGLISH),
    ("Bengali", "bn", Quotes::ENGLISH),
    ("Punjabi", "pa", Quotes::ENGLISH),
    ("Tamil", "ta", Quotes::ENGLISH),
    ("Telugu", "te", Quotes::ENGLISH),
    ("Kannada", "kn", Quotes::ENGLISH),
    ("Malayalam", "ml", Quotes::ENGLISH),
    ("Odia", "or", Quotes::ENGLISH),
    ("Assamese", "as", Quotes::ENGLISH),
    ("Sanskrit", "sa", Quotes::ENGLISH),
    ("Urdu", "ur", Quotes::ENGLISH),
];

/// The quotes of a [`LANGUAGES`] entry (case-insensitive).
pub fn language_quotes(language: &str) -> Option<Quotes> {
    LANGUAGES.iter().find(|(l, _, _)| l.eq_ignore_ascii_case(language)).map(|(_, _, q)| *q)
}

/// The [`LANGUAGES`] entry named or tagged `language` (case-insensitive) as a character's
/// language ([`crate::CharStyle::language`]) stores it: its tag, or `None` for the default (the
/// first entry); `Err` for a language not in the list.
pub fn char_language(language: &str) -> Result<Option<&'static str>, String> {
    let i = LANGUAGES
        .iter()
        .position(|(n, t, _)| n.eq_ignore_ascii_case(language) || t.eq_ignore_ascii_case(language))
        .ok_or_else(|| format!("unknown language `{language}` (one of {})", LANGUAGES.iter().map(|(n, _, _)| *n).collect::<Vec<_>>().join(", ")))?;
    Ok(LANGUAGES.get(i).filter(|_| i > 0).map(|(_, t, _)| *t))
}

/// The name of the language a character's stored `language` tag stands for: the default's for
/// none, `None` for a tag not in [`LANGUAGES`] (from a file).
pub fn char_language_name(language: Option<&str>) -> Option<&'static str> {
    match language {
        None => LANGUAGES.first().map(|(n, _, _)| *n),
        Some(tag) => LANGUAGES.iter().find(|(_, t, _)| t.eq_ignore_ascii_case(tag)).map(|(n, _, _)| *n),
    }
}

/// The transparency flattener preset Document Setup uses unless another is chosen.
pub const DEFAULT_FLATTENER: &str = "Medium Resolution";

/// Background Contents (New Document): what shows behind the art on the artboards, and what
/// raster exports put there.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Background {
    #[default]
    Transparent,
    /// A white artboard background (not an object): raster exports are opaque white there.
    White,
}

impl Background {
    pub const ALL: [Background; 2] = [Background::Transparent, Background::White];
    pub fn id(self) -> &'static str {
        match self {
            Background::Transparent => "transparent",
            Background::White => "white",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|b| b.id().eq_ignore_ascii_case(s))
    }
}

/// Largest bleed on any side, in points (1 inch).
pub const MAX_BLEED: f64 = 72.0;

/// File → Document Setup.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DocSetup {
    /// Bleed beyond every artboard edge in points: top, bottom, left, right.
    pub bleed: [f64; 4],
    /// Show Images in Outline Mode.
    pub outline_images: bool,
    /// Highlight text whose font is missing (drawn in a substitute font).
    pub highlight_substituted_fonts: bool,
    /// Highlight glyphs the font lacks (drawn from a fallback font).
    pub highlight_substituted_glyphs: bool,
    /// Transparency grid cell size.
    pub grid_size: GridSize,
    /// Transparency grid colours; the first is also the simulated paper colour.
    pub grid_colors: [Color; 2],
    /// Simulate Colored Paper: artboards show the paper colour and art is multiplied onto it.
    pub simulate_paper: bool,
    /// Transparency flattener preset for output that can't keep transparency (None =
    /// [`DEFAULT_FLATTENER`]).
    pub flattener_preset: Option<String>,
    /// Discard White Overprint in Output.
    pub discard_white_overprint: bool,
    /// Document language (one of [`LANGUAGES`]).
    pub language: String,
    pub quotes: Quotes,
    /// Use Typographer's Quotes: typed straight quotes become [`DocSetup::quotes`].
    pub typographers_quotes: bool,
    pub superscript: ScriptMetrics,
    pub subscript: ScriptMetrics,
    /// Synthesized small capitals, percent of the font size.
    pub small_caps_size: f64,
    pub export_text: ExportText,
    /// Background Contents chosen in New Document.
    pub background: Background,
}

impl Default for DocSetup {
    fn default() -> Self {
        Self {
            bleed: [0.0; 4],
            outline_images: false,
            highlight_substituted_fonts: true,
            highlight_substituted_glyphs: false,
            grid_size: GridSize::Medium,
            grid_colors: GRID_COLOR_PRESETS[0].1,
            simulate_paper: false,
            flattener_preset: None,
            discard_white_overprint: true,
            language: LANGUAGES[0].0.to_string(),
            quotes: Quotes::ENGLISH,
            typographers_quotes: true,
            superscript: ScriptMetrics::DEFAULT,
            subscript: ScriptMetrics::DEFAULT,
            small_caps_size: 70.0,
            export_text: ExportText::Editable,
            background: Background::Transparent,
        }
    }
}

impl DocSetup {
    /// `artboard` grown by the bleed.
    pub fn bleed_rect(&self, artboard: Rect) -> Rect {
        crate::marks::outset(artboard, self.bleed)
    }
    pub fn has_bleed(&self) -> bool {
        self.bleed.iter().any(|b| *b > 0.0)
    }
    /// The simulated paper colour.
    pub fn paper(&self) -> Color {
        self.grid_colors[0]
    }
    /// The name of the grid colour preset in use, or "Custom".
    pub fn grid_colors_name(&self) -> &'static str {
        GRID_COLOR_PRESETS.iter().find(|(_, c)| *c == self.grid_colors).map_or("Custom", |(n, _)| n)
    }
    /// The flattener preset in use.
    pub fn flattener(&self) -> &str {
        self.flattener_preset.as_deref().unwrap_or(DEFAULT_FLATTENER)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quotes_open_and_close_by_context() {
        let mut prev = None;
        assert_eq!(Quotes::ENGLISH.apply("\"Hi,\" it's 'x'", &mut prev), "“Hi,” it’s ‘x’");
        let german = language_quotes("german").unwrap();
        let mut prev = Some('(');
        assert_eq!(german.apply("\"a\"", &mut prev), "„a“");
        assert_eq!(prev, Some('“'));
        // Converting in pieces keeps the context.
        let mut prev = None;
        let a = Quotes::ENGLISH.apply("\"", &mut prev);
        let b = Quotes::ENGLISH.apply("\"", &mut prev);
        assert_eq!((a.as_str(), b.as_str()), ("“", "“"));
    }

    #[test]
    fn bleed_grows_the_artboard() {
        let s = DocSetup { bleed: [9.0, 9.0, 4.0, 2.0], ..Default::default() };
        assert_eq!(s.bleed_rect(Rect::new(0.0, 0.0, 100.0, 50.0)), Rect::new(-4.0, -9.0, 102.0, 59.0));
        assert!(s.has_bleed() && !DocSetup::default().has_bleed());
    }

    #[test]
    fn presets_and_defaults() {
        let d = DocSetup::default();
        assert_eq!(d.grid_colors_name(), "Light");
        assert_eq!(d.flattener(), "Medium Resolution");
        assert_eq!(Background::parse("WHITE"), Some(Background::White));
        assert_eq!(GridSize::parse("LARGE"), Some(GridSize::Large));
        assert_eq!(ExportText::parse("appearance"), Some(ExportText::Appearance));
        // A partial setup fills in the rest.
        let s: DocSetup = serde_json::from_str(r#"{"bleed":[1,2,3,4]}"#).unwrap();
        assert_eq!(s, DocSetup { bleed: [1.0, 2.0, 3.0, 4.0], ..DocSetup::default() });
    }

    #[test]
    fn indic_languages_canonicalize_and_old_character_styles_keep_the_default() {
        for (name, tag) in [("Telugu", "te"), ("Hindi", "hi"), ("Assamese", "as"), ("Sanskrit", "sa")] {
            assert_eq!(char_language(name), Ok(Some(tag)));
            assert_eq!(char_language(&tag.to_uppercase()), Ok(Some(tag)));
            assert_eq!(char_language_name(Some(tag)), Some(name));
        }
        assert_eq!(char_language("en-US"), Ok(None));
        assert!(char_language("made-up").is_err());
        let old = serde_json::to_value(crate::CharStyle::default()).unwrap();
        assert!(old.get("language").is_none());
        assert_eq!(serde_json::from_value::<crate::CharStyle>(old).unwrap().language, None);
    }
}
