use super::*;
use serde_json::json;

#[test]
fn english_labels_and_unicode_names_are_preserved() {
    assert_eq!(Lang::all().collect::<Vec<_>>(), [Lang::EN]);
    assert_eq!(Lang::EN.code(), "en");
    for label in ["File", "Appearance", "తెలుగు", "हिन्दी", "বাংলা", "தமிழ்"] {
        assert_eq!(t(label), label);
        assert_eq!(tr_ctx(Lang::EN, "axis", label), label);
        assert_eq!(tr_id(Lang::EN, "object.group", label), label);
        assert_eq!(label_or_name(Lang::EN, label, false), label);
        assert_eq!(msg(label), label);
    }
    assert_eq!(tn(0, "{n} file", "{n} files"), "0 files");
    assert_eq!(tn(1, "{n} file", "{n} files"), "1 file");
    assert_eq!(tn(2, "{n} file", "{n} files"), "2 files");
    assert_eq!(fmt("Open {name}: {unknown}", &[("name", "తెలుగు.vcnr")]), "Open తెలుగు.vcnr: {unknown}");
}

#[test]
fn obsolete_language_preferences_are_ignored_without_losing_other_settings() {
    for code in ["ja", "fr", "hi", "auto"] {
        let mut app = crate::VectnorApp::new(vectnor_engine::Session::new(), Default::default());
        app.ui = serde_json::from_value(json!({"language": code, "engine_prefs": {"interfaceLanguage": code, "uiScaling": 1.25}})).unwrap();
        crate::prefs_dialog::restore(&mut app);
        assert_eq!(app.ui_language(), Lang::EN);
        assert_eq!(app.session.prefs.ui_scaling, 1.25);
        crate::prefs_dialog::snapshot(&mut app);
        let saved = serde_json::to_value(&app.ui).unwrap();
        assert!(saved.get("language").is_none());
        assert!(saved["engine_prefs"].get("interfaceLanguage").is_none());
        assert!(app.run("prefs.set", json!({"key": "interfaceLanguage", "value": code})).is_err());
    }
    assert!(!crate::menus::UI_COMMANDS.iter().any(|(id, _, _, _)| *id == "app.language"));
}
