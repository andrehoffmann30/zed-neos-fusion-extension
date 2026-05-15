#[allow(unused_imports)]
use std::fs;
use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

#[allow(dead_code)]
const LSP_BINARY_NAME: &str = "neos-fusion-language-server";
#[allow(dead_code)]
const LSP_SERVER_VERSION: &str = "0.1.0";

#[allow(dead_code)]
struct NeosFusionExtension {
    cached_binary_path: Option<String>,
}

impl NeosFusionExtension {
    #[allow(dead_code)]
    fn release_asset_name() -> Result<String> {
        let (os, arch) = zed::current_platform();
        let os_str = match os {
            zed::Os::Mac => "macos",
            zed::Os::Linux => "linux",
            zed::Os::Windows => return Err("Windows is not yet supported by neos-fusion-ls".into()),
        };
        let arch_str = match arch {
            zed::Architecture::Aarch64 => "aarch64",
            zed::Architecture::X8664 => "x86_64",
            zed::Architecture::X86 => return Err("32-bit x86 is not supported".into()),
        };
        Ok(format!("{LSP_BINARY_NAME}-{os_str}-{arch_str}"))
    }
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
        // Download logic added in Task 6
        Err("LSP binary download not yet implemented".into())
    }
}

zed::register_extension!(NeosFusionExtension);
