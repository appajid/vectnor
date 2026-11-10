//! The text of an Illustrator editing copy.
//!
//! The editing copy of a file keeps the characters of its text objects, with their fonts, sizes,
//! colours and where they sit, in one text document of its own: between `%AI11_BeginTextDocument`
//! and `%AI11_EndTextDocument`, ASCII85 of nested dictionaries with numbers for keys. A text object
//! in a layer is only a stub naming its story in it (`/StoryIndex`).
//!
//! The format isn't documented. What is read here was worked out from the files of the project's
//! users, used locally and never committed, by comparing it with the type their pages draw: the
//! meaning of each key below was checked against the page's own type in more than a thousand text
//! objects, and it is only used for the keys that matched. A story the reader doesn't recognise (area
//! type, type on a path, anything with a frame that has more than a point and a matrix) is left
//! alone: it has no text, and the importer says so.
//!
//! Where type sits: a story lays its lines out round a point `F` (the anchor: the start of a line of
//! left-aligned type, the middle of a centred one, the end of a right-aligned one), each line with an
//! offset from it. The frame of the story gives a matrix that carries that layout onto the canvas of
//! the app, which is 16383 points a side; the canvas's centre, 8191.5, is the centre of the file's
//! `%AI3_TemplateBox`. See [`Story::place`].

use std::collections::BTreeMap;

use kurbo::{BezPath, CubicBez, ParamCurve, ParamCurveArclen};
use vectnor_color::{Color, Paint};
use vectnor_doc::{CharPosition, CharStyle, FirstBaseline, Justify, Node, NodeId, NodeKind, ParaStyle, ScriptMetrics, TextKind, TextObject, TextRun};
use vectnor_geom::{Affine, PathData, Point, Rect};

/// Longest story read (bytes).
const MAX_STORY: usize = 1 << 20;
/// Biggest text document read (bytes, decoded).
const MAX_BYTES: usize = 64 << 20;
/// Deepest nesting read.
const MAX_DEPTH: usize = 64;
/// Most values read.
const MAX_VALUES: usize = 4_000_000;
/// The centre of the app's canvas, in the units of its text document.
const CANVAS_CENTRE: f64 = 8191.5;

/// A value of the text document.
#[derive(Debug, Clone, PartialEq)]
pub(super) enum Val {
    Num(f64),
    Str(String),
    Name(String),
    Bool(bool),
    List(Vec<Val>),
    Dict(BTreeMap<String, Val>),
}

impl Val {
    fn get(&self, key: &str) -> Option<&Val> {
        match self {
            Val::Dict(d) => d.get(key),
            _ => None,
        }
    }

    fn num(&self) -> Option<f64> {
        match self {
            Val::Num(n) if n.is_finite() => Some(*n),
            _ => None,
        }
    }

    fn str(&self) -> Option<&str> {
        match self {
            Val::Str(s) => Some(s),
            _ => None,
        }
    }

    fn list(&self) -> &[Val] {
        match self {
            Val::List(l) => l,
            _ => &[],
        }
    }

    fn nums(&self) -> Option<Vec<f64>> {
        self.list().iter().map(Val::num).collect()
    }

    /// The `name` entry of a dictionary that is a typed node (`/99 /name`).
    fn is_node(&self, name: &str) -> bool {
        matches!(self.get("99"), Some(Val::Name(n)) if n == name)
    }
}

struct Reader<'a> {
    b: &'a [u8],
    at: usize,
    budget: usize,
}

impl Reader<'_> {
    fn skip_space(&mut self) {
        while self.b.get(self.at).is_some_and(u8::is_ascii_whitespace) {
            self.at += 1;
        }
    }

    fn value(&mut self, depth: usize) -> Option<Val> {
        self.skip_space();
        self.budget = self.budget.checked_sub(1)?;
        let c = *self.b.get(self.at)?;
        match c {
            b'<' if self.b.get(self.at + 1) == Some(&b'<') => {
                if depth > MAX_DEPTH {
                    return None;
                }
                self.at += 2;
                let mut d = BTreeMap::new();
                loop {
                    self.skip_space();
                    if self.b.get(self.at..self.at + 2) == Some(b">>") {
                        self.at += 2;
                        return Some(Val::Dict(d));
                    }
                    let key = self.key()?;
                    let v = self.value(depth + 1)?;
                    d.insert(key, v);
                }
            }
            b'[' => {
                if depth > MAX_DEPTH {
                    return None;
                }
                self.at += 1;
                let mut l = vec![];
                loop {
                    self.skip_space();
                    if self.b.get(self.at) == Some(&b']') {
                        self.at += 1;
                        return Some(Val::List(l));
                    }
                    l.push(self.value(depth + 1)?);
                }
            }
            b'(' => self.string(),
            b'/' => Some(Val::Name(self.key()?)),
            _ => {
                let start = self.at;
                while self.b.get(self.at).is_some_and(|c| !c.is_ascii_whitespace() && !matches!(c, b'<' | b'>' | b'[' | b']' | b'(' | b')' | b'/')) {
                    self.at += 1;
                }
                let word = std::str::from_utf8(self.b.get(start..self.at)?).ok()?;
                match word {
                    "true" => Some(Val::Bool(true)),
                    "false" => Some(Val::Bool(false)),
                    "" => None,
                    w => Some(Val::Num(w.parse().ok()?)),
                }
            }
        }
    }

    /// A `/name`, without its slash.
    fn key(&mut self) -> Option<String> {
        self.skip_space();
        if self.b.get(self.at) != Some(&b'/') {
            return None;
        }
        self.at += 1;
        let start = self.at;
        while self.b.get(self.at).is_some_and(|c| !c.is_ascii_whitespace() && !matches!(c, b'<' | b'>' | b'[' | b']' | b'(' | b')' | b'/')) {
            self.at += 1;
        }
        Some(String::from_utf8_lossy(self.b.get(start..self.at)?).into_owned())
    }

    /// A string: up to the first `)` that isn't escaped; UTF-16 when it starts with a byte order mark.
    fn string(&mut self) -> Option<Val> {
        self.at += 1;
        let mut bytes = vec![];
        loop {
            let c = *self.b.get(self.at)?;
            self.at += 1;
            match c {
                b')' => break,
                b'\\' => {
                    let e = *self.b.get(self.at)?;
                    self.at += 1;
                    bytes.push(match e {
                        b'n' => b'\n',
                        b'r' => b'\r',
                        b't' => b'\t',
                        b'b' => 8,
                        b'f' => 12,
                        b'0'..=b'7' => {
                            let mut v = u32::from(e - b'0');
                            for _ in 0..2 {
                                match self.b.get(self.at) {
                                    Some(d @ b'0'..=b'7') => {
                                        v = v * 8 + u32::from(d - b'0');
                                        self.at += 1;
                                    }
                                    _ => break,
                                }
                            }
                            (v & 0xff) as u8
                        }
                        other => other,
                    });
                }
                c => bytes.push(c),
            }
            if bytes.len() > MAX_BYTES {
                return None;
            }
        }
        Some(Val::Str(match bytes.strip_prefix(&[0xfe, 0xff]) {
            Some(rest) => {
                let units: Vec<u16> = rest.as_chunks::<2>().0.iter().map(|p| u16::from_be_bytes(*p)).collect();
                String::from_utf16_lossy(&units)
            }
            None => bytes.iter().map(|b| char::from(*b)).collect(),
        }))
    }
}

/// The values of a text document: its top-level `/key value` pairs.
fn read(bytes: &[u8]) -> Option<Val> {
    let mut r = Reader { b: bytes, at: 0, budget: MAX_VALUES };
    let mut d = BTreeMap::new();
    loop {
        r.skip_space();
        if r.at >= bytes.len() {
            return Some(Val::Dict(d));
        }
        let key = r.key()?;
        let v = r.value(0)?;
        d.insert(key, v);
    }
}

/// Most character styles read of a text document, and most styles a run's style is followed
/// through to the ones they are based on.
const MAX_SHEETS: usize = 4096;
const MAX_SHEETS_BASED_ON: usize = 16;

/// Most frames read of one story, and most path segments of a frame.
const MAX_FRAMES: usize = 4096;
const MAX_SEGMENTS: usize = 1 << 16;

/// Where a frame puts its story's text.
#[derive(Debug, Clone)]
enum FrameKind {
    /// Point type: the matrix carries the layout (its anchor at the origin of its lines) onto the
    /// canvas.
    Point([f64; 6]),
    /// Area type flowed inside a path: its cubic segments on the canvas, four points each.
    Area { segments: Vec<[f64; 8]>, offset: (f64, f64) },
    /// Type on a path, from `start` to `end` (segment index and the place in that segment).
    OnPath { segments: Vec<[f64; 8]>, start: f64, end: f64 },
}

/// A paragraph's attributes, as the text document keeps them.
#[derive(Clone, Debug, Default, PartialEq)]
struct Para {
    justify: u8,
    first_indent: f64,
    left_indent: f64,
    right_indent: f64,
    space_before: f64,
    space_after: f64,
    hyphenate: bool,
}

/// The text of one story, in the units of the text document.
#[derive(Debug)]
pub(super) struct Story {
    /// The characters, with `\r` ending paragraphs and `\x03` ending lines.
    text: String,
    /// The anchor of the first frame's layout.
    anchor: (f64, f64),
    /// Where each of the first frame's lines starts, from the anchor.
    lines: Vec<(f64, f64)>,
    /// First printed baseline of each frame, in its untranslated layout space.
    frame_starts: Vec<Option<(f64, f64)>>,
    /// The frames the story flows through, in order.
    frames: Vec<FrameKind>,
    /// `(characters, paragraph)` in order.
    paras: Vec<(usize, Para)>,
    /// `(characters, style)` in order.
    runs: Vec<(usize, Style)>,
}

#[derive(Clone, Debug)]
struct Style {
    font: Option<String>,
    size: f64,
    /// `None`: auto.
    leading: Option<f64>,
    tracking: f64,
    baseline_shift: f64,
    position: CharPosition,
    h_scale: f64,
    v_scale: f64,
    fill: Option<Color>,
    stroke: Option<Color>,
    stroke_width: f64,
    /// Key 12, the case: 1 Small Caps, 2 All Caps.
    caps: u8,
}

/// The text document of an editing copy.
pub(super) struct Texts {
    stories: Vec<Val>,
    fonts: Vec<String>,
    /// The character and paragraph styles a story starts from.
    defaults: Val,
    para_defaults: Val,
    /// The document's character styles (`[Normal Character Style]` first), each a name, the
    /// character style it is based on (key 5, an index into these) and its attributes (key 6).
    char_sheets: Vec<Val>,
    para_sheets: Vec<Val>,
    frames: Vec<Val>,
}

impl Texts {
    /// The text document in the editing copy `data`, if it has one that can be read.
    pub(super) fn read(data: &[u8]) -> Option<Texts> {
        fn find(h: &[u8], n: &[u8], from: usize) -> Option<usize> {
            h.get(from..)?.windows(n.len()).position(|w| w == n).map(|p| p + from)
        }
        let start = find(data, b"%AI11_BeginTextDocument", 0)?;
        let from = find(data, b"/ASCII85Decode", start)? + b"/ASCII85Decode".len();
        let end = find(data, b"~>", from)? + 2;
        // Each line after the first starts with a `%`.
        // ASCII85 makes at most four bytes of a character (`z`): more than this would be over `MAX_BYTES`.
        if end - from > MAX_BYTES / 4 {
            return None;
        }
        let packed: String = String::from_utf8_lossy(data.get(from..end)?)
            .split(['\r', '\n'])
            .enumerate()
            .map(|(i, l)| if i > 0 { l.strip_prefix('%').unwrap_or(l) } else { l })
            .collect::<String>()
            .trim_start_matches([' ', ','])
            .to_string();
        let bytes = crate::ps::ascii85_decode(&packed)?;
        if bytes.len() > MAX_BYTES {
            return None;
        }
        let doc = read(&bytes)?;
        let fonts = doc
            .get("0")?
            .get("1")?
            .get("0")?
            .list()
            .iter()
            .map(|f| f.get("0").and_then(|f| f.get("0")).and_then(|f| f.get("0")).and_then(Val::str).unwrap_or_default().to_string())
            .collect();
        let main = doc.get("1")?;
        let sheets = |key: &str| -> Vec<Val> {
            let list = doc.get("0").and_then(|d| d.get(key)).and_then(|s| s.get("0")).map(Val::list).unwrap_or_default();
            list.iter().take(MAX_SHEETS).map(|s| s.get("0").cloned().unwrap_or(Val::Dict(BTreeMap::new()))).collect()
        };
        Some(Texts {
            stories: main.get("1")?.list().to_vec(),
            fonts,
            defaults: main.get("2")?.clone(),
            para_defaults: main.get("3").cloned().unwrap_or(Val::Dict(BTreeMap::new())),
            char_sheets: sheets("5"),
            para_sheets: sheets("6"),
            frames: doc.get("0")?.get("8")?.get("0")?.list().to_vec(),
        })
    }

    /// Frame `i` of the document, if it is of a kind this reads.
    fn frame(&self, i: usize) -> Option<FrameKind> {
        let f = self.frames.get(i)?.get("0")?;
        let carries = f.get("2")?;
        let segments = || -> Option<Vec<[f64; 8]>> {
            let n = f.get("1")?.get("0")?.nums()?;
            let (chunks, rest) = n.as_chunks::<8>();
            (rest.is_empty() && !chunks.is_empty() && chunks.len() <= MAX_SEGMENTS).then(|| chunks.to_vec())
        };
        match carries.get("0").and_then(Val::num).unwrap_or(0.0) as i64 {
            0 => Some(FrameKind::Point(match carries.get("2") {
                Some(m) => m.nums()?.try_into().ok()?,
                None => Affine::IDENTITY.as_coeffs(),
            })),
            1 => {
                let mut segments = segments()?;
                let mut offset = (0.0, 0.0);
                if let Some(m) = carries.get("2") {
                    let [a, b, c, d, x, y]: [f64; 6] = m.nums()?.try_into().ok()?;
                    // Translated area frames retain their path in the layout's old space.
                    // Other affine changes need corresponding type-style transforms too.
                    if [a - 1.0, b, c, d - 1.0].iter().any(|v| v.abs() > 1e-9) {
                        return None;
                    }
                    offset = (x, y);
                    for segment in &mut segments {
                        for [px, py] in segment.as_chunks_mut::<2>().0 {
                            *px += x;
                            *py += y;
                        }
                    }
                }
                if segments.iter().flatten().any(|v| !v.is_finite() || v.abs() > 1e7) {
                    return None;
                }
                Some(FrameKind::Area { segments, offset })
            }
            2 => {
                let range = carries.get("6").and_then(Val::nums).unwrap_or_default();
                let segments = segments()?;
                let n = segments.len() as f64;
                let finite = |v: Option<&f64>| v.copied().filter(|v| v.is_finite());
                let (start, end) = if closed(&segments) {
                    // Round a closed path the end is written unwrapped (the start plus the run, so
                    // past the last segment), or wrapped the other way: both brackets are places on
                    // the loop.
                    let start = finite(range.first()).unwrap_or(0.0).rem_euclid(n);
                    (start, finite(range.get(1)).map_or(start, |e| e.rem_euclid(n)))
                } else {
                    let start = finite(range.first()).unwrap_or(0.0).clamp(0.0, n);
                    (start, finite(range.get(1)).unwrap_or(n).clamp(start, n))
                };
                Some(FrameKind::OnPath { segments, start, end })
            }
            _ => None,
        }
    }

    /// Story `i`, if its frames are of kinds this reads.
    pub(super) fn story(&self, i: usize) -> Option<Story> {
        let s = self.stories.get(i)?;
        let text = s.get("0")?.get("0")?.str().filter(|t| t.len() <= MAX_STORY)?.to_string();
        let view = s.get("1")?;
        let frame_ids: Vec<usize> =
            view.get("0")?.list().iter().take(MAX_FRAMES).map(|f| f.get("0").and_then(Val::num).map(|n| n as usize)).collect::<Option<_>>()?;
        let frames: Vec<FrameKind> = frame_ids.iter().map(|i| self.frame(*i)).collect::<Option<_>>()?;
        if frames.is_empty() {
            return None;
        }
        // The first frame's layout, when the file keeps one: where its lines start.
        let (mut anchor, mut lines) = ((0.0, 0.0), vec![]);
        if let Some(f) = view.get("2").and_then(|l| find_node(l, "F")) {
            if let Some(a) = f.get("0").and_then(|a| a.get("0")).and_then(Val::nums)
                && let [x, y, ..] = a.as_slice()
            {
                anchor = (*x, *y);
            }
            let mut found = vec![];
            find_nodes(f, "L", &mut found);
            for l in found {
                let off = l.get("0").and_then(|o| o.get("0")).and_then(Val::nums).unwrap_or_else(|| vec![0.0, 0.0]);
                let Some(seg) = l.get("6").map(Val::list).and_then(|c| c.iter().find(|c| c.is_node("S"))) else { continue };
                let so = seg.get("0").and_then(|o| o.get("0")).and_then(Val::nums).unwrap_or_else(|| vec![0.0, 0.0]);
                if let (Some(ox), Some(oy), Some(sx)) = (off.first(), off.get(1), so.first()) {
                    lines.push((ox + sx, *oy));
                }
            }
        }
        let mut layout_frames = vec![];
        if let Some(layout) = view.get("2") {
            find_nodes(layout, "F", &mut layout_frames);
        }
        // Bound the identity lookup even in a hostile layout containing many frame nodes.
        if frame_ids.len().saturating_mul(layout_frames.len()) > 1_000_000 {
            return None;
        }
        let frame_starts = frame_ids
            .iter()
            .map(|id| {
                let mut matching = layout_frames.iter().filter(|f| f.get("10").and_then(Val::num) == Some(*id as f64));
                let f = match matching.next() {
                    Some(f) if matching.next().is_none() => *f,
                    None if frame_ids.len() == 1 && layout_frames.len() == 1 && layout_frames.first()?.get("10").is_none() => {
                        *layout_frames.first()?
                    }
                    _ => return None,
                };
                let anchor = f.get("0")?.get("0")?.nums()?;
                let l = find_node(f, "L")?;
                let off = l.get("0").and_then(|o| o.get("0")).and_then(Val::nums).unwrap_or_else(|| vec![0.0, 0.0]);
                let segment = l.get("6")?.list().iter().find(|c| c.is_node("S"))?;
                let so = segment.get("0").and_then(|o| o.get("0")).and_then(Val::nums).unwrap_or_else(|| vec![0.0, 0.0]);
                Some((anchor.first()? + off.first()? + so.first()?, anchor.get(1)? + off.get(1)?))
            })
            .collect();
        let mut paras = vec![];
        for run in s.get("0")?.get("5").map(|p| p.get("0").map(Val::list).unwrap_or_default()).unwrap_or_default() {
            let n = run.get("1").and_then(Val::num)? as usize;
            paras.push((n, self.para(run.get("0").and_then(|r| r.get("0")))));
        }
        if paras.is_empty() {
            paras.push((text.chars().count(), self.para(None)));
        }
        let mut runs = vec![];
        for run in s.get("0")?.get("6").map(|p| p.get("0").map(Val::list).unwrap_or_default()).unwrap_or_default() {
            let n = run.get("1").and_then(Val::num)? as usize;
            runs.push((n, self.style(run.get("0").and_then(|r| r.get("0")))));
        }
        if runs.is_empty() {
            runs.push((text.chars().count(), self.style(None)));
        }
        Some(Story { text, anchor, lines, frame_starts, frames, paras, runs })
    }

    /// Run overrides, based-on paragraph sheets, Normal Paragraph Style, then document defaults.
    fn para(&self, run: Option<&Val>) -> Para {
        let mut chain = vec![];
        let mut at = run;
        for _ in 0..=MAX_SHEETS_BASED_ON {
            let Some(style) = at else { break };
            chain.extend(style.get("5"));
            at = style.get("6").and_then(Val::num).filter(|i| i.is_finite() && *i >= 0.0).and_then(|i| self.para_sheets.get(i as usize));
        }
        chain.extend(self.para_sheets.first().and_then(|s| s.get("5")));
        let pick = |k: &str| chain.iter().find_map(|s| s.get(k)).or_else(|| self.para_defaults.get(k));
        let num = |k: &str| pick(k).and_then(Val::num).filter(|v| v.abs() < 1e5).unwrap_or(0.0);
        Para {
            justify: pick("0").and_then(Val::num).map_or(0, |v| v.clamp(0.0, 6.0) as u8),
            first_indent: num("1"),
            left_indent: num("2"),
            right_indent: num("3"),
            space_before: num("4"),
            space_after: num("5"),
            hyphenate: !matches!(pick("9"), Some(Val::Bool(false))),
        }
    }

    /// A run's character style (`run`: its attributes, key 6, and the character style it is based
    /// on, key 5): its own keys, else those of its character style and the ones that is based on,
    /// else the document's.
    fn style(&self, run: Option<&Val>) -> Style {
        // The run's attributes, then its character style's, then the styles each is based on.
        let mut chain = vec![];
        let mut at = run;
        for _ in 0..=MAX_SHEETS_BASED_ON {
            let Some(style) = at else { break };
            chain.extend(style.get("6"));
            at = style.get("5").and_then(Val::num).filter(|i| i.is_finite() && *i >= 0.0).and_then(|i| self.char_sheets.get(i as usize));
        }
        chain.extend(self.char_sheets.first().and_then(|s| s.get("6")));
        let pick = |k: &str| chain.iter().find_map(|s| s.get(k)).or_else(|| self.defaults.get(k));
        let num = |k: &str| pick(k).and_then(Val::num);
        let on = |k: &str| !matches!(pick(k), Some(Val::Bool(false)));
        let font = num("0").and_then(|i| self.fonts.get(i as usize)).filter(|f| !f.is_empty()).cloned();
        let size = num("1").filter(|v| *v > 0.0 && *v < 1e5).unwrap_or(12.0);
        Style {
            font,
            size,
            leading: if on("4") { None } else { num("5").filter(|v| *v > 0.0 && *v < 1e5) },
            tracking: num("8").filter(|v| v.abs() < 1e5).unwrap_or(0.0),
            baseline_shift: num("9").filter(|v| v.abs() < 1e5).unwrap_or(0.0),
            position: match num("13") {
                Some(1.0) => CharPosition::Superscript(ScriptMetrics::DEFAULT),
                Some(2.0) => CharPosition::Subscript(ScriptMetrics::DEFAULT),
                _ => CharPosition::Normal,
            },
            h_scale: num("6").filter(|v| *v > 0.0 && *v < 100.0).unwrap_or(1.0),
            v_scale: num("7").filter(|v| *v > 0.0 && *v < 100.0).unwrap_or(1.0),
            fill: if on("56") { Some(pick("53").and_then(paint).unwrap_or(Color::BLACK)) } else { None },
            stroke: if matches!(pick("57"), Some(Val::Bool(true))) { pick("54").and_then(paint) } else { None },
            stroke_width: num("63").filter(|v| *v >= 0.0 && *v < 1e4).unwrap_or(1.0),
            caps: num("12").filter(|v| *v == 1.0 || *v == 2.0).map_or(0, |v| v as u8),
        }
    }
}

/// The colour of a paint of the text document: kind 0 grey (`[alpha, level]`, 1 is white), 1 RGB
/// (`[alpha, r, g, b]`), 2 CMYK (`[alpha, c, m, y, k]`).
fn paint(p: &Val) -> Option<Color> {
    let p = p.get("0")?;
    let v = p.get("1")?.nums()?;
    let unit = |i: usize| v.get(i).map(|c| c.clamp(0.0, 1.0) as f32);
    match p.get("0")?.num()? as i64 {
        0 => Some(Color::gray(1.0 - unit(1)?)),
        1 => Some(Color::rgb(unit(1)?, unit(2)?, unit(3)?)),
        2 => Some(Color::cmyk(unit(1)?, unit(2)?, unit(3)?, unit(4)?)),
        _ => None,
    }
}

fn find_node<'a>(v: &'a Val, name: &str) -> Option<&'a Val> {
    let mut out = vec![];
    find_nodes(v, name, &mut out);
    out.into_iter().next()
}

/// The typed nodes called `name` in `v`, in order, outermost first.
fn find_nodes<'a>(v: &'a Val, name: &str, out: &mut Vec<&'a Val>) {
    match v {
        Val::Dict(d) => {
            if v.is_node(name) {
                out.push(v);
            }
            for x in d.values() {
                find_nodes(x, name, out);
            }
        }
        Val::List(l) => {
            for x in l {
                find_nodes(x, name, out);
            }
        }
        _ => {}
    }
}

/// A point of the canvas (y down) in the units of the art (y up), given the centre `template` of
/// the file's template box.
fn to_art((x, y): (f64, f64), template: (f64, f64)) -> (f64, f64) {
    (x - CANVAS_CENTRE + template.0, CANVAS_CENTRE + template.1 - y)
}

/// Segments of the canvas as a path of the document.
fn path_of(segments: &[[f64; 8]], template: (f64, f64), to_doc: Affine) -> PathData {
    let at = |x: f64, y: f64| {
        let (x, y) = to_art((x, y), template);
        to_doc * Point::new(x, y)
    };
    let mut bp = BezPath::new();
    for (i, [x0, y0, x1, y1, x2, y2, x3, y3]) in segments.iter().enumerate() {
        if i == 0 {
            bp.move_to(at(*x0, *y0));
        }
        bp.curve_to(at(*x1, *y1), at(*x2, *y2), at(*x3, *y3));
    }
    let closed = closed(segments);
    if closed {
        bp.close_path();
    }
    PathData::from_bezpath(&bp)
}

/// Do `segments` end where they start (a closed path)?
fn closed(segments: &[[f64; 8]]) -> bool {
    matches!((segments.first(), segments.last()), (Some(f), Some(l)) if (f[0] - l[6]).abs() < 1e-3 && (f[1] - l[7]).abs() < 1e-3)
}

/// Where on `segments` the place `t` (segment index and the place in it) is, as a fraction of their
/// length.
fn length_fraction(segments: &[[f64; 8]], t: f64) -> f64 {
    let cubic = |s: &[f64; 8]| CubicBez::new((s[0], s[1]), (s[2], s[3]), (s[4], s[5]), (s[6], s[7]));
    let lengths: Vec<f64> = segments.iter().map(|s| cubic(s).arclen(0.01)).collect();
    let total: f64 = lengths.iter().sum();
    if !total.is_finite() || total <= 1e-9 {
        return 0.0;
    }
    let whole = t.floor().max(0.0) as usize;
    let before: f64 = lengths.iter().take(whole).sum();
    let within = match segments.get(whole) {
        Some(s) => cubic(s).subsegment(0.0..t.fract().clamp(0.0, 1.0)).arclen(0.01),
        None => 0.0,
    };
    ((before + within) / total).clamp(0.0, 1.0)
}

impl Story {
    /// How many bytes of text the story has.
    pub(super) fn len(&self) -> usize {
        self.text.len()
    }

    /// The point frame's matrix (the first frame's, when it is one), else the identity.
    fn matrix(&self) -> [f64; 6] {
        match self.frames.first() {
            Some(FrameKind::Point(m)) => *m,
            _ => [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        }
    }

    /// The anchor of the story on the file's canvas: `(x, y)` in the units of the art, y up, given
    /// the centre `(tx, ty)` of the file's template box.
    pub(super) fn place(&self, template: (f64, f64)) -> (f64, f64) {
        let [a, b, c, d, e, f] = self.matrix();
        to_art((a * self.anchor.0 + c * self.anchor.1 + e, b * self.anchor.0 + d * self.anchor.1 + f), template)
    }

    /// Each line's characters and where it starts (in the units of the art, y up): where the page
    /// draws the line when the type is shown.
    pub(super) fn line_starts(&self, template: (f64, f64)) -> Vec<(String, (f64, f64))> {
        let [a, b, c, d, e, f] = self.matrix();
        let text: Vec<&str> = self.text.split(['\r', '\x03']).collect();
        self.lines
            .iter()
            .enumerate()
            .map(|(i, (lx, ly))| {
                let (x, y) = (self.anchor.0 + lx, self.anchor.1 + ly);
                (text.get(i).copied().unwrap_or_default().to_string(), to_art((a * x + c * y + e, b * x + d * y + f), template))
            })
            .collect()
    }

    /// Roughly where the story's type lies on the document: its frame's box, or round its lines
    /// for point type (what the page draws of it lies there).
    pub(super) fn region(&self, template: (f64, f64), to_doc: Affine) -> Option<Rect> {
        let size = self.runs.iter().map(|(_, s)| s.size).fold(0.0, f64::max);
        let bounds = |segments: &[[f64; 8]]| path_of(segments, template, to_doc).bounds();
        let r = match self.frames.first()? {
            FrameKind::Area { segments, .. } => bounds(segments)?,
            FrameKind::OnPath { segments, .. } => bounds(segments)?.inflate(size * 1.5, size * 1.5),
            FrameKind::Point(_) => {
                let longest = self.text.split(['\r', '\x03']).map(|l| l.chars().count()).max().unwrap_or(0) as f64;
                let reach = longest * size + size;
                let points: Vec<Point> = self.line_starts(template).into_iter().map(|(_, (x, y))| to_doc * Point::new(x, y)).collect();
                let first = points.first().copied().unwrap_or_else(|| {
                    let (x, y) = self.place(template);
                    to_doc * Point::new(x, y)
                });
                points.iter().fold(Rect::from_points(first, first), |r, p| r.union_pt(*p)).inflate(reach, size * 1.5)
            }
        };
        r.is_finite().then_some(r)
    }

    /// The story as the text object of its frame `frame` (`to_doc` takes the art's units onto the
    /// document), or `None` for type that is mirrored, has no size, or a frame past the first of
    /// threaded type (the first frame holds the whole story).
    pub(super) fn node(&self, id: NodeId, frame: usize, template: (f64, f64), to_doc: Affine) -> Option<Node> {
        let kind = self.frames.get(frame)?;
        let threaded = frame > 0;
        let (scale, xf, kind) = match kind {
            FrameKind::Point(m) => {
                let [a, b, c, d, ..] = *m;
                let scale = (a * d - b * c).abs().sqrt();
                if !(scale.is_finite() && scale > 1e-6 && a * d - b * c > 0.0) || threaded {
                    return None;
                }
                let (x, y) = self.place(template);
                let at = to_doc * Point::new(x, y);
                (scale, Affine::new([a / scale, b / scale, c / scale, d / scale, at.x, at.y]), TextKind::Point)
            }
            FrameKind::Area { segments, .. } => (1.0, Affine::IDENTITY, TextKind::Area { frame: path_of(segments, template, to_doc) }),
            FrameKind::OnPath { segments, start, end } => {
                let n = segments.len() as f64;
                // A closed path keeps its end bracket wherever it is (the same place as the start: a
                // full turn, see `TextKind::path_span`); an open one's end at its end is none.
                let end = (closed(segments) || *end < n - 0.02).then(|| length_fraction(segments, *end));
                (1.0, Affine::IDENTITY, TextKind::OnPath { path: path_of(segments, template, to_doc), start: length_fraction(segments, *start), end })
            }
        };
        let text: String = if threaded {
            String::new()
        } else {
            self.text.trim_end_matches('\r').chars().map(|c| if matches!(c, '\r' | '\x03') { '\n' } else { c }).collect()
        };
        if text.trim().is_empty() && !threaded {
            return None;
        }
        // Area layout already applies paragraph spacing. Its printed baseline gaps include
        // that spacing and wrapping, so treating a gap as character leading counts it twice.
        let leading = if matches!(self.frames.get(frame), Some(FrameKind::Point(_))) {
            self.lines.windows(2).map(|w| (w[1].1 - w[0].1) * scale).find(|l| l.is_finite() && *l > 0.0)
        } else {
            None
        };
        let chars: Vec<char> = text.chars().collect();
        let mut runs = vec![];
        let mut from = 0usize;
        for (n, st) in &self.runs {
            let to = from.saturating_add(*n).min(chars.len());
            let piece: String = chars.get(from..to).unwrap_or_default().iter().collect();
            from = to;
            if piece.is_empty() {
                continue;
            }
            runs.push(TextRun::new(piece, char_style(st, scale, leading)));
        }
        if from < chars.len() {
            let last = self.runs.last().map(|r| &r.1)?;
            runs.push(TextRun::new(chars.get(from..).unwrap_or_default().iter().collect::<String>(), char_style(last, scale, leading)));
        }
        let first = self.runs.first().map(|r| char_style(&r.1, scale, leading))?;
        let mut t = TextObject::point(Point::ZERO, "", first);
        if !runs.is_empty() {
            t.runs = runs;
        }
        let paras: Vec<ParaStyle> = self.paragraph_styles(&text, scale);
        t.para = paras.first().cloned().unwrap_or_default();
        if paras.len() > 1 && paras.iter().any(|p| *p != t.para) {
            t.paras = paras;
        }
        t.kind = kind;
        if let (TextKind::Area { frame: path }, FrameKind::Area { offset, .. }) = (&t.kind, self.frames.get(frame)?)
            && let Some(Some((x, y))) = self.frame_starts.get(frame)
            && let Some(bounds) = path.bounds()
        {
            let (x, y) = to_art((x + offset.0, y + offset.1), template);
            let baseline = (to_doc * Point::new(x, y)).y - bounds.y0;
            if baseline.is_finite() && (0.0..=bounds.height()).contains(&baseline) {
                t.area.first_baseline = FirstBaseline::Fixed;
                t.area.first_baseline_min = baseline;
            }
        }
        t.xf = xf;
        Some(Node::new(id, NodeKind::Text(Box::new(t))))
    }

    /// The style of each paragraph of `text` (paragraphs split at `\n`).
    fn paragraph_styles(&self, text: &str, scale: f64) -> Vec<ParaStyle> {
        let count = text.split('\n').count();
        let mut out = Vec::with_capacity(count);
        let mut runs = self.paras.iter();
        let mut current = runs.next();
        let mut left = current.map_or(usize::MAX, |r| r.0);
        for p in text.split('\n') {
            let para = current.map(|r| r.1.clone()).unwrap_or_default();
            out.push(para_style(&para, scale));
            // Each paragraph and its ending `\r` count.
            let mut used = p.chars().count() + 1;
            while used > 0 {
                if left > used {
                    left -= used;
                    used = 0;
                } else {
                    used -= left;
                    current = runs.next();
                    left = current.map_or(usize::MAX, |r| r.0);
                    if current.is_none() {
                        break;
                    }
                }
            }
        }
        out
    }
}

fn para_style(p: &Para, scale: f64) -> ParaStyle {
    ParaStyle {
        justify: match p.justify {
            1 => Justify::Right,
            2 => Justify::Center,
            3 => Justify::JustifyLeft,
            4 => Justify::JustifyRight,
            5 => Justify::JustifyCenter,
            6 => Justify::JustifyAll,
            _ => Justify::Left,
        },
        left_indent: p.left_indent * scale,
        right_indent: p.right_indent * scale,
        first_line_indent: p.first_indent * scale,
        space_before: p.space_before * scale,
        space_after: p.space_after * scale,
        hyphenate: p.hyphenate,
        ..ParaStyle::default()
    }
}

fn char_style(s: &Style, scale: f64, leading: Option<f64>) -> CharStyle {
    let (family, style) = s.font.as_deref().map_or_else(|| (CharStyle::default().font_family, "Regular".to_string()), crate::family_style);
    let family = vectnor_text::FontDb::global().find_family(&family).unwrap_or(family);
    CharStyle {
        font_family: family,
        font_style: style,
        size: s.size * scale,
        leading: s.leading.map(|l| l * scale).or(leading),
        tracking: s.tracking,
        baseline_shift: s.baseline_shift * scale,
        position: s.position,
        h_scale: s.h_scale * 100.0,
        v_scale: s.v_scale * 100.0,
        fill: s.fill.map_or(Paint::None, Paint::solid),
        stroke: s.stroke.map_or(Paint::None, Paint::solid),
        stroke_width: if s.stroke.is_some() { s.stroke_width * scale } else { 0.0 },
        all_caps: s.caps == 2,
        // At the default Small Caps size of Document Setup: the file's own isn't read.
        small_caps: (s.caps == 1).then(|| vectnor_doc::DocSetup::default().small_caps_size),
        ..CharStyle::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_read_nested_dictionaries_lists_and_utf16_strings() {
        let v = read(b" /1 << /0 [ 1 2.5 -3 ] /2 (\xfe\xff\x00h\x00i) /3 true /99 /Name >> /0 (a\\)b\\n)").unwrap();
        let one = v.get("1").unwrap();
        assert_eq!(one.get("0").unwrap().nums(), Some(vec![1.0, 2.5, -3.0]));
        assert_eq!(one.get("2").unwrap().str(), Some("hi"));
        assert_eq!(one.get("3"), Some(&Val::Bool(true)));
        assert!(one.is_node("Name"));
        assert_eq!(v.get("0").unwrap().str(), Some("a)b\n"));
    }

    #[test]
    fn damaged_documents_are_not_read() {
        for bad in [&b"/0 << /1"[..], b"/0 [ 1 2", b"/0 (abc", b"/0 nope", b"0 1"] {
            assert!(read(bad).is_none(), "{}", String::from_utf8_lossy(bad));
        }
        let deep = format!("/0 {}", "[".repeat(200));
        assert!(read(deep.as_bytes()).is_none());
    }
    /// A text document's frame on a path of `segments` (each `x0 y0 x1 y1 x2 y2 x3 y3`) with the
    /// range `range`, written by hand.
    fn on_path_frame(segments: &[[f64; 8]], range: [f64; 2]) -> Option<FrameKind> {
        let nums = |v: &[f64]| v.iter().map(|n| format!("{n}")).collect::<Vec<_>>().join(" ");
        let all: Vec<f64> = segments.iter().flatten().copied().collect();
        let doc = format!("/0 << /1 << /0 [ {} ] >> /2 << /0 2 /6 [ {} ] >> >>", nums(&all), nums(&range));
        let texts = Texts {
            stories: vec![],
            fonts: vec![],
            defaults: Val::Dict(BTreeMap::new()),
            para_defaults: Val::Dict(BTreeMap::new()),
            char_sheets: vec![],
            para_sheets: vec![],
            frames: vec![read(doc.as_bytes()).unwrap()],
        };
        texts.frame(0)
    }

    /// A circle of radius 100 round the origin in four cubics, from its right, closed.
    fn circle() -> Vec<[f64; 8]> {
        let k = 55.228475;
        vec![
            [100.0, 0.0, 100.0, k, k, 100.0, 0.0, 100.0],
            [0.0, 100.0, -k, 100.0, -100.0, k, -100.0, 0.0],
            [-100.0, 0.0, -100.0, -k, -k, -100.0, 0.0, -100.0],
            [0.0, -100.0, k, -100.0, 100.0, -k, 100.0, 0.0],
        ]
    }

    /// The text object of type `text` on `frame`, and how much of its path its brackets span.
    fn on_path(text: &str, frame: FrameKind) -> (TextObject, f64) {
        let reader = frame_reader(b"/0 << >>");
        let st = Story {
            text: text.into(),
            anchor: (0.0, 0.0),
            lines: vec![],
            frame_starts: vec![None],
            frames: vec![frame],
            paras: vec![],
            runs: vec![(text.chars().count(), reader.style(None))],
        };
        let node = st.node(NodeId(1), 0, (0.0, 0.0), Affine::IDENTITY).unwrap();
        let NodeKind::Text(t) = node.kind else { panic!("not type") };
        let (a, b) = t.kind.path_span().unwrap();
        (*t, b - a)
    }

    /// Round a closed path the file writes the end bracket unwrapped, past the last segment (the
    /// start plus the run): the type keeps that run, nearly the whole circle here, not the bit
    /// from the start to the path's end.
    #[test]
    fn type_on_a_closed_path_keeps_its_unwrapped_end() {
        let Some(FrameKind::OnPath { start, end, .. }) = on_path_frame(&circle(), [3.86426, 7.85426]) else { panic!("on a path") };
        assert!((start - 3.86426).abs() < 1e-9 && (end - 3.85426).abs() < 1e-9, "{start} {end}");
        let (t, span) = on_path("type", on_path_frame(&circle(), [3.86426, 7.85426]).unwrap());
        assert!((span - 0.9975).abs() < 1e-3, "{span}");
        assert_eq!(t.plain_text(), "type");
        // Wrapped the other way, or below zero, the brackets are the same places.
        for range in [[3.86426, 3.85426], [-0.13574, 3.85426]] {
            let (_, other) = on_path("type", on_path_frame(&circle(), range).unwrap());
            assert!((other - span).abs() < 1e-6, "{range:?}: {other}");
        }
        // Values that aren't numbers fall back to the start of the path, a full turn.
        let Some(FrameKind::OnPath { start, end, .. }) = on_path_frame(&circle(), [f64::NAN, f64::INFINITY]) else { panic!("on a path") };
        assert!(start.is_finite() && end.is_finite());
    }

    /// A range inside the loop is kept as it is, and a run of exactly the loop is a full turn,
    /// where a short line of type doesn't overflow.
    #[test]
    fn type_on_a_closed_path_keeps_a_range_inside_it_and_a_full_turn() {
        let (_, half) = on_path("type", on_path_frame(&circle(), [0.5, 2.5]).unwrap());
        assert!((half - 0.5).abs() < 1e-3, "{half}");
        let (t, full) = on_path("type", on_path_frame(&circle(), [1.0, 5.0]).unwrap());
        assert!((full - 1.0).abs() < 1e-9, "{full}");
        assert!(!vectnor_text::layout(vectnor_text::FontDb::global(), &t).overflow, "a short line fits a full turn");
    }

    /// An open path keeps clamping: a range past its end stops at it.
    #[test]
    fn type_on_an_open_path_clamps_its_range() {
        let open: Vec<[f64; 8]> = circle().into_iter().take(2).collect();
        let Some(FrameKind::OnPath { start, end, .. }) = on_path_frame(&open, [0.5, 7.0]) else { panic!("on a path") };
        assert_eq!((start, end), (0.5, 2.0));
        let (_, span) = on_path("type", on_path_frame(&open, [0.5, 7.0]).unwrap());
        assert!((span - 0.75).abs() < 1e-3, "{span}");
    }

    fn frame_reader(frames: &[u8]) -> Texts {
        Texts {
            stories: vec![],
            fonts: vec![],
            defaults: Val::Dict(BTreeMap::new()),
            para_defaults: Val::Dict(BTreeMap::new()),
            char_sheets: vec![],
            para_sheets: vec![],
            frames: read(frames).unwrap().get("0").unwrap().list().to_vec(),
        }
    }

    #[test]
    fn point_frame_defaults_to_identity_but_rejects_a_malformed_explicit_matrix() {
        let t = frame_reader(b"/0 [ << /0 << /2 << >> >> >> << /0 << /2 << /2 [1 0] >> >> >> ]");
        assert!(matches!(t.frame(0), Some(FrameKind::Point(m)) if m == Affine::IDENTITY.as_coeffs()));
        assert!(t.frame(1).is_none());
    }

    #[test]
    fn translated_area_frames_apply_their_own_matrix() {
        let t = frame_reader(b"/0 [ << /0 << /1 << /0 [10 20 10 20 30 40 30 40] >> /2 << /0 1 /2 [1 0 0 1 100 200] >> >> >> ]");
        let Some(FrameKind::Area { segments: s, .. }) = t.frame(0) else { panic!("area") };
        assert_eq!(s, [[110.0, 220.0, 110.0, 220.0, 130.0, 240.0, 130.0, 240.0]]);
        let t = frame_reader(b"/0 [ << /0 << /1 << /0 [10 20 10 20 30 40 30 40] >> /2 << /0 1 /2 [2 0 0 1 0 0] >> >> >> ]");
        assert!(t.frame(0).is_none(), "non-translation needs corresponding style transforms");
    }

    #[test]
    fn character_positions_retain_superscript_and_subscript_styles() {
        let t = frame_reader(b"/0 []");
        for (value, expected) in
            [(1, CharPosition::Superscript(ScriptMetrics::DEFAULT)), (2, CharPosition::Subscript(ScriptMetrics::DEFAULT)), (0, CharPosition::Normal)]
        {
            let v = read(format!("/0 << /6 << /1 20 /13 {value} >> >>").as_bytes()).unwrap();
            assert_eq!(char_style(&t.style(v.get("0")), 1.0, None).position, expected);
        }
    }
    #[test]
    fn area_first_baseline_comes_from_its_layout_after_frame_translation() {
        let mut t = frame_reader(b"/0 [ << /0 << /1 << /0 [10 20 10 20 110 80 110 80] >> /2 << /0 1 /2 [1 0 0 1 7 9] >> >> >> ]");
        let story=read(b"/0 << /0 << /0 (First\\rSecond\\r) >> /1 << /0 [ << /0 0 >> ] /2 [ << /99 /F /0 << /0 [10 20] >> /6 [ << /99 /L /0 << /0 [0 12] >> /6 [ << /99 /S >> ] >> ] >> ] >> >>").unwrap();
        t.stories = vec![story.get("0").unwrap().clone()];
        let story = t.story(0).unwrap();
        let node = story.node(NodeId(1), 0, (0.0, 0.0), Affine::new([1.0, 0.0, 0.0, -1.0, 0.0, 0.0])).unwrap();
        let NodeKind::Text(text) = node.kind else { panic!("text") };
        assert_eq!(text.area.first_baseline, FirstBaseline::Fixed);
        assert_eq!(text.area.first_baseline_min, 12.0);
    }
    #[test]
    fn frame_baselines_match_explicit_identity_and_reject_duplicate_layouts() {
        let mut t = frame_reader(b"/0 [ << /0 << /1 << /0 [10 20 10 20 110 80 110 80] >> /2 << /0 1 >> >> >> ]");
        let story=read(b"/0 << /0 << /0 (Example) >> /1 << /0 [ << /0 0 >> ] /2 [ << /99 /F /10 7 /0 << /0 [10 20] >> /6 [ << /99 /L /0 << /0 [0 55] >> /6 [ << /99 /S >> ] >> ] >> << /99 /F /10 0 /0 << /0 [10 20] >> /6 [ << /99 /L /0 << /0 [0 12] >> /6 [ << /99 /S >> ] >> ] >> ] >> >>").unwrap();
        t.stories = vec![story.get("0").unwrap().clone()];
        let story = t.story(0).unwrap();
        assert_eq!(story.frame_starts, [Some((10.0, 32.0))]);
        let Val::Dict(story) = t.stories.first_mut().unwrap() else { panic!("story") };
        let Val::Dict(view) = story.get_mut("1").unwrap() else { panic!("view") };
        let Val::List(layout) = view.get_mut("2").unwrap() else { panic!("layout") };
        layout[0] = layout[1].clone();
        assert_eq!(t.story(0).unwrap().frame_starts, [None]);
        let Val::Dict(story) = t.stories.first_mut().unwrap() else { panic!("story") };
        let Val::Dict(view) = story.get_mut("1").unwrap() else { panic!("view") };
        let reference = view.get("0").unwrap().list()[0].clone();
        let layout = view.get("2").unwrap().list()[0].clone();
        view.insert("0".into(), Val::List(vec![reference; 1001]));
        view.insert("2".into(), Val::List(vec![layout; 1001]));
        assert!(t.story(0).is_none(), "hostile frame/layout cross-products stay bounded");
    }
    #[test]
    fn area_auto_leading_does_not_double_count_paragraph_spacing() {
        let mut t = frame_reader(b"/0 [ << /0 << /1 << /0 [10 20 10 20 110 80 110 80] >> /2 << /0 1 >> >> >> ]");
        let story=read(b"/0 << /0 << /0 (First\\rSecond\\r) /5 << /0 [ << /0 << /0 << /5 << /5 8 >> >> >> /1 13 >> ] >> >> /1 << /0 [ << /0 0 >> ] /2 [ << /99 /F /0 << /0 [10 20] >> /6 [ << /99 /L /0 << /0 [0 10] >> /6 [ << /99 /S >> ] >> << /99 /L /0 << /0 [0 32.4] >> /6 [ << /99 /S >> ] >> ] >> ] >> >>").unwrap();
        t.stories = vec![story.get("0").unwrap().clone()];
        let story = t.story(0).unwrap();
        let node = story.node(NodeId(1), 0, (0.0, 0.0), Affine::IDENTITY).unwrap();
        let NodeKind::Text(text) = node.kind else { panic!("text") };
        assert_eq!(text.para.space_after, 8.0);
        assert!(text.runs.iter().all(|r| r.style.leading.is_none()), "saved baseline gap includes paragraph space, not an explicit leading override");
    }

    #[test]
    fn character_style_inheritance_is_bounded_and_run_attributes_win() {
        let mut t = frame_reader(b"/0 []");
        t.defaults = read(b"/1 12 /8 10").unwrap();
        t.char_sheets = vec![read(b"/5 0 /6 << /1 18 /8 20 >>").unwrap()];
        let run = read(b"/5 0 /6 << /1 24 >>").unwrap();
        let st = t.style(Some(&run));
        assert_eq!((st.size, st.tracking), (24.0, 20.0));
        let missing = read(b"/5 4096 /6 << /1 24 >>").unwrap();
        let st = t.style(Some(&missing));
        assert_eq!((st.size, st.tracking), (24.0, 20.0));
    }

    #[test]
    fn paragraph_sheet_cycles_are_bounded_and_run_attributes_win() {
        let mut t = frame_reader(b"/0 []");
        t.para_defaults = read(b"/1 2 /9 true").unwrap();
        t.para_sheets = vec![read(b"/6 0 /5 << /1 6 /9 false >>").unwrap()];
        let own = read(b"/6 0 /5 << /1 8 >>").unwrap();
        assert_eq!((t.para(Some(&own)).first_indent, t.para(Some(&own)).hyphenate), (8.0, false));
        assert_eq!((t.para(None).first_indent, t.para(None).hyphenate), (6.0, false));
    }
}
