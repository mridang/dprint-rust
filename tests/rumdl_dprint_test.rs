use dprint_plugin_rumdl::{Configuration as RumdlConfig, RumdlPluginHandler};

#[test]
fn rumdl_formats_markdown() {
  let mut handler = RumdlPluginHandler::default();
  let config = RumdlConfig { line_length: 80, ..Default::default() };
  let request = dprint_core::plugins::SyncFormatRequest {
    file_path: "sample.md".into(),
    file_bytes: b"# Title\n\nThis  is  markdown with  bad spacing.\n".to_vec(),
    range: None,
    config,
    config_id: None,
    token: vec![],
  };
  let result = handler.format(request, |_req| Ok(None)).expect("rumdl should run");
  assert!(result.is_some());
}
