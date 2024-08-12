use zed::LanguageServerId;
use zed_extension_api::{self as zed, serde_json::json, settings::LspSettings, Result};

struct PythonLspExtension {}

impl zed::Extension for PythonLspExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match worktree.which("pylsp") {
            Some(path) => Ok(zed::Command {
                command: path,
                args: vec!["-v".into()],
                env: vec![],
            }),
            None => Err("Unable to find pylsp from worktree".into()),
        }
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        // pylsp expects a top level `pylsp` key, so we'll wrap the settings
        let settings = json!({
            "pylsp": settings
        });

        Ok(Some(settings))
    }
}

zed::register_extension!(PythonLspExtension);
