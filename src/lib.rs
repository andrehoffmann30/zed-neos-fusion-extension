use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

#[allow(dead_code)]
struct NeosFusionExtension {
    cached_binary_path: Option<String>,
}

impl zed::Extension for NeosFusionExtension {
    fn new() -> Self {
        Self { cached_binary_path: None }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<zed::Command> {
        Err("not yet implemented".into())
    }
}

zed::register_extension!(NeosFusionExtension);
