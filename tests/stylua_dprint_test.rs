use dprint_plugin_stylua::{Configuration as StyLuaConfig, StyLuaPluginHandler};
use stylua_lib::QuoteStyle;

#[test]
fn stylua_formats_lua() {
  let mut handler = StyLuaPluginHandler::default();
  let config = StyLuaConfig { quote_style: QuoteStyle::AutoPreferDouble, ..Default::default() };
  let request = dprint_core::plugins::SyncFormatRequest {
    file_path: "sample.lua".into(),
    file_bytes: b"local   x   =    1".to_vec(),
    range: None,
    config,
    config_id: None,
    token: vec![],
  };
  let result = handler.format(request, |_req| Ok(None)).expect("stylua should run");
  assert!(result.is_some());
}
