//! Illustrator EPS and `.ai` files open from the editing data they carry: layers, groups, names,
//! hidden objects, artboards, colours and images as they were; what the reader doesn't read yet
//! opens as before, with a warning saying why.

use std::io::Write as _;
use std::sync::Arc;

use serde_json::{Value, json};
use vectnor_doc::{Node, NodeKind};
use vectnor_testkit::ai;

use super::*;

fn open(s: &mut Session, name: &str, bytes: &[u8], params: Value) -> Result<Value> {
    let mut p = json!({"name": name, "dataBase64": vectnor_format::base64_encode(bytes)});
    if let (Some(p), Some(extra)) = (p.as_object_mut(), params.as_object()) {
        p.extend(extra.clone());
    }
    s.execute("document.open", &p)
}

/// The `.ai` private data of `data`, compressed as older `.ai` files compress it.
fn compressed(data: &[u8]) -> Vec<u8> {
    let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    e.write_all(data).unwrap();
    [&b"%AI12_CompressedData"[..], &e.finish().unwrap()].concat()
}

fn names(nodes: &[Arc<Node>]) -> Vec<String> {
    nodes.iter().map(|n| n.name.clone().unwrap_or_default()).collect()
}

/// The sample's structure, as [`ai::sample_data`] builds it.
fn check_sample(s: &Session) {
    let doc = &s.doc().unwrap().doc;
    assert_eq!(names(&doc.layers), ["Art", "Hidden"]);
    assert_eq!(doc.artboards.len(), 1);
    assert_eq!(doc.artboards[0].rect, vectnor_geom::Rect::new(0.0, 0.0, 200.0, 100.0));
    let art = doc.layers[0].children().unwrap();
    assert_eq!(names(art), ["Pair", "Ring", "Window"]);
    let pair = art[0].children().unwrap();
    assert_eq!(names(pair), ["Red", ""]);
    assert!(pair[0].visible && !pair[1].visible, "the hidden member stays hidden");
    assert!(matches!(art[1].kind, NodeKind::Compound { .. }));
    assert!(matches!(&art[2].kind, NodeKind::Group { clip: true, children } if matches!(children[0].kind, NodeKind::Path { clipping: true, .. })));
    let hidden = &doc.layers[1];
    assert!(!hidden.visible);
    let kids = hidden.children().unwrap();
    assert_eq!(names(kids), ["Shaded", "", "Sub", ""]);
    assert!(matches!(kids[0].appearance.fill().map(|f| &f.paint), Some(vectnor_color::Paint::Gradient(_))));
    assert_eq!((kids[1].opacity, kids[1].blend), (0.5, vectnor_color::BlendMode::Screen));
    assert!(kids[2].locked && matches!(kids[2].kind, NodeKind::Layer { .. }));
    let NodeKind::Image(im) = &kids[3].kind else { panic!("an image") };
    assert_eq!((im.width, im.height), (2, 1));
}

#[test]
fn an_illustrator_eps_opens_with_its_layers() {
    let mut s = Session::new();
    let r = open(&mut s, "art.eps", &ai::eps(&ai::sample_data(), ai::page_ps()), json!({})).unwrap();
    assert_eq!((&r["format"], &r["warnings"]), (&json!("eps"), &json!([])), "{r}");
    check_sample(&s);
    // Saved as EPS and opened again, it comes back the same.
    let eps = s.execute("document.exportEps", &json!({})).unwrap();
    let bytes = vectnor_format::base64_decode(eps["dataBase64"].as_str().unwrap()).unwrap();
    let r = open(&mut s, "again.eps", &bytes, json!({})).unwrap();
    assert_eq!(r["restored"], json!(true), "{r}");
    check_sample(&s);
}

#[test]
fn an_illustrator_ai_file_opens_from_its_editing_data() {
    let mut s = Session::new();
    let file = ai::ai(&compressed(&ai::sample_data()), ai::page_pdf());
    let r = open(&mut s, "art.ai", &file, json!({})).unwrap();
    assert_eq!(r["format"], json!("ai"), "{r}");
    assert!(r["warnings"].as_array().unwrap().iter().all(|w| !w.as_str().unwrap().contains("weren't read")), "{r}");
    check_sample(&s);
    // Pages picked: the PDF's.
    open(&mut s, "art.ai", &file, json!({"pages": "1"})).unwrap();
    assert_ne!(names(&s.doc().unwrap().doc.layers), ["Art", "Hidden"]);
}

#[test]
fn ai_open_does_not_depend_on_the_pdf_drawing_or_its_page_size() {
    let mut s = Session::new();
    let private = compressed(&ai::sample_data());
    // A different drawing on a page with 18 pt extra on each side. The AI document, including
    // its hidden layer, sublayer and named groups, must come from the editing data alone.
    let page = "0 0 1 rg 0 0 236 136 re f";
    let file = ai::ai_with_page_size(&private, page, 236.0, 136.0);
    for name in ["bleed.ai", "template.ait"] {
        let r = open(&mut s, name, &file, json!({})).unwrap();
        assert_eq!(r["warnings"], json!([]), "{name}: {r}");
        check_sample(&s);
    }
    // Explicitly selecting the printed page still opens the PDF-sized drawing.
    for opts in [json!({"editingData": false}), json!({"layers": false}), json!({"page": 1}), json!({"cropTo": "media"})] {
        let r = open(&mut s, "bleed.ai", &file, opts.clone()).unwrap();
        let doc = &s.doc().unwrap().doc;
        assert_eq!(doc.artboards[0].rect, vectnor_geom::Rect::new(0.0, 0.0, 236.0, 136.0), "{opts}: {r}");
        assert_ne!(names(&doc.layers), ["Art", "Hidden"]);
    }
    // A regular PDF keeps the page-first behaviour even if it carries AI editing data.
    open(&mut s, "bleed.pdf", &file, json!({})).unwrap();
    assert_eq!(s.doc().unwrap().doc.artboards[0].rect.width(), 236.0);
}

#[test]
fn a_complete_ai_import_still_protects_the_original_editing_data() {
    let file = ai::ai(&compressed(&ai::sample_data()), ai::page_pdf());
    let mut s = Session::new();
    let original = "/art/complete.ai";
    let r = super::load::open_bytes(&mut s, original, &file, Some(original.into())).unwrap();
    assert_eq!(r["warnings"], json!([]));
    check_sample(&s);
    let st = s.doc().unwrap();
    assert!(st.path.is_none(), "Save must ask for a new name");
    let losses = super::load::overwrite_losses(st, original, &json!({})).unwrap();
    assert!(losses.iter().any(|l| l.contains("original AI editing data")));
    let e = s.execute("document.export", &json!({"path": original, "format": "ai"})).unwrap_err().to_string();
    assert!(e.contains("original AI editing data") && e.contains("acknowledgeLoss"), "{e}");
    assert!(super::load::overwrite_losses(s.doc().unwrap(), "/art/copy.ai", &json!({})).is_none());
    assert!(super::load::overwrite_losses(s.doc().unwrap(), original, &json!({"acknowledgeLoss": true})).is_none());
}

#[test]
fn what_the_reader_doesnt_read_opens_as_before_with_a_warning() {
    let symbols = ai::editing_data(200.0, 100.0, &ai::layer("Art", "/SymbolInstance :\n(Dot) /SymbolRef ,\n;\n"));
    let mut s = Session::new();
    let r = open(&mut s, "symbols.eps", &ai::eps(symbols.as_bytes(), ai::page_ps()), json!({})).unwrap();
    assert!(r["warnings"].to_string().contains("layers weren't read") && r["warnings"].to_string().contains("symbols"), "{r}");
    assert_eq!(names(&s.doc().unwrap().doc.layers), ["Layer 1"]);
    let r = open(&mut s, "symbols.ai", &ai::ai(&compressed(symbols.as_bytes()), ai::page_pdf()), json!({})).unwrap();
    assert!(r["warnings"].to_string().contains("symbols") && r["warnings"].to_string().contains("PDF part"), "{r}");
}

/// A page showing only a placeholder line, as a `.ai` saved without PDF compatibility has.
const PLACEHOLDER: &str = "BT /F1 12 Tf 10 50 Td (Saved without its PDF part) Tj ET";

#[test]
fn an_ai_file_saved_without_its_pdf_part_opens_from_its_editing_data() {
    let mut s = Session::new();
    let r = open(&mut s, "art.ai", &ai::ai(&compressed(&ai::sample_data()), PLACEHOLDER), json!({})).unwrap();
    assert_eq!(r["format"], json!("ai"), "{r}");
    check_sample(&s);
    // Its editing data damaged: it says so (the placeholder isn't the art).
    let e = open(&mut s, "damaged.ai", &ai::ai(b"%AI12_CompressedData not zlib", PLACEHOLDER), json!({})).unwrap_err().to_string();
    assert!(e.contains("without PDF compatibility") && e.contains("zlib"), "{e}");
    // Without editing data, the placeholder still isn't opened as the art.
    let e = open(&mut s, "plain.ai", &ai::ai(b"", PLACEHOLDER), json!({})).unwrap_err().to_string();
    assert!(e.contains("placeholder"), "{e}");
}

#[test]
fn editing_data_false_opens_only_what_the_page_prints() {
    let mut s = Session::new();
    for (name, file) in [("art.eps", ai::eps(&ai::sample_data(), ai::page_ps())), ("art.ai", ai::ai(&compressed(&ai::sample_data()), ai::page_pdf()))]
    {
        let r = open(&mut s, name, &file, json!({"editingData": false})).unwrap();
        let layers = &s.doc().unwrap().doc.layers;
        assert_eq!(layers.len(), 1, "{name}: {r}");
        assert_ne!(names(layers), ["Art"], "{name}: the page's layer");
    }
    let e = open(&mut s, "art.eps", &ai::eps(&ai::sample_data(), ai::page_ps()), json!({"editingData": "no"})).unwrap_err().to_string();
    assert!(e.contains("editingData must be true or false"), "{e}");
    // A file that is only its editing data can't open without it.
    let e = open(&mut s, "art.ai", &ai::ai(&compressed(&ai::sample_data()), PLACEHOLDER), json!({"editingData": false})).unwrap_err().to_string();
    assert!(e.contains("placeholder"), "{e}");
}

#[test]
fn fit_to_artwork_bounds_leaves_out_the_hidden_art_and_guides_the_editing_data_has() {
    // A shown square; a hidden one and a long guide far from it, which the page doesn't draw.
    let art =
        format!("0 g\n{}f\n1 Xw\n{}f\n0 Xw\n-500 20 m\n700 20 L\n(N) *\n", ai::rect(10.0, 10.0, 50.0, 50.0), ai::rect(150.0, 60.0, 190.0, 95.0));
    let data = ai::editing_data(200.0, 100.0, &ai::layer("Art", &art));
    let mut s = Session::new();
    let r = open(&mut s, "fit.eps", &ai::eps(data.as_bytes(), "0 setgray 10 10 40 40 rectfill\n"), json!({})).unwrap();
    let layer = &s.doc().unwrap().doc.layers[0];
    let kids = layer.children().unwrap();
    assert_eq!(kids.len(), 3, "{r}");
    assert!(!kids[1].visible && matches!(kids[2].kind, NodeKind::Path { guide: true, .. }));
    s.execute("artboard.fitToArt", &json!({})).unwrap();
    // The square, 50 to 90 down the 100-point page.
    assert_eq!(s.doc().unwrap().doc.artboards[0].rect, vectnor_geom::Rect::new(10.0, 50.0, 50.0, 90.0));
}

#[test]
fn type_asked_for_as_outlines_opens_a_file_whose_type_shows_from_its_pdf_part() {
    let text = "/AI11Text :\n0 /FreeUndo ,\n0 /FrameIndex ,\n0 /StoryIndex ,\n;\n";
    let data = ai::editing_data(200.0, 100.0, &ai::layer("Art", &format!("0 g\n{}f\n{text}", ai::rect(10.0, 10.0, 50.0, 50.0))));
    let file = ai::ai(&compressed(data.as_bytes()), "0 g 10 10 40 40 re f BT /F1 12 Tf 100 50 Td (Hi) Tj ET");
    let mut s = Session::new();
    // As type, the page's type goes into the text object of the layers.
    let r = open(&mut s, "type.ai", &file, json!({})).unwrap();
    assert_eq!(names(&s.doc().unwrap().doc.layers), ["Art"], "{r}");
    // As outlines, only the PDF part has them.
    let r = open(&mut s, "type.ai", &file, json!({"textAs": "outlines"})).unwrap();
    assert!(r["warnings"].to_string().contains("outlines"), "{r}");
    assert_ne!(names(&s.doc().unwrap().doc.layers), ["Art"], "{r}");
}

/// Original synthetic area type: one frame, or a story linked through two frames.
fn area_text_ai(first_height: f64, second_height: Option<f64>) -> (String, Vec<u8>) {
    fn frame(y: f64, height: f64) -> String {
        let (x0, x1, y0, y1) = (8151.5, 8231.5, 8141.5 + y, 8141.5 + y + height);
        format!(
            "<< /0 << /1 << /0 [ {x0} {y0} {x0} {y0} {x0} {y1} {x0} {y1} {x0} {y1} {x0} {y1} {x1} {y1} {x1} {y1} {x1} {y1} {x1} {y1} {x1} {y0} {x1} {y0} {x1} {y0} {x1} {y0} {x0} {y0} {x0} {y0} ] >> /2 << /0 1 >> >> >>"
        )
    }
    let mut frames = vec![frame(10.0, first_height)];
    let mut objects = "/AI11Text : 0 /StoryIndex , 0 /FrameIndex , ;\n".to_string();
    let mut refs = "<< /0 0 >>".to_string();
    if let Some(height) = second_height {
        frames.push(frame(20.0, height));
        objects.push_str("/AI11Text : 0 /StoryIndex , 1 /FrameIndex , ;\n");
        refs.push_str(" << /0 1 >>");
    }
    let text = format!(
        "/0 << /1 << /0 [ << /0 << /0 << /0 (Helvetica) >> >> >> ] >> /8 << /0 [ {} ] >> >> /1 << /1 [ << /0 << /0 (Sample text) >> /1 << /0 [ {refs} ] >> >> ] /2 << /1 14 >> >>",
        frames.join(" ")
    );
    let mut encoded = String::new();
    for chunk in text.as_bytes().chunks(4) {
        let mut number = chunk.iter().enumerate().fold(0u32, |n, (i, b)| n | u32::from(*b) << (24 - 8 * i));
        let mut digits = [0u8; 5];
        for d in digits.iter_mut().rev() {
            *d = (number % 85) as u8 + b'!';
            number /= 85;
        }
        encoded.extend(digits.iter().take(chunk.len() + 1).map(|d| char::from(*d)));
    }
    encoded.push_str("~>");
    let body =
        format!("{}%AI11_BeginTextDocument\n/AI11TextDocument : /ASCII85Decode ,\n{encoded}\n%AI11_EndTextDocument\n", ai::layer("Text", &objects));
    let data = ai::editing_data(100.0, 100.0, &body).replace("%%EndComments", "%AI3_TemplateBox: 50 50 50 50\n%%EndComments");
    let file = ai::ai_with_page_size(&compressed(data.as_bytes()), "0 g 0 0 2 2 re f", 100.0, 100.0);
    (data, file)
}

#[test]
fn ai_overflow_warning_is_general_and_keeps_original_text_and_frame() {
    let (data, file) = area_text_ai(4.0, None);
    let expected = vectnor_eps::ai_alone(data.as_bytes()).unwrap().0;
    for name in ["poster.ai", "unrelated.AI", "template.ait"] {
        let loaded = super::load::load(name, &file).unwrap();
        assert!(loaded.warnings.first().is_some_and(|w| w.starts_with("1 imported text frame has overflow text")), "{name}: {:?}", loaded.warnings);
        assert_eq!(loaded.doc.layers, expected.layers, "diagnostic preserves text, styles and geometry");
    }
    let raw = super::load::load("legacy-container.ai", data.as_bytes()).unwrap();
    assert!(raw.warnings.iter().any(|w| w.starts_with("1 imported text frame has overflow text")));
}

#[test]
fn ai_fitting_text_and_flow_into_the_next_linked_frame_do_not_warn() {
    for (first, second) in [(80.0, None), (4.0, Some(80.0))] {
        let (_, file) = area_text_ai(first, second);
        let loaded = super::load::load("different.ai", &file).unwrap();
        assert!(!loaded.warnings.iter().any(|w| w.contains("overflow text")), "{:?}", loaded.warnings);
        if second.is_some() {
            assert_eq!(loaded.doc.text_threads.len(), 1);
        }
    }
    let (_, file) = area_text_ai(4.0, Some(4.0));
    let loaded = super::load::load("linked.ai", &file).unwrap();
    assert!(
        loaded.warnings.iter().any(|w| w.starts_with("1 imported text frame has overflow text")),
        "only the last frame retains overset text: {:?}",
        loaded.warnings
    );
}
