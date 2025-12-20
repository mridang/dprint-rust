use dprint_plugin_mago::MagoPluginHandler;
use dprint_plugin_rumdl::RumdlPluginHandler;
use dprint_plugin_stylua::StyLuaPluginHandler;

#[test]
fn plugin_info_config_keys_are_set() {
  let mut mago = MagoPluginHandler::default();
  let mut stylua = StyLuaPluginHandler::default();
  let mut rumdl = RumdlPluginHandler::default();

  assert_eq!(mago.plugin_info().config_key, "mago");
  assert_eq!(stylua.plugin_info().config_key, "stylua");
  assert_eq!(rumdl.plugin_info().config_key, "rumdl");
}
