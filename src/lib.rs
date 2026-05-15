use std::fs;
use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree, serde_json};

const LSP_BINARY_NAME: &str = "neos-fusion-language-server";
// Bump this constant to trigger a fresh binary download on next use.
const LSP_SERVER_VERSION: &str = "0.1.0";

struct NeosFusionExtension {
    cached_binary_path: Option<String>,
}

impl NeosFusionExtension {
    fn release_asset_name() -> Result<String> {
        let (os, arch) = zed::current_platform();
        let os_str = match os {
            zed::Os::Mac => "macos",
            zed::Os::Linux => "linux",
            zed::Os::Windows => return Err("Windows is not yet supported".into()),
        };
        let arch_str = match arch {
            zed::Architecture::Aarch64 => "aarch64",
            zed::Architecture::X8664 => "x86_64",
            zed::Architecture::X86 => return Err("32-bit x86 is not supported".into()),
        };
        Ok(format!("{LSP_BINARY_NAME}-{os_str}-{arch_str}"))
    }

    fn lsp_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
    ) -> Result<String> {
        let asset_name = Self::release_asset_name()?;
        let version_dir = format!("{LSP_BINARY_NAME}-{LSP_SERVER_VERSION}");
        let binary_path = format!("{version_dir}/{asset_name}");

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        // Use the cached path if the binary file still exists on disk.
        if let Some(ref cached) = self.cached_binary_path {
            if fs::metadata(cached).is_ok_and(|m| m.is_file()) {
                return Ok(cached.clone());
            }
        }

        // Skip download if the expected version is already on disk.
        if fs::metadata(&binary_path).is_ok_and(|m| m.is_file()) {
            self.cached_binary_path = Some(binary_path.clone());
            return Ok(binary_path);
        }

        // Download from this repository's GitHub Releases.
        let release = zed::latest_github_release(
            "andrehoffmann30/zed-neos-fusion-extension",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )
        .map_err(|e| format!("failed to fetch latest release: {e}"))?;

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| {
                format!("no release asset found for {asset_name} in release {}", release.version)
            })?;

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        fs::create_dir_all(&version_dir)
            .map_err(|e| format!("failed to create version directory: {e}"))?;

        zed::download_file(&asset.download_url, &binary_path, zed::DownloadedFileType::Uncompressed)
            .map_err(|e| format!("failed to download {asset_name}: {e}"))?;

        zed::make_file_executable(&binary_path)
            .map_err(|e| format!("failed to make {asset_name} executable: {e}"))?;

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for NeosFusionExtension {
    fn new() -> Self {
        Self { cached_binary_path: None }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<zed::Command> {
        let binary = self.lsp_binary_path(language_server_id)?;
        Ok(zed::Command {
            command: binary,
            args: vec!["--stdio".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(serde_json::json!({
            "workspacePath": worktree.root_path(),
            "folders": {
                "packageFolders": ["DistributionPackages", "Packages/Application"],
                "ignore": []
            },
            "diagnostics": {
                "enabled": true,
                "enabledDiagnostics": [
                    "FusionProperties",
                    "ResourceUris",
                    "TagNames",
                    "EelHelperArguments",
                    "PrototypeNames",
                    "EmptyEel",
                    "ActionUri",
                    "NodeTypeDefinitions",
                    "NonParsedFusion",
                    "RootFusionConfiguration",
                    "TranslationShortHand",
                    "ParserError",
                    "AfxWithDollarEel",
                    "DuplicateStatements"
                ],
                "levels": {
                    "deprecations": "warning"
                },
                "ignoreNodeTypes": [],
                "alwaysDiagnoseChangedFile": true
            },
            "inlayHint": {
                "depth": "literal"
            }
        })))
    }
}

zed::register_extension!(NeosFusionExtension);
