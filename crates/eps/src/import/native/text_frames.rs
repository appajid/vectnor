//! Recover original text-frame boundaries when unsupported artwork requires the PDF fallback.
//! Frame identities and styled text come only from AI editing data. Geometry is a filter, never
//! a reason to join text: every fragment must reproduce one complete source frame, and no
//! fragment may belong to two frames. Uncertain cases keep the page's objects unchanged.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use vectnor_color::BlendMode;
use vectnor_doc::{Document, Node, NodeId, NodeKind, TextKind};
use vectnor_geom::{Affine, Point, Rect};

use super::{MAX_PAIRS, MAX_SLOTS, MAX_TEXT, Page, ai, build_structure};

/// Avoid an additional large artwork parse on already expensive fallback imports.
const MAX_RECOVERY_DATA: usize = 64 << 20;
const MAX_DEPTH: usize = 256;
const MAX_SCANNED_NODES: usize = 100_000;
const MAX_MATCH_BYTES: usize = 64 << 20;

pub(super) fn recover(data: &[u8], page: &mut Document) -> usize {
    if data.len() > MAX_RECOVERY_DATA || page.artboards.is_empty() {
        return 0;
    }
    let Ok(structure) = ai::read_for_text(data) else { return 0 };
    let Ok((native, notes)) = build_structure(data, structure, None, Page::Artboard, false) else { return 0 };
    // An unread frame may share a story with a readable one. Without all frame identities,
    // the remaining story must not absorb the unread frame's printed text.
    if notes.iter().any(|n| n.ends_with(super::TYPE_LEFT_OUT)) {
        return 0;
    }
    recover_document(&native, page)
}

struct Source {
    node: Node,
    inverse: Affine,
    region: Rect,
    text: String,
    direction: (f64, f64),
}

struct Piece {
    id: NodeId,
    parent: Option<NodeId>,
    index: usize,
    at: Point,
    extent: Option<Rect>,
    text: String,
    safe: bool,
    direction: (f64, f64),
}

fn canonical(text: &str) -> String {
    text.chars().filter(|c| !c.is_whitespace()).fold(String::new(), |mut out, c| {
        match c {
            '\u{fb00}' => out.push_str("ff"),
            '\u{fb01}' => out.push_str("fi"),
            '\u{fb02}' => out.push_str("fl"),
            '\u{fb03}' => out.push_str("ffi"),
            '\u{fb04}' => out.push_str("ffl"),
            '\u{fb05}' | '\u{fb06}' => out.push_str("st"),
            _ => out.push(c),
        }
        out
    })
}

fn source(n: &Node, shift: Affine) -> Option<Source> {
    let mut node = n.clone();
    node.transform(shift, false);
    let NodeKind::Text(t) = &node.kind else { return None };
    let text = t.plain_text();
    let region = match &t.kind {
        TextKind::Area { frame } => frame.bounds()?,
        TextKind::Point => {
            let size = t.runs.iter().map(|r| r.style.size).fold(0.0, f64::max);
            let leading = t.runs.iter().map(|r| r.style.leading.unwrap_or(r.style.size * 1.2)).fold(0.0, f64::max);
            let lines = text.split('\n').count();
            let longest = text.split('\n').map(|l| l.chars().count()).max().unwrap_or(0) as f64;
            let reach = (longest + 1.0) * size * 3.0;
            Rect::new(-reach, -size * 0.5, reach, leading * lines.saturating_sub(1) as f64 + size * 1.5)
        }
        TextKind::OnPath { .. } => return None,
    };
    let inverse = t.xf.inverse();
    let text = canonical(&text);
    if !region.is_finite() || !inverse.as_coeffs().iter().all(|v| v.is_finite()) {
        return None;
    }
    let [a, b, ..] = t.xf.as_coeffs();
    let length = a.hypot(b);
    if !length.is_finite() || length < 1e-9 {
        return None;
    }
    let direction = (a / length, b / length);
    Some(Source { node, inverse, region: region.inflate(0.5, 0.5), text, direction })
}

fn collect_sources(
    nodes: &[Arc<Node>],
    boards: &[(Rect, Affine)],
    out: &mut Vec<Source>,
    bytes: &mut usize,
    depth: usize,
    visited: &mut usize,
) -> bool {
    if depth > MAX_DEPTH {
        return false;
    }
    for n in nodes {
        *visited = visited.saturating_add(1);
        if *visited > MAX_SCANNED_NODES {
            return false;
        }
        if !n.visible {
            continue;
        }
        if matches!(n.kind, NodeKind::Text(_)) {
            let Some(raw) = source(n, Affine::IDENTITY) else { continue };
            let NodeKind::Text(t) = &n.kind else { continue };
            let on = match t.kind {
                TextKind::Point => Rect::from_origin_size(t.xf * Point::ZERO, (0.0, 0.0)),
                _ => t.xf.transform_rect_bbox(raw.region.inflate(-0.5, -0.5)),
            };
            let mut matches = boards.iter().filter(|(b, _)| b.inflate(0.5, 0.5).contains_rect(on));
            let shift = if boards.len() == 1 {
                boards.first().map(|(_, m)| *m)
            } else {
                matches.next().filter(|_| matches.next().is_none()).map(|(_, m)| *m)
            };
            if let Some(s) = shift.and_then(|m| source(n, m)) {
                *bytes = bytes.saturating_add(s.text.len());
                if out.len() >= MAX_SLOTS || *bytes > MAX_TEXT {
                    return false;
                }
                out.push(s);
            }
        } else if let Some(c) = n.children()
            && !collect_sources(c, boards, out, bytes, depth + 1, visited)
        {
            return false;
        }
    }
    true
}

fn collect_pieces(nodes: &[Arc<Node>], parent: Option<NodeId>, out: &mut Vec<Piece>, bytes: &mut usize, depth: usize, visited: &mut usize) -> bool {
    if depth > MAX_DEPTH {
        return false;
    }
    for (index, n) in nodes.iter().enumerate() {
        *visited = visited.saturating_add(1);
        if *visited > MAX_SCANNED_NODES {
            return false;
        }
        if !n.visible {
            continue;
        }
        if let NodeKind::Text(t) = &n.kind {
            let text = canonical(&t.plain_text());
            *bytes = bytes.saturating_add(text.len());
            if out.len() >= MAX_SLOTS || *bytes > MAX_TEXT {
                return false;
            }
            let [a, b, c, d, ..] = t.xf.as_coeffs();
            let length = a.hypot(b);
            let extent = if let TextKind::Area { frame } = &t.kind { frame.bounds().map(|r| t.xf.transform_rect_bbox(r)) } else { None };
            out.push(Piece {
                id: n.id,
                parent,
                index,
                at: t.xf * Point::ZERO,
                extent,
                text,
                direction: (a / length, b / length),
                safe: length.is_finite()
                    && length > 1e-9
                    && a * d - b * c > 0.0
                    && (matches!(t.kind, TextKind::Point) || extent.is_some())
                    && n.opacity == 1.0
                    && n.blend == BlendMode::Normal
                    && n.mask.is_none()
                    && n.appearance.items.is_empty()
                    && n.appearance.effects.is_empty(),
            });
        } else if let Some(c) = n.children()
            && !collect_pieces(c, Some(n.id), out, bytes, depth + 1, visited)
        {
            return false;
        }
    }
    true
}

/// Pair complete artboard sets in file order. Cropped, missing, or ambiguous boards do
/// not provide a safe coordinate mapping for the original frame identities.
fn recover_document(native: &Document, page: &mut Document) -> usize {
    if native.artboards.is_empty() || native.artboards.len() != page.artboards.len() || native.artboards.len() > 1000 {
        return 0;
    }
    let mut boards = vec![];
    for (a, b) in native.artboards.iter().zip(&page.artboards) {
        if !a.rect.is_finite()
            || !b.rect.is_finite()
            || (a.rect.width() - b.rect.width()).abs() > 0.5
            || (a.rect.height() - b.rect.height()).abs() > 0.5
        {
            return 0;
        }
        boards.push((a.rect, Affine::translate(b.rect.origin() - a.rect.origin())));
    }
    let (mut sources, mut pieces) = (vec![], vec![]);
    if !collect_sources(&native.layers, &boards, &mut sources, &mut 0, 0, &mut 0)
        || !collect_pieces(&page.layers, None, &mut pieces, &mut 0, 0, &mut 0)
        || sources.len().saturating_mul(pieces.len()) > MAX_PAIRS
        || sources.iter().map(|s| s.text.len()).sum::<usize>().saturating_mul(pieces.len()) > MAX_MATCH_BYTES
    {
        return 0;
    }
    // A linked story is recovered atomically, keeping every original frame and link. The
    // first native frame holds its story; engine import subsequently flows it through them.
    let mut sources: HashMap<NodeId, Source> = sources.into_iter().map(|s| (s.node.id, s)).collect();
    let mut units = vec![];
    for thread in &native.text_threads {
        if thread.len() < 2 || thread.iter().any(|id| !sources.contains_key(id)) {
            return 0;
        }
        let frames: Vec<Source> = thread.iter().filter_map(|id| sources.remove(id)).collect();
        if frames.len() == thread.len() && frames.len() > 1 {
            units.push(frames);
        }
    }
    let mut singles: Vec<Source> = sources.into_values().collect();
    singles.sort_by_key(|s| s.node.id);
    units.extend(singles.into_iter().filter(|s| !s.text.is_empty()).map(|s| vec![s]));
    let mut plans = vec![];
    let mut claims: HashMap<NodeId, usize> = HashMap::new();
    let mut work = 0usize;
    for frames in units {
        let text: String = frames.iter().map(|s| s.text.as_str()).collect();
        if text.is_empty() {
            continue;
        }
        work = work.saturating_add(text.len().saturating_mul(pieces.len()).saturating_mul(frames.len()));
        if work > MAX_MATCH_BYTES {
            return 0;
        }
        let mut owned = HashSet::new();
        let mut matches = vec![];
        let mut unique = true;
        for s in &frames {
            let matched: Vec<&Piece> = pieces
                .iter()
                .filter(|p| {
                    let dot = p.direction.0 * s.direction.0 + p.direction.1 * s.direction.1;
                    s.region.contains(s.inverse * p.at)
                        && dot.is_finite()
                        && (dot - 1.0).abs() <= 1e-4
                        && !p.text.is_empty()
                        && text.contains(&p.text)
                })
                .collect();
            for p in &matched {
                if !owned.insert(p.id) {
                    unique = false;
                }
            }
            matches.push(matched);
        }
        // Partial stories also claim their plausible pieces, so a neighbouring original
        // can never absorb them. Multiple frames in one story count as one owner.
        for id in owned {
            *claims.entry(id).or_default() += 1;
        }
        if !unique || matches.iter().any(|m| m.is_empty()) {
            continue;
        }
        let valid = frames.iter().zip(&matches).all(|(s, matched)| {
            let Some(first) = matched.first() else { return false };
            matched.iter().all(|p| {
                p.safe
                    && p.parent == first.parent
                    && p.extent.is_none_or(|r| r.is_finite() && s.region.contains_rect(s.inverse.transform_rect_bbox(r)))
            }) && matched.windows(2).all(|w| matches!(w, [a, b] if b.index == a.index + 1))
        });
        if !valid || !matches.iter().flatten().flat_map(|p| p.text.chars()).eq(text.chars()) {
            continue;
        }
        if frames.len() == 1 && matches.first().is_none_or(|m| m.len() < 2) {
            continue;
        }
        plans.push((frames, matches.iter().map(|m| m.iter().map(|p| p.id).collect::<Vec<_>>()).collect::<Vec<_>>()));
    }
    let mut replacements = HashMap::new();
    let mut removed = HashSet::new();
    let mut threads = vec![];
    for (frames, pieces) in plans {
        if pieces.iter().flatten().any(|id| claims.get(id) != Some(&1)) {
            continue;
        }
        let mut thread = vec![];
        for (s, ids) in frames.into_iter().zip(pieces) {
            let Some((first, rest)) = ids.split_first() else { continue };
            let mut node = s.node;
            node.id = *first;
            replacements.insert(*first, node);
            removed.extend(rest.iter().copied());
            thread.push(*first);
        }
        if thread.len() > 1 {
            threads.push(thread);
        }
    }
    let restored = replacements.len();
    if restored > 0 {
        apply(&mut page.layers, &mut replacements, &removed);
        page.text_threads.extend(threads);
    }
    restored
}

fn apply(nodes: &mut Vec<Arc<Node>>, replacements: &mut HashMap<NodeId, Node>, removed: &HashSet<NodeId>) {
    nodes.retain(|n| !removed.contains(&n.id));
    for n in nodes {
        if let Some(mut replacement) = replacements.remove(&n.id) {
            replacement.visible = n.visible;
            replacement.locked = n.locked;
            *n = Arc::new(replacement);
        } else if n.visible
            && n.children().is_some()
            && let Some(c) = Arc::make_mut(n).children_mut()
        {
            apply(c, replacements, removed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kurbo::Shape;
    use vectnor_color::{Color, Paint};
    use vectnor_doc::{CharStyle, TextObject, TextRun};
    use vectnor_geom::PathData;

    fn area(d: &mut Document, text: &str, rect: Rect) -> NodeId {
        let mut t = TextObject::point(Point::ZERO, text, CharStyle::default());
        t.kind = TextKind::Area { frame: PathData::from_bezpath(&rect.to_path(0.1)) };
        add(d, t)
    }

    fn piece(d: &mut Document, text: &str, x: f64, y: f64) -> NodeId {
        add(d, TextObject::point(Point::new(x, y), text, CharStyle::default()))
    }

    fn add(d: &mut Document, t: TextObject) -> NodeId {
        let id = d.alloc_id();
        Arc::make_mut(&mut d.layers[0]).children_mut().unwrap().push(Arc::new(Node::new(id, NodeKind::Text(Box::new(t)))));
        id
    }

    fn texts(d: &Document) -> Vec<&TextObject> {
        d.layers[0].children().unwrap().iter().filter_map(|n| if let NodeKind::Text(t) = &n.kind { Some(&**t) } else { None }).collect()
    }

    #[test]
    fn multiline_and_mixed_style_fragments_recover_as_separate_original_frames() {
        let mut native = Document::new(200.0, 160.0);
        let first = area(&mut native, "Alpha Beta\nGamma", Rect::new(10.0, 10.0, 180.0, 90.0));
        let NodeKind::Text(t) = &mut native.node_mut(first).unwrap().kind else { panic!("text") };
        t.runs = vec![
            TextRun::new("Alpha ", CharStyle { fill: Paint::solid(Color::rgb(1.0, 0.0, 0.0)), ..Default::default() }),
            TextRun::new("Beta\nGamma", CharStyle { size: 16.0, ..Default::default() }),
        ];
        area(&mut native, "Other\nbox", Rect::new(10.0, 100.0, 180.0, 150.0));
        let mut page = Document::new(200.0, 160.0);
        piece(&mut page, "Alpha", 20.0, 30.0);
        piece(&mut page, "Beta", 60.0, 30.0);
        piece(&mut page, "Gamma", 20.0, 50.0);
        piece(&mut page, "Other", 20.0, 115.0);
        piece(&mut page, "box", 20.0, 135.0);
        assert_eq!(recover_document(&native, &mut page), 2);
        let t = texts(&page);
        assert_eq!(t.iter().map(|t| t.plain_text()).collect::<Vec<_>>(), ["Alpha Beta\nGamma", "Other\nbox"]);
        assert!(t.iter().all(|t| matches!(t.kind, TextKind::Area { .. })));
        assert_eq!(t[0].runs.len(), 2);
        assert_eq!(t[0].runs[1].style.size, 16.0);
        assert_eq!(t[0].runs[0].style.fill, Paint::solid(Color::rgb(1.0, 0.0, 0.0)));
    }

    #[test]
    fn point_type_lines_recover_with_their_original_line_breaks() {
        let mut native = Document::new(200.0, 160.0);
        piece(&mut native, "One\nTwo", 20.0, 30.0);
        let mut page = Document::new(200.0, 160.0);
        piece(&mut page, "One", 20.0, 30.0);
        piece(&mut page, "Two", 20.0, 44.0);
        assert_eq!(recover_document(&native, &mut page), 1);
        assert_eq!(texts(&page)[0].plain_text(), "One\nTwo");
        assert!(matches!(texts(&page)[0].kind, TextKind::Point));
        assert_eq!(texts(&page)[0].xf, Affine::translate((20.0, 30.0)));
    }

    #[test]
    fn unicode_joiners_and_separate_original_boxes_stay_intact() {
        let mut native = Document::new(200.0, 160.0);
        area(&mut native, "తెలుగు\nक्‌क क्‍क", Rect::new(10.0, 10.0, 180.0, 90.0));
        area(&mut native, "Separate", Rect::new(10.0, 100.0, 180.0, 150.0));
        let mut page = Document::new(200.0, 160.0);
        piece(&mut page, "తెలుగు", 20.0, 30.0);
        piece(&mut page, "क्‌क", 20.0, 50.0);
        piece(&mut page, "क्‍क", 60.0, 50.0);
        let separate = piece(&mut page, "Separate", 20.0, 115.0);
        let before = page.node(separate).unwrap().clone();
        assert_eq!(recover_document(&native, &mut page), 1);
        assert_eq!(texts(&page)[0].plain_text(), "తెలుగు\nक्‌क क्‍क");
        assert_eq!(page.node(separate).unwrap(), &before);
    }

    #[test]
    fn overlapping_frame_identities_never_absorb_each_others_text() {
        let mut native = Document::new(200.0, 160.0);
        area(&mut native, "AlphaBeta", Rect::new(10.0, 10.0, 180.0, 90.0));
        piece(&mut native, "Alpha", 20.0, 30.0);
        piece(&mut native, "Beta", 70.0, 30.0);
        let mut page = Document::new(200.0, 160.0);
        piece(&mut page, "Alpha", 20.0, 30.0);
        piece(&mut page, "Beta", 70.0, 30.0);
        let before = page.clone();
        assert_eq!(recover_document(&native, &mut page), 0);
        assert_eq!(page, before);
    }

    #[test]
    fn mismatches_crops_threads_and_different_graphics_contexts_are_left_alone() {
        let mut native = Document::new(200.0, 160.0);
        let id = area(&mut native, "Alpha Beta", Rect::new(10.0, 10.0, 180.0, 90.0));
        let mut page = Document::new(200.0, 160.0);
        piece(&mut page, "Alpha", 20.0, 30.0);
        let second = piece(&mut page, "BETA", 60.0, 30.0);
        let before = page.clone();
        assert_eq!(recover_document(&native, &mut page), 0);
        assert_eq!(page, before);
        let NodeKind::Text(t) = &mut page.node_mut(second).unwrap().kind else { panic!("text") };
        t.runs[0].text = "Beta".into();
        native.text_threads = vec![vec![id]];
        assert_eq!(recover_document(&native, &mut page), 0);
        native.text_threads.clear();
        page.artboards[0].rect.x1 += 18.0;
        assert_eq!(recover_document(&native, &mut page), 0);
        page.artboards[0].rect.x1 -= 18.0;
        let group_id = page.alloc_id();
        let c = Arc::make_mut(&mut page.layers[0]).children_mut().unwrap();
        let last = c.pop().unwrap();
        c.push(Arc::new(Node::group(group_id, vec![last])));
        let before = page.clone();
        assert_eq!(recover_document(&native, &mut page), 0);
        assert_eq!(page, before);
    }

    #[test]
    fn rotated_fragments_and_intervening_art_are_not_replaced() {
        let mut native = Document::new(200.0, 160.0);
        area(&mut native, "Alpha Beta", Rect::new(10.0, 10.0, 180.0, 90.0));
        let mut page = Document::new(200.0, 160.0);
        piece(&mut page, "Alpha", 20.0, 30.0);
        let second = piece(&mut page, "Beta", 60.0, 30.0);
        let NodeKind::Text(t) = &mut page.node_mut(second).unwrap().kind else { panic!("text") };
        t.xf = Affine::translate((60.0, 30.0)) * Affine::rotate(0.5);
        assert_eq!(recover_document(&native, &mut page), 0);
        let NodeKind::Text(t) = &mut page.node_mut(second).unwrap().kind else { panic!("text") };
        t.xf = Affine::translate((60.0, 30.0));
        let group_id = page.alloc_id();
        Arc::make_mut(&mut page.layers[0]).children_mut().unwrap().insert(1, Arc::new(Node::group(group_id, vec![])));
        let before = page.clone();
        assert_eq!(recover_document(&native, &mut page), 0);
        assert_eq!(page, before);
    }
    #[test]
    fn multiple_artboards_map_each_original_frame_without_joining_identical_boxes() {
        let mut native = Document::new(200.0, 160.0);
        let mut board = native.artboards[0].clone();
        board.id = 2;
        board.rect = Rect::new(0.0, 200.0, 200.0, 360.0);
        native.artboards.push(board);
        area(&mut native, "Alpha Beta", Rect::new(10.0, 10.0, 180.0, 90.0));
        area(&mut native, "Alpha Beta", Rect::new(10.0, 210.0, 180.0, 290.0));
        let mut page = Document::new(200.0, 160.0);
        let mut board = page.artboards[0].clone();
        board.id = 2;
        board.rect = Rect::new(236.0, 0.0, 436.0, 160.0);
        page.artboards.push(board);
        for x in [20.0, 256.0] {
            piece(&mut page, "Alpha", x, 30.0);
            piece(&mut page, "Beta", x + 40.0, 30.0);
        }
        assert_eq!(recover_document(&native, &mut page), 2);
        assert_eq!(texts(&page).len(), 2);
        let before = page.clone();
        page.artboards.pop();
        assert_eq!(recover_document(&native, &mut page), 0);
        page = before;
        page.artboards[1].rect.y1 += 18.0;
        assert_eq!(recover_document(&native, &mut page), 0);
    }

    #[test]
    fn linked_frames_recover_atomically_with_ligatures_and_existing_area_fragments() {
        let mut native = Document::new(200.0, 200.0);
        let a = area(&mut native, "First office\nSecond flow", Rect::new(10.0, 10.0, 180.0, 80.0));
        let b = area(&mut native, "", Rect::new(10.0, 100.0, 180.0, 180.0));
        native.text_threads = vec![vec![a, b]];
        let mut page = Document::new(200.0, 200.0);
        let first = piece(&mut page, "First", 20.0, 30.0);
        piece(&mut page, "oﬃce", 60.0, 30.0);
        let second = area(&mut page, "Second ﬂow", Rect::new(0.0, -9.0, 140.0, 12.0));
        let NodeKind::Text(t) = &mut page.node_mut(second).unwrap().kind else { panic!("text") };
        t.xf = Affine::translate((20.0, 115.0));
        assert_eq!(recover_document(&native, &mut page), 2);
        assert_eq!(page.text_threads, vec![vec![first, second]]);
        assert_eq!(texts(&page)[0].plain_text(), "First office\nSecond flow");
        assert_eq!(texts(&page)[1].plain_text(), "");
    }

    #[test]
    fn incomplete_linked_story_and_overlapping_frames_are_not_partially_recovered() {
        let mut native = Document::new(200.0, 200.0);
        let a = area(&mut native, "First Second", Rect::new(10.0, 10.0, 180.0, 80.0));
        let b = area(&mut native, "", Rect::new(10.0, 100.0, 180.0, 180.0));
        native.text_threads = vec![vec![a, b]];
        let mut page = Document::new(200.0, 200.0);
        piece(&mut page, "First", 20.0, 30.0);
        piece(&mut page, "WRONG", 20.0, 115.0);
        let before = page.clone();
        assert_eq!(recover_document(&native, &mut page), 0);
        assert_eq!(page, before);
        let NodeKind::Text(t) = &mut native.node_mut(b).unwrap().kind else { panic!("text") };
        t.kind = TextKind::Area { frame: PathData::from_bezpath(&Rect::new(10.0, 10.0, 180.0, 180.0).to_path(0.1)) };
        let last = page.layers[0].children().unwrap().last().unwrap().id;
        let NodeKind::Text(t) = &mut page.node_mut(last).unwrap().kind else { panic!("text") };
        t.runs[0].text = "Second".into();
        let before = page.clone();
        assert_eq!(recover_document(&native, &mut page), 0);
        assert_eq!(page, before);
    }
    #[test]
    fn a_printed_area_fragment_extending_outside_the_original_is_not_absorbed() {
        let mut native = Document::new(200.0, 160.0);
        area(&mut native, "Alpha Beta", Rect::new(10.0, 10.0, 180.0, 90.0));
        let mut page = Document::new(200.0, 160.0);
        piece(&mut page, "Alpha", 20.0, 30.0);
        area(&mut page, "Beta", Rect::new(-20.0, -10.0, 200.0, 150.0));
        let last = page.layers[0].children().unwrap().last().unwrap().id;
        let NodeKind::Text(t) = &mut page.node_mut(last).unwrap().kind else { panic!("text") };
        t.xf = Affine::translate((60.0, 30.0));
        let before = page.clone();
        assert_eq!(recover_document(&native, &mut page), 0);
        assert_eq!(page, before);
    }
}
