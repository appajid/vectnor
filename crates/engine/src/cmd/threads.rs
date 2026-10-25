//! Type → Threaded Text: one story flowing through area-type frames in order.
//!
//! Frames are ordinary area text objects holding their slice of the story; the document lists the
//! thread order. After every edit, threads whose frames changed are re-flowed ([`reflow`]).

use serde_json::{Value, json};
use vectnor_doc::{Document, Node, NodeId, NodeKind, TextKind, TextObject, TextRun};

use super::edit::selected_roots;
use super::typecmd::refresh_bounds;
use super::*;

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(
            "text.thread.create",
            "Create",
            ["Type", "Threaded Text"],
            None,
            "{} thread the selected area type (and closed paths, which become frames) in stacking order → {ids}",
            has_multi,
            create
        ),
        cmd!(
            "text.thread.releaseSelection",
            "Release Selection",
            ["Type", "Threaded Text"],
            None,
            "{} take the selected frames out of their threads (the story re-flows through the rest)",
            has_thread_selection,
            release
        ),
        cmd!(
            "text.thread.remove",
            "Remove Threading",
            ["Type", "Threaded Text"],
            None,
            "{} break the selected frames' threads; each frame keeps the text it shows",
            has_thread_selection,
            remove
        ),
        cmd!(query "text.thread.info", "Threads", [], None, "{} → [[id…]] every thread in order", has_doc, |s, _| {
            Ok(json!(s.doc()?.doc.text_threads.iter().map(|t| t.iter().map(|i| i.0).collect::<Vec<_>>()).collect::<Vec<_>>()))
        }),
    ]
}

fn has_thread_selection(s: &Session) -> std::result::Result<(), String> {
    has_selection(s)?;
    let st = s.active().ok_or("no document open")?;
    if st.selection.objects.iter().any(|id| st.doc.text_threads.iter().any(|t| t.contains(id))) {
        Ok(())
    } else {
        Err("no threaded text selected".into())
    }
}

fn text_of(d: &Document, id: NodeId) -> Option<&TextObject> {
    match d.node(id).map(|n| &n.kind) {
        Some(NodeKind::Text(t)) if matches!(t.kind, TextKind::Area { .. }) => Some(t),
        _ => None,
    }
}

/// Re-flow every thread with a frame that changed between `before` and `doc` (structural sharing
/// makes "changed" a pointer comparison). Deleted frames' text flows into the remaining ones; a
/// frame that is now point type leaves its thread with the text it holds.
pub(crate) fn reflow(before: &Document, doc: &mut Document) {
    let threads = std::mem::take(&mut doc.text_threads);
    let mut kept = vec![];
    for thread in threads {
        let changed = thread.iter().any(|id| match (before.node(*id), doc.node(*id)) {
            (Some(a), Some(b)) => !std::ptr::eq(a, b),
            (None, None) => false,
            _ => true,
        });
        // The story: live frames' runs, plus the text of frames deleted by this edit.
        let mut story: Vec<TextRun> = vec![];
        let mut sources: Vec<TextObject> = vec![];
        let mut later_wins = vec![];
        let mut live = vec![];
        for id in &thread {
            match (text_of(doc, *id), text_of(before, *id)) {
                (Some(t), old) => {
                    story.extend(t.runs.iter().cloned());
                    // A frame whose first paragraph was just restyled gives the paragraph it
                    // shares with the frame before it that style.
                    later_wins.push(old.is_some_and(|o| o.para_at(0) != t.para_at(0)));
                    sources.push(t.clone());
                    live.push(*id);
                }
                // A frame converted to point type keeps the text it holds and leaves the thread.
                (None, Some(_)) if matches!(doc.node(*id).map(|n| &n.kind), Some(NodeKind::Text(_))) => {}
                (None, Some(old)) => {
                    story.extend(old.runs.iter().cloned());
                    later_wins.push(false);
                    sources.push(old.clone());
                }
                (None, None) => {}
            }
        }
        if !changed || live.is_empty() {
            if live.len() >= 2 {
                kept.push(live);
            }
            continue;
        }
        let source_refs: Vec<&TextObject> = sources.iter().collect();
        let paras = vectnor_text::thread::story_paras(&source_refs, &later_wins);
        vectnor_text::edit::normalize(&mut story);
        if live.len() >= 2 {
            // Threaded frames keep their size: Auto Size is off for them (as in Illustrator).
            for id in &live {
                if text_of(doc, *id).is_some_and(|t| t.area.fit == vectnor_doc::AreaFit::AutoHeight)
                    && let Some(NodeKind::Text(t)) = doc.node_mut(*id).map(|n| &mut n.kind)
                {
                    t.area.fit = vectnor_doc::AreaFit::None;
                }
            }
        }
        let frames: Vec<TextObject> = live.iter().filter_map(|id| text_of(doc, *id).cloned()).collect();
        let refs: Vec<&TextObject> = frames.iter().collect();
        let parts = vectnor_text::thread::distribute(vectnor_text::FontDb::global(), &refs, &story, &paras);
        for (id, part) in live.iter().zip(parts) {
            if let Some(NodeKind::Text(t)) = doc.node_mut(*id).map(|n| &mut n.kind) {
                let mut next = (**t).clone();
                next.runs = part.runs;
                next.set_paragraph_styles(part.paras);
                if next.runs != t.runs || next.para != t.para || next.paras != t.paras {
                    t.runs = next.runs;
                    t.para = next.para;
                    t.paras = next.paras;
                    refresh_bounds(t);
                }
            }
        }
        if live.len() >= 2 {
            kept.push(live);
        }
    }
    doc.text_threads = kept;
}

/// A closed path converted to an empty area-type frame with `style`.
fn frame_from_path(n: &Node, style: vectnor_doc::CharStyle) -> Option<Node> {
    let NodeKind::Path { path, .. } = &n.kind else { return None };
    if !path.subpaths.first().is_some_and(|s| s.closed) {
        return None;
    }
    let mut t = TextObject::point(vectnor_geom::Point::ZERO, "", style);
    t.kind = TextKind::Area { frame: path.clone() };
    t.xf = vectnor_geom::Affine::IDENTITY;
    refresh_bounds(&mut t);
    Some(Node::new(n.id, NodeKind::Text(Box::new(t))))
}

fn create(s: &mut Session, _: &Value) -> Result<Value> {
    const C: &str = "text.thread.create";
    let ids = selected_roots(s)?;
    let d = &s.doc()?.doc;
    let style = ids.iter().find_map(|i| text_of(d, *i)).map(|t| t.first_style()).unwrap_or_default();
    let ok = ids.iter().all(|i| text_of(d, *i).is_some() || d.node(*i).is_some_and(|n| frame_from_path(n, style.clone()).is_some()));
    if ids.len() < 2 || !ok || !ids.iter().any(|i| text_of(d, *i).is_some()) {
        return Err(bad(C, "select area type and closed paths (at least one area type object)"));
    }
    let out = ids.clone();
    s.edit("Thread Text", |d, _| {
        for id in &ids {
            if text_of(d, *id).is_none() {
                let n = d.node(*id).cloned().ok_or(EngineError::NoNode(*id))?;
                let f = frame_from_path(&n, style.clone()).ok_or_else(|| bad(C, "not a closed path"))?;
                *d.node_mut(*id).ok_or(EngineError::NoNode(*id))? = f;
            }
        }
        // Existing threads containing any of these frames merge into the new one, keeping order.
        let mut order: Vec<NodeId> = vec![];
        for id in &ids {
            match d.text_threads.iter().position(|t| t.contains(id)) {
                Some(i) => {
                    for x in d.text_threads.remove(i) {
                        if !order.contains(&x) {
                            order.push(x);
                        }
                    }
                }
                None if !order.contains(id) => order.push(*id),
                None => {}
            }
        }
        d.text_threads.push(order);
        // Force a re-flow of the new thread on this edit.
        touch(d, ids[0]);
        Ok(())
    })?;
    Ok(json!({ "ids": out.iter().map(|i| i.0).collect::<Vec<_>>() }))
}

/// Give a node a fresh allocation so [`reflow`] sees its thread as changed.
fn touch(d: &mut Document, id: NodeId) {
    if let Some(n) = d.node_mut(id) {
        let copy = n.clone();
        *n = copy;
    }
}

fn release(s: &mut Session, _: &Value) -> Result<Value> {
    let ids = selected_roots(s)?;
    s.edit("Release Threaded Text", |d, _| {
        // The released frames' text joins the rest of the story (they become empty).
        for thread in d.text_threads.clone() {
            let (out, keep): (Vec<NodeId>, Vec<NodeId>) = thread.iter().partition(|i| ids.contains(i));
            if out.is_empty() {
                continue;
            }
            let mut story: Vec<TextRun> = vec![];
            let sources: Vec<TextObject> = thread.iter().filter_map(|id| text_of(d, *id).cloned()).collect();
            for t in &sources {
                story.extend(t.runs.iter().cloned());
            }
            let paras = vectnor_text::thread::story_paras(&sources.iter().collect::<Vec<_>>(), &[]);
            let style = story.first().map(|r| r.style.clone()).unwrap_or_default();
            let first_para = paras.first().cloned().unwrap_or_default();
            for id in &out {
                if let Some(NodeKind::Text(t)) = d.node_mut(*id).map(|n| &mut n.kind) {
                    t.runs = vec![TextRun { text: String::new(), style: style.clone(), inline: None }];
                    t.set_all_paras(first_para.clone());
                    refresh_bounds(t);
                }
            }
            let end_style = story.last().map_or_else(|| style.clone(), |r| r.style.clone());
            let last_para = paras.last().cloned().unwrap_or_default();
            for id in keep.iter().skip(1) {
                if let Some(NodeKind::Text(t)) = d.node_mut(*id).map(|n| &mut n.kind) {
                    t.runs = vec![TextRun { text: String::new(), style: end_style.clone(), inline: None }];
                    t.set_all_paras(last_para.clone());
                }
            }
            if let Some(first) = keep.first()
                && let Some(NodeKind::Text(t)) = d.node_mut(*first).map(|n| &mut n.kind)
            {
                vectnor_text::edit::normalize(&mut story);
                t.runs = story;
                t.set_paragraph_styles(paras);
            }
            let i = d.text_threads.iter().position(|t| *t == thread).unwrap_or(0);
            d.text_threads[i] = keep;
        }
        d.text_threads.retain(|t| t.len() >= 2);
        Ok(())
    })?;
    ok()
}

fn remove(s: &mut Session, _: &Value) -> Result<Value> {
    let ids = selected_roots(s)?;
    s.edit("Remove Threading", |d, _| {
        d.text_threads.retain(|t| !t.iter().any(|i| ids.contains(i)));
        Ok(())
    })?;
    ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "The quick brown fox jumps over the lazy dog and keeps running far beyond the first frame of this story. ";

    fn setup() -> (Session, u64, u64) {
        let mut s = Session::new();
        s.execute("file.new", &json!({"width": 600, "height": 300})).unwrap();
        let a = s.execute("text.create", &json!({"x": 10, "y": 10, "text": TEXT.repeat(4), "area": {"width": 150, "height": 60}})).unwrap()["id"]
            .as_u64()
            .unwrap();
        let b = s.execute("shape.rectangle", &json!({"x": 200, "y": 10, "width": 150, "height": 200})).unwrap()["id"].as_u64().unwrap();
        s.execute("select.set", &json!({"ids": [a, b]})).unwrap();
        (s, a, b)
    }

    fn text(s: &Session, id: u64) -> String {
        match &s.doc().unwrap().doc.node(NodeId(id)).unwrap().kind {
            NodeKind::Text(t) => t.plain_text(),
            _ => panic!("not text"),
        }
    }

    #[test]
    fn thread_flows_reflows_on_edit_and_survives_delete() {
        let (mut s, a, b) = setup();
        s.execute("text.thread.create", &json!({})).unwrap();
        let (ta, tb) = (text(&s, a), text(&s, b));
        assert!(!ta.is_empty() && !tb.is_empty());
        assert_eq!(format!("{ta}{tb}"), TEXT.repeat(4));
        // Editing the first frame re-flows into the second.
        s.execute("text.editRange", &json!({"id": a, "start": 0, "end": 0, "insert": "NEW "})).unwrap();
        assert_eq!(format!("{}{}", text(&s, a), text(&s, b)), format!("NEW {}", TEXT.repeat(4)));
        assert_ne!(text(&s, b), tb, "the overflow moved on");
        // Deleting the first frame moves all its text into the second.
        s.execute("select.set", &json!({"ids": [a]})).unwrap();
        s.execute("edit.clear", &json!({})).unwrap();
        assert_eq!(text(&s, b), format!("NEW {}", TEXT.repeat(4)));
        assert!(s.doc().unwrap().doc.text_threads.is_empty(), "a one-frame thread is no thread");
        s.execute("edit.undo", &json!({})).unwrap();
        assert_eq!(s.doc().unwrap().doc.text_threads.len(), 1);
    }

    /// A thread of `n` frames (2 or 3), each showing part of the story: area type `a` of [`setup`],
    /// for 3 an area type frame that is empty until it is threaded, then the rectangle `b`.
    fn thread_of(n: usize) -> (Session, Vec<u64>) {
        let (mut s, a, b) = setup();
        let mut frames = vec![a, b];
        if n == 3 {
            // Keep all three frames occupied even with paint-order-based thread creation.
            s.execute("select.set", &json!({"ids": [b]})).unwrap();
            s.execute("object.setBounds", &json!({"height": 40})).unwrap();
            let c = s.execute("text.create", &json!({"x": 10, "y": 100, "text": "", "area": {"width": 150, "height": 40}})).unwrap()["id"]
                .as_u64()
                .unwrap();
            s.execute("select.set", &json!({"ids": [a, c]})).unwrap();
            s.execute("text.thread.create", &json!({})).unwrap();
            frames = vec![c, b];
        }
        s.execute("select.set", &json!({"ids": frames})).unwrap();
        s.execute("text.thread.create", &json!({})).unwrap();
        let frames: Vec<u64> = s.doc().unwrap().doc.text_threads[0].iter().map(|id| id.0).collect();
        assert!(frames.iter().all(|id| !text(&s, *id).is_empty()), "each frame shows part of the story");
        (s, frames)
    }

    /// Convert To Point Type on a frame of threaded text takes it out of its thread with the text
    /// it holds, and the other frames keep the text they hold: the story holds each character once.
    /// A thread left with one frame is no thread. Undo threads the frame again.
    #[test]
    fn a_frame_converted_to_point_type_leaves_its_thread_with_its_text() {
        // Soft wraps of the point type become line breaks, so its characters are compared without
        // white space.
        let squeeze = |t: &str| t.split_whitespace().collect::<String>();
        for n in [2, 3] {
            for k in 0..n {
                let (mut s, frames) = thread_of(n);
                let shown: Vec<String> = frames.iter().map(|id| text(&s, *id)).collect();
                let id = frames[k];
                s.execute("type.convertToPointType", &json!({"ids": [id]})).unwrap();
                let NodeKind::Text(t) = &s.doc().unwrap().doc.node(NodeId(id)).unwrap().kind else { panic!("not text") };
                assert!(matches!(t.kind, TextKind::Point), "{n} frames, frame {k}: point type");
                assert_eq!(squeeze(&text(&s, id)), squeeze(&shown[k]), "{n} frames, frame {k}: the point type keeps its text");
                for (f, before) in frames.iter().zip(&shown).filter(|(f, _)| **f != id) {
                    assert_eq!(text(&s, *f), *before, "{n} frames, frame {k}: frame {f} keeps its text");
                }
                let rest: Vec<u64> = frames.iter().copied().filter(|f| *f != id).collect();
                assert_eq!(
                    s.doc().unwrap().doc.text_threads.iter().map(|t| t.iter().map(|id| id.0).collect::<Vec<_>>()).collect::<Vec<_>>(),
                    if rest.len() >= 2 { vec![rest] } else { vec![] },
                    "{n} frames, frame {k}"
                );
                let all: String = frames.iter().map(|f| text(&s, *f)).collect();
                assert_eq!(squeeze(&all), squeeze(&TEXT.repeat(4)), "{n} frames, frame {k}: each character once");
                s.execute("edit.undo", &json!({})).unwrap();
                assert_eq!(
                    s.doc().unwrap().doc.text_threads.iter().map(|t| t.iter().map(|id| id.0).collect::<Vec<_>>()).collect::<Vec<_>>(),
                    std::slice::from_ref(&frames)
                );
                assert_eq!(frames.iter().map(|f| text(&s, *f)).collect::<Vec<_>>(), shown, "{n} frames, frame {k}: undo");
            }
        }
    }

    #[test]
    fn release_and_remove() {
        let (mut s, a, b) = setup();
        s.execute("text.thread.create", &json!({})).unwrap();
        let tb = text(&s, b);
        s.execute("select.set", &json!({"ids": [b]})).unwrap();
        s.execute("text.thread.remove", &json!({})).unwrap();
        assert_eq!(text(&s, b), tb, "remove keeps what each frame shows");
        assert!(s.doc().unwrap().doc.text_threads.is_empty());
        s.execute("edit.undo", &json!({})).unwrap();
        s.execute("select.set", &json!({"ids": [b]})).unwrap();
        s.execute("text.thread.releaseSelection", &json!({})).unwrap();
        assert_eq!(text(&s, b), "");
        assert_eq!(text(&s, a), TEXT.repeat(4), "the story returns to the remaining frame");
    }

    #[test]
    fn release_from_a_longer_thread_keeps_the_story_once() {
        let (mut s, a, b) = setup();
        let c = s.execute("text.create", &json!({"x":10,"y":100,"text":"","area":{"width":150,"height":40}})).unwrap()["id"].as_u64().unwrap();
        s.execute("select.set", &json!({"ids":[a,b,c]})).unwrap();
        s.execute("text.thread.create", &json!({})).unwrap();
        let thread = s.doc().unwrap().doc.text_threads[0].clone();
        let middle = thread[1].0;
        s.execute("select.set", &json!({"ids":[middle]})).unwrap();
        s.execute("text.thread.releaseSelection", &json!({})).unwrap();
        assert!(text(&s, middle).is_empty());
        let story: String = s.doc().unwrap().doc.text_threads[0].iter().map(|id| text(&s, id.0)).collect();
        assert_eq!(story, TEXT.repeat(4));
        s.execute("edit.undo", &json!({})).unwrap();
        assert_eq!(s.doc().unwrap().doc.text_threads[0], thread);
        s.execute("edit.redo", &json!({})).unwrap();
        assert!(text(&s, middle).is_empty());
    }
}
