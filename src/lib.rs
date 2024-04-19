use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

struct RuffExtension;

impl zed::Extension for RuffExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let Some(ruff_bin) = worktree.which("ruff-lsp") else {
            return Err("ruff-lsp not available".to_string());
        };
        Ok(zed::Command {
            command: ruff_bin.to_string(),
            args: Default::default(),
            env: Default::default(),
        })
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        _completion: zed::lsp::Completion,
    ) -> Option<zed::CodeLabel> {
        // Ruff LSP doesn't have completions, so we don't have to label them.
        None
    }
}

zed::register_extension!(RuffExtension);
