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
use rumdl_lib::config::{Config as RumdlConfig, MarkdownFlavor};
use rumdl_lib::fix_coordinator::FixCoordinator;
use rumdl_lib::rules::{all_rules, filter_rules};
use rumdl_lib::types::LineLength;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub line_length: u32,
    pub flavor: MarkdownFlavor,
}

#[derive(Default)]
pub struct RumdlPluginHandler;

impl SyncPluginHandler<Configuration> for RumdlPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        let version = env!("CARGO_PKG_VERSION").to_string();
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: version.clone(),
            config_key: "rumdl".to_string(),
            help_url: "https://github.com/rvben/rumdl".to_string(),
            config_schema_url: format!(
                "https://plugins.dprint.dev/mridang/dprint-plugin-rusty/{}/rumdl-schema.json",
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

        let line_length = get_value(
            &mut config,
            "lineWidth",
            global_config.line_width.unwrap_or(120),
            &mut diagnostics,
        );
        let flavor = get_value(
            &mut config,
            "flavor",
            MarkdownFlavor::Standard,
            &mut diagnostics,
        );

        diagnostics.extend(get_unknown_property_diagnostics(config));

        PluginResolveConfigurationResult {
            config: Configuration {
                line_length,
                flavor,
            },
            diagnostics,
            file_matching: FileMatchingInfo {
                file_extensions: vec!["md".to_string(), "mdx".to_string(), "qmd".to_string()],
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

        let mut config = RumdlConfig::default();
        config.global.line_length = LineLength::new(request.config.line_length as usize);
        config.global.flavor = request.config.flavor;

        let all_rules = all_rules(&config);
        let rules = filter_rules(&all_rules, &config.global);

        let warnings = rumdl_lib::lint(
            &content,
            &rules,
            false,
            config.global.flavor,
            None,
            Some(&config),
        )
        .map_err(|e| anyhow!("{e}"))?;

        let coordinator = FixCoordinator::new();
        let mut fixed_content = content.clone();

        let _ = coordinator.apply_fixes_iterative(
            &rules,
            &warnings,
            &mut fixed_content,
            &config,
            10,
            None,
        );

        if fixed_content == content {
            Ok(None)
        } else {
            Ok(Some(fixed_content.into_bytes()))
        }
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
dprint_core::generate_plugin_code!(RumdlPluginHandler, RumdlPluginHandler);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_trailing_spaces() {
        let content = "Line with trailing spaces   \n";

        let mut config = RumdlConfig::default();
        config.global.line_length = LineLength::new(80);
        config.global.flavor = MarkdownFlavor::Standard;

        let all_rules = all_rules(&config);
        let rules = filter_rules(&all_rules, &config.global);
        let warnings = rumdl_lib::lint(
            content,
            &rules,
            false,
            config.global.flavor,
            None,
            Some(&config),
        )
        .expect("lint should succeed");

        let coordinator = FixCoordinator::new();
        let mut fixed_content = content.to_string();
        let _ = coordinator.apply_fixes_iterative(
            &rules,
            &warnings,
            &mut fixed_content,
            &config,
            10,
            None,
        );

        assert!(fixed_content.ends_with("Line with trailing spaces\n"));
        assert!(fixed_content.len() <= content.len());
    }
}
