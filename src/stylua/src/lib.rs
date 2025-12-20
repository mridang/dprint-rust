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
use stylua_lib::{
    format_code, Config as StyluaConfig, IndentType, LineEndings, OutputVerification, QuoteStyle,
};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub line_width: u32,
    pub indent_width: u32,
    pub use_tabs: bool,
    pub quote_style: QuoteStyle,
}

#[derive(Default)]
pub struct StyLuaPluginHandler;

impl SyncPluginHandler<Configuration> for StyLuaPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        let version = env!("CARGO_PKG_VERSION").to_string();
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: version.clone(),
            config_key: "stylua".to_string(),
            help_url: "https://github.com/JohnnyMorganz/StyLua".to_string(),
            config_schema_url: format!(
                "https://plugins.dprint.dev/mridang/dprint-plugin-rusty/{}/stylua-schema.json",
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
        let quote_style = get_value(
            &mut config,
            "quoteStyle",
            QuoteStyle::AutoPreferDouble,
            &mut diagnostics,
        );

        diagnostics.extend(get_unknown_property_diagnostics(config));

        PluginResolveConfigurationResult {
            config: Configuration {
                line_width,
                indent_width: indent_width as u32,
                use_tabs,
                quote_style,
            },
            diagnostics,
            file_matching: FileMatchingInfo {
                file_extensions: vec!["lua".to_string()],
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

        let content = String::from_utf8_lossy(&request.file_bytes);
        let indent_type = if request.config.use_tabs {
            IndentType::Tabs
        } else {
            IndentType::Spaces
        };

        let config = StyluaConfig {
            column_width: request.config.line_width as usize,
            indent_type,
            indent_width: request.config.indent_width as usize,
            quote_style: request.config.quote_style,
            line_endings: LineEndings::Unix,
            ..Default::default()
        };

        let formatted = format_code(&content, config, None, OutputVerification::None)
            .map_err(|err| anyhow!("stylua error: {err}"))?;

        Ok(Some(formatted.into_bytes()))
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
dprint_core::generate_plugin_code!(StyLuaPluginHandler, StyLuaPluginHandler);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_lua_snippet() {
        let content = "local   x   =    1";
        let config = StyluaConfig {
            column_width: 120,
            ..Default::default()
        };
        let formatted = format_code(content, config, None, OutputVerification::None)
            .expect("stylua formats successfully");
        assert_eq!(formatted, "local x = 1\n");
    }
}
