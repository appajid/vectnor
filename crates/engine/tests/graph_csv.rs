//! Quoted graph labels stay intact across CSV parsing and command undo/redo.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use proptest::prelude::*;
use serde_json::json;
use vectnor_engine::Session;

proptest! {
    #![proptest_config(ProptestConfig { cases: 40, failure_persistence: None, ..ProptestConfig::default() })]
    #[test]
    fn csv_label_roundtrip(label in prop::collection::vec(prop::sample::select(vec!['a', ',', '"', '\n', '\t', ' ', 'क', 'ష', '1', '&']), 0..24)) {
        let label:String=label.into_iter().collect();
        let series=json!([format!("Series {label}"),"Other"]);
        let categories=json!([format!("Category {label}"),"2026"," padded "]);
        let mut s=Session::new();
        s.execute("file.new",&json!({"width":400,"height":300})).unwrap();
        let id=s.execute("graph.create",&json!({"type":"column","x":20,"y":20,"width":300,"height":200,"series":series,"categories":categories,"rows":[[1.23456789,2],[3,null],[4,-5]]})).unwrap()["id"].clone();
        let before=s.execute("graph.setData",&json!({"id":id})).unwrap();
        s.execute("graph.setData",&json!({"id":id,"csv":before["csv"]})).unwrap();
        let after=s.execute("graph.setData",&json!({"id":id})).unwrap();
        prop_assert_eq!(&after["series"],&before["series"]);
        prop_assert_eq!(&after["categories"],&before["categories"]);
        prop_assert_eq!(&after["rows"],&before["rows"]);
        s.execute("edit.undo",&json!({})).unwrap();
        s.execute("edit.redo",&json!({})).unwrap();
        prop_assert_eq!(s.execute("graph.setData",&json!({"id":id})).unwrap(),after);
    }
}
