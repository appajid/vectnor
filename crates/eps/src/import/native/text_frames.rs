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
    if data.len() > MAX_RECOVERY_DATA || page.artboards.len() != 1 {
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
    text: String,
    safe: bool,
    direction: (f64, f64),
}

fn canonical(text: &str) -> String {
    text.chars().filter(|c| !c.is_whitespace()).collect()
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
            Rect::new(-reach, -size * 0.5, reach, leading * lines.saturating_sub(1) as f64 + size * 0.5)
        }
        TextKind::OnPath { .. } => return None,
    };
    let inverse = t.xf.inverse();
    let text = canonical(&text);
    if text.is_empty() || !region.is_finite() || !inverse.as_coeffs().iter().all(|v| v.is_finite()) {
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
    threaded: &HashSet<NodeId>,
    shift: Affine,
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
            if threaded.contains(&n.id) {
                continue;
            }
            if let Some(s) = source(n, shift) {
                *bytes = bytes.saturating_add(s.text.len());
                if out.len() >= MAX_SLOTS || *bytes > MAX_TEXT {
                    return false;
                }
                out.push(s);
            }
        } else if let Some(c) = n.children()
            && !collect_sources(c, threaded, shift, out, bytes, depth + 1, visited)
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
            out.push(Piece {
                id: n.id,
                parent,
                index,
                at: t.xf * Point::ZERO,
                text,
                direction: (a / length, b / length),
                safe: length.is_finite()
                    && length > 1e-9
                    && a * d - b * c > 0.0
                    && matches!(t.kind, TextKind::Point)
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

/// Recover only on a single, matching artboard: another page/crop/rotation cannot safely use
/// the native positions. Threaded/path type and spans across different graphics contexts stay
/// untouched. Ordinary direct AI imports already retain those frames and their threading.
fn recover_document(native: &Document, page: &mut Document) -> usize {
    if native.artboards.len() != 1 || page.artboards.len() != 1 {
        return 0;
    }
    let (Some(a), Some(b)) = (native.artboards.first(), page.artboards.first()) else { return 0 };
    if (a.rect.width() - b.rect.width()).abs() > 0.5 || (a.rect.height() - b.rect.height()).abs() > 0.5 {
        return 0;
    }
    let shift = Affine::translate(b.rect.origin() - a.rect.origin());
    let threaded = native.text_threads.iter().flatten().copied().collect();
    let (mut sources, mut pieces) = (vec![], vec![]);
    if !collect_sources(&native.layers, &threaded, shift, &mut sources, &mut 0, 0, &mut 0)
        || !collect_pieces(&page.layers, None, &mut pieces, &mut 0, 0, &mut 0)
        || sources.len().saturating_mul(pieces.len()) > MAX_PAIRS
        || sources.iter().map(|s| s.text.len()).sum::<usize>().saturating_mul(pieces.len()) > MAX_MATCH_BYTES
    {
        return 0;
    }
    let mut plans = vec![];
    let mut claims: HashMap<NodeId, usize> = HashMap::new();
    for s in sources {
        let matched: Vec<&Piece> = pieces
            .iter()
            .filter(|p| {
                let dot = p.direction.0 * s.direction.0 + p.direction.1 * s.direction.1;
                s.region.contains(s.inverse * p.at) && dot.is_finite() && (dot - 1.0).abs() <= 1e-4 && s.text.contains(&p.text)
            })
            .collect();
        // Even a frame that cannot be rebuilt claims its plausible fragments. Otherwise an
        // overlapping frame could absorb text belonging to a separate original object.
        for p in &matched {
            *claims.entry(p.id).or_default() += 1;
        }
        let Some(first) = matched.first().copied() else { continue };
        if matched
            .iter()
            .any(|p| !p.safe || p.parent != first.parent || (p.direction.0 * s.direction.0 + p.direction.1 * s.direction.1 - 1.0).abs() > 1e-4)
            || matched.windows(2).any(|w| !matches!(w, [a, b] if b.index == a.index + 1))
            || !matched.iter().flat_map(|p| p.text.chars()).eq(s.text.chars())
        {
            continue;
        }
        let ids: Vec<NodeId> = matched.iter().map(|p| p.id).collect();
        // Whole one-piece frames also claim their fragments, preventing a different frame
        // from absorbing them just because its region overlaps.
        if ids.len() > 1 {
            plans.push((s.node, ids));
        }
    }
    let mut replacements = HashMap::new();
    let mut removed = HashSet::new();
    for (mut node, ids) in plans {
        if ids.iter().any(|id| claims.get(id) != Some(&1)) {
            continue;
        }
        let Some((first, rest)) = ids.split_first() else { continue };
        node.id = *first;
        replacements.insert(*first, node);
        removed.extend(rest.iter().copied());
    }
    let restored = replacements.len();
    if restored > 0 {
        apply(&mut page.layers, &mut replacements, &removed);
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
}
