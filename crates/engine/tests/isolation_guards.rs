//! Isolation refusals preserve the current scope and selection.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use serde_json::json;
use vectnor_doc::NodeId;
use vectnor_engine::Session;

fn scene() -> (Session, NodeId, NodeId) {
    let mut s = Session::new();
    s.execute("file.new", &json!({"width":400,"height":300})).unwrap();
    let a = s.execute("shape.rectangle", &json!({"x":10,"y":10,"width":30,"height":30})).unwrap()["id"].as_u64().unwrap();
    let b = s.execute("shape.rectangle", &json!({"x":70,"y":10,"width":30,"height":30})).unwrap()["id"].as_u64().unwrap();
    s.execute("select.set", &json!({"ids":[a,b]})).unwrap();
    let group = NodeId(s.execute("object.group", &json!({})).unwrap()["id"].as_u64().unwrap());
    let layer = s.doc().unwrap().doc.layers.first().unwrap().id;
    (s, group, layer)
}

#[test]
fn hidden_or_locked_groups_and_ancestors_refuse_isolation_without_mutation() {
    for (command, target_layer, property) in [
        ("object.setProps", false, "locked"),
        ("object.setProps", false, "visible"),
        ("layer.setProps", true, "locked"),
        ("layer.setProps", true, "visible"),
    ] {
        let (mut s, group, layer) = scene();
        let target = if target_layer { layer } else { group };
        s.execute(command, &json!({"ids":[target.0],property:property=="locked"})).unwrap();
        let before = s.execute("document.inspect", &json!({})).unwrap();
        let doc = s.doc().unwrap().doc.clone();
        assert!(s.execute("object.isolate", &json!({"id":group.0})).unwrap_err().to_string().contains("hidden or locked"));
        assert_eq!(s.execute("document.inspect", &json!({})).unwrap(), before);
        assert_eq!(s.doc().unwrap().doc, doc);
    }
}

#[test]
fn template_layers_refuse_their_layer_and_groups_even_when_unlocked() {
    for isolate_layer in [false, true] {
        let (mut s, group, layer) = scene();
        s.execute("layer.setProps", &json!({"ids":[layer.0],"template":true,"locked":false})).unwrap();
        let before = s.execute("document.inspect", &json!({})).unwrap();
        let id = if isolate_layer { layer } else { group };
        assert!(s.execute("object.isolate", &json!({"id":id.0})).unwrap_err().to_string().contains("template"));
        assert_eq!(s.execute("document.inspect", &json!({})).unwrap(), before);
    }
}

#[test]
fn valid_group_still_isolates_and_a_refusal_preserves_that_scope() {
    let (mut s, group, _) = scene();
    s.execute("object.isolate", &json!({"id":group.0})).unwrap();
    assert_eq!(s.doc().unwrap().isolation, Some(group));
    let before = s.execute("document.inspect", &json!({})).unwrap();
    assert!(s.execute("object.isolate", &json!({"id":u64::MAX})).is_err());
    assert_eq!(s.execute("document.inspect", &json!({})).unwrap(), before);
}
