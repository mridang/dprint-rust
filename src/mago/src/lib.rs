use anyhow::{anyhow, Result};
use bumpalo::Bump;
use dprint_core::configuration::{
    get_unknown_property_diagnostics, get_value, ConfigKeyMap, GlobalConfiguration,
};
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use dprint_core::generate_plugin_code;
use dprint_core::plugins::{
    CheckConfigUpdatesMessage, ConfigChange, FileMatchingInfo, FormatResult, PluginInfo,
    PluginResolveConfigurationResult, SyncFormatRequest, SyncHostFormatRequest, SyncPluginHandler,
};
use mago_formatter::{settings::FormatSettings, Formatter};
use mago_php_version::PHPVersion;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub line_width: u32,
    pub indent_width: u32,
    pub use_tabs: bool,
}

#[derive(Default)]
pub struct MagoPluginHandler;

impl SyncPluginHandler<Configuration> for MagoPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        let version = env!("CARGO_PKG_VERSION").to_string();
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: version.clone(),
            config_key: "mago".to_string(),
            help_url: "https://github.com/carthage-software/mago".to_string(),
            config_schema_url: format!(
                "https://plugins.dprint.dev/mridang/dprint-plugin-rusty/{}/mago-schema.json",
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
            global_config.indent_width.unwrap_or(4),
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
                file_extensions: vec!["php".to_string(), "phtml".to_string()],
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

        let text = String::from_utf8_lossy(&request.file_bytes);
        let arena = Bump::new();

        let settings = FormatSettings {
            print_width: request.config.line_width as usize,
            tab_width: request.config.indent_width as usize,
            use_tabs: request.config.use_tabs,
            ..Default::default()
        };

        let formatter = Formatter::new(&arena, PHPVersion::LATEST, settings);
        let file_name = request.file_path.to_string_lossy().into_owned();
        let formatted = formatter
            .format_code(Cow::Owned(file_name), Cow::Owned(text.to_string()))
            .map_err(|err| anyhow!("mago parse error: {err}"))?;

        Ok(Some(formatted.as_bytes().to_vec()))
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
dprint_core::generate_plugin_code!(MagoPluginHandler, MagoPluginHandler);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_php_without_error() {
        let arena = Bump::new();
        let formatter = Formatter::new(
            &arena,
            PHPVersion::PHP81,
            FormatSettings {
                print_width: 80,
                ..Default::default()
            },
        );
        let formatted = formatter
            .format_code(
                Cow::Borrowed("sample.php"),
                Cow::Owned("<?php class Test{function hi(){echo\"hi\";}}".to_string()),
            )
            .expect("formatter should succeed");
        assert!(!formatted.trim().is_empty());
        assert!(formatted.contains("function hi"));
    }
}
