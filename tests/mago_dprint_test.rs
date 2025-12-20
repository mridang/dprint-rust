use dprint_plugin_mago::MagoPluginHandler;

#[test]
fn mago_formats_php() {
  let mut handler = MagoPluginHandler::default();
  let resolved = handler.resolve_config(Default::default(), &Default::default());
  let result = handler
    .format(
      dprint_core::plugins::SyncFormatRequest {
        file_path: "sample.php".into(),
        file_bytes: b"<?php class Test{function hi(){echo\"hi\";}}".to_vec(),
        range: None,
        config: resolved.config,
        config_id: None,
        token: vec![],
      },
      |_req| Ok(None),
    )
    .expect("mago should run");
  assert!(result.is_some());
}
