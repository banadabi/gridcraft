//! Exercise rendered controls, not only the translation lookup tables: a translated catalog
//! must actually reach the dialogs, while field identifiers and workbook data stay canonical.

use egui_kittest::kittest::Queryable;
use gridcraft_ui_egui::{SheetApp, i18n::Language};
use serde_json::json;

fn harness() -> egui_kittest::Harness<'static, SheetApp> {
    let mut h = crate::support::blank_harness(6);
    h.state_mut().ui.language = Language::Tr;
    h.run_steps(6);
    h
}

#[test]
fn turkish_replace_controls_edit_text_without_translating_workbook_data() {
    let mut h = harness();
    h.state_mut().run("cell.set", json!({"cell": "A1", "input": "İstanbul ığüşöç"})).unwrap();
    h.state_mut().open_dialog("find", json!({}));
    let d = h.state_mut().dialog.as_mut().unwrap();
    d.tab = "Replace".into();
    d.values.insert("what".into(), json!("İstanbul"));
    d.values.insert("with".into(), json!("İzmir"));
    h.run_steps(6);
    assert!(h.query_by_label("Büyük/küçük harf eşleştir").is_some());
    assert!(h.query_by_label("Çalışma kitabında").is_some());
    h.get_by_label("Tümünü Değiştir").click();
    h.run_steps(6);
    let doc = h.state().session.active().unwrap();
    assert_eq!(doc.wb.active().unwrap().value(gridcraft_engine::core::CellRef::new(0, 0)).display(), "İzmir ığüşöç");
    assert_eq!(h.state().toast.as_ref().unwrap().0, "İşlem tamamlandı. 1 değişiklik yapıldı.");
    assert_eq!(h.state().dialog.as_ref().unwrap().tab, "Replace");
}

#[test]
fn turkish_save_prompt_keeps_the_document_title_and_cancel_preserves_edits() {
    let mut h = harness();
    h.state_mut().run("cell.set", json!({"cell": "A1", "input": "Kaydedilmemiş çalışma"})).unwrap();
    h.state_mut().open_dialog("saveChanges", json!({"title": "İstanbul {count}.xlsx"}));
    h.run_steps(6);
    assert!(h.query_by_label("İstanbul {count}.xlsx üzerinde yaptığınız değişiklikleri kaydetmek istiyor musunuz?").is_some());
    assert!(h.query_by_label("Kaydetme").is_some());
    // The formula bar has its own Cancel; the dialog's is drawn last.
    h.get_all_by_role_and_label(egui::accesskit::Role::Button, "İptal").last().unwrap().click();
    h.run_steps(6);
    assert!(h.state().dialog.is_none());
    assert!(h.state().session.active().unwrap().is_dirty());
}

#[test]
fn turkish_format_cells_dialog_translates_tabs_without_changing_internal_tab_ids() {
    let mut h = harness();
    h.state_mut().open_dialog("formatCells", json!({}));
    h.run_steps(6);
    h.get_by_label("Dolgu").click();
    h.run_steps(6);
    assert_eq!(h.state().dialog.as_ref().unwrap().tab, "Fill");
    assert!(h.query_by_label("Arka Plan Rengi:").is_some());
    assert!(h.query_by_label("   Tamam   ").is_some());
}

#[test]
fn command_search_accepts_turkish_labels_and_canonical_command_ids() {
    let mut h = harness();
    h.state_mut().open_dialog("commandSearch", json!({}));
    for query in ["kaydet", "file.save", "FILE.SAVE", "save"] {
        h.state_mut().dialog.as_mut().unwrap().search = query.into();
        h.run_steps(6);
        assert!(h.query_by_label("Kaydet    Dosya").is_some(), "missing Save result for {query}");
    }
    // Turkish folding against a translated label: plain lowercasing turns SIRALA into "sirala".
    h.state_mut().dialog.as_mut().unwrap().search = "SIRALA".into();
    h.run_steps(6);
    assert!(h.query_by_label("A'dan Z'ye Sırala    Veri › Sırala ve Filtrele").is_some());
}

#[test]
fn turkish_chart_preview_heading_matches_the_selected_list_entry() {
    let mut h = harness();
    h.state_mut().run("range.setValues", json!({"range": "A1", "values": [["Quarter", "North"], ["Q1", 10], ["Q2", 30]]})).unwrap();
    h.state_mut().run("selection.set", json!({"range": "A1:B3"})).unwrap();
    h.state_mut().open_dialog("recommendedCharts", json!({"tab": "All Charts"}));
    h.run_steps(6);
    assert_eq!(h.query_all_by_label("Kümelenmiş Sütun").count(), 2, "list entry and preview heading");
    assert!(h.query_by_label("Clustered Column").is_none());
}

#[test]
fn turkish_protect_sheet_lists_every_permission_in_turkish() {
    let mut h = harness();
    h.state_mut().open_dialog("protectSheet", json!({}));
    h.run_steps(6);
    for label in ["Kilitli hücreleri seç", "Kilidi açık hücreleri seç", "Sütun ekle", "Sütun sil", "Satır ekle", "Satır sil"] {
        assert!(h.query_by_label(label).is_some(), "missing {label}");
    }
}

#[test]
fn turkish_solver_adds_constraints_with_ekle_not_arithmetic_topla() {
    let mut h = harness();
    // The ribbon's Insert tab is also "Ekle": the dialog must add exactly one more.
    let before = h.query_all_by_label("Ekle").count();
    h.state_mut().open_dialog("solver", json!({}));
    h.run_steps(6);
    assert_eq!(h.query_all_by_label("Ekle").count(), before + 1);
    assert!(h.query_by_label("Topla").is_none());
}
