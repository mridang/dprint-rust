use anyhow::{anyhow, Result};
use dprint_core::configuration::{
    get_unknown_property_diagnostics, get_value, ConfigKeyMap, GlobalConfiguration,
};
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use dprint_core::generate_plugin_code;
use dprint_core::plugins::{
    CheckConfigUpdatesMessage, ConfigChange, FileMatchingInfo, FormatResult, PluginInfo,
    PluginResolveConfigurationResult, SyncFormatRequest, SyncHostFormatRequest, SyncPluginHandler,
};
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub line_width: u32,
    pub indent_width: u32,
    pub use_tabs: bool,
}

#[derive(Default)]
pub struct AlejandraPluginHandler;

impl SyncPluginHandler<Configuration> for AlejandraPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        let version = env!("CARGO_PKG_VERSION").to_string();
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: version.clone(),
            config_key: "alejandra".to_string(),
            help_url: "https://github.com/kamadorueda/alejandra".to_string(),
            config_schema_url: format!(
                "https://plugins.dprint.dev/mridang/dprint-plugin-rusty/{}/alejandra-schema.json",
                version
            ),
            update_url: None,
        }
    }

    fn license_text(&mut self) -> String {
        std::str::from_utf8(include_bytes!("../../../LICENSE"))
            .unwrap_or("License not available")
            .to_string()
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<Configuration> {
        let mut config = config;
        let mut diagnostics = Vec::new();

        let line_width = get_value(
            &mut config,
            "lineWidth",
            global_config.line_width.unwrap_or(120),
            &mut diagnostics,
        );
        let indent_width = get_value(
            &mut config,
            "indentWidth",
            global_config.indent_width.unwrap_or(2),
            &mut diagnostics,
        );
        let use_tabs = get_value(
            &mut config,
            "useTabs",
            global_config.use_tabs.unwrap_or(false),
            &mut diagnostics,
        );

        diagnostics.extend(get_unknown_property_diagnostics(config));

        PluginResolveConfigurationResult {
            config: Configuration {
                line_width,
                indent_width: indent_width as u32,
                use_tabs,
            },
            diagnostics,
            file_matching: FileMatchingInfo {
                file_extensions: vec!["nix".to_string()],
                file_names: vec![],
            },
        }
    }

    fn check_config_updates(
        &self,
        _message: CheckConfigUpdatesMessage,
    ) -> Result<Vec<ConfigChange>> {
        Ok(vec![])
    }

    fn format(
        &mut self,
        request: SyncFormatRequest<Configuration>,
        _format_with_host: impl FnMut(SyncHostFormatRequest) -> FormatResult,
    ) -> FormatResult {
        if request.range.is_some() {
            return Ok(None);
        }

        let content = String::from_utf8_lossy(&request.file_bytes).to_string();
        let path = request
            .file_path
            .to_str()
            .unwrap_or("input.nix")
            .to_string();

        let (status, formatted) = alejandra::format::in_memory(path, content.clone());

        match status {
            alejandra::format::Status::Error(err) => Err(anyhow!("alejandra error: {}", err)),
            alejandra::format::Status::Changed(changed) => {
                if changed {
                    Ok(Some(formatted.into_bytes()))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
dprint_core::generate_plugin_code!(AlejandraPluginHandler, AlejandraPluginHandler);

#[cfg(test)]
mod tests {
    use alejandra::format::{in_memory, Status};

    #[test]
    fn formats_nix_snippet() {
        let content = "{   foo   =   1;   }";
        let (status, formatted) = in_memory("test.nix".to_string(), content.to_string());
        match status {
            Status::Error(e) => panic!("alejandra error: {}", e),
            Status::Changed(_) => {
                assert_eq!(formatted, "{foo = 1;}\n");
            }
        }
    }
}
