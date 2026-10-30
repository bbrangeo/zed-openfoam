use std::{env, fs};
use zed::settings::LspSettings;
use zed_extension_api::{self as zed, LanguageServerId, Result};

const PACKAGE_NAME: &str = "foam-language-server";
const BINARY_NAME: &str = "foam-ls";
// The npm package exposes its launcher at bin/foam-ls (plain JS, Node >= 18).
const SERVER_PATH: &str = "node_modules/foam-language-server/bin/foam-ls";

struct FoamExtension {
    cached_binary_path: Option<String>,
}

impl FoamExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).is_ok_and(|stat| stat.is_file())
    }

    fn server_script_path(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        let server_exists = self.server_exists();
        if self.cached_binary_path.is_some() && server_exists {
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

        if !server_exists
            || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let result = zed::npm_install_package(PACKAGE_NAME, &version);
            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                            "installed package '{PACKAGE_NAME}' did not contain expected path '{SERVER_PATH}'"
                        ))?;
                    }
                }
                Err(error) => {
                    if !self.server_exists() {
                        Err(error)?;
                    }
                }
            }
        }
        Ok(SERVER_PATH.to_string())
    }
}

impl zed::Extension for FoamExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Prefer a globally installed foam-ls if one is on the PATH.
        if let Some(path) = worktree.which(BINARY_NAME) {
            return Ok(zed::Command {
                command: path,
                args: vec!["--stdio".to_string()],
                env: Default::default(),
            });
        }

        let server_path = self.server_script_path(language_server_id)?;
        let server_path = env::current_dir()
            .unwrap()
            .join(&server_path)
            .to_string_lossy()
            .to_string();
        self.cached_binary_path = Some(server_path.clone());

        // Published foam-language-server versions (<= 0.4.6) are incompatible
        // with Zed's InitializeResult deserializer:
        // 1. crash in `initialize` with `Cannot read properties of undefined
        //    (reading 'uri')` when `workspaceFolders` is absent, and
        // 2. declare `completionProvider: false` (boolean), where Zed expects
        //    a `CompletionOptions` struct.
        // Patch the compiled server in place with upstream master's fixes:
        //   rootUri = params.workspaceFolders?.[0]?.uri ?? params.rootUri ?? null;
        //   completionProvider: { resolveProvider: true, triggerCharacters: [...] }
        let lib_path = env::current_dir()
            .unwrap()
            .join("node_modules/foam-language-server/lib/foam-ls.js");
        if let Ok(source) = fs::read_to_string(&lib_path) {
            let patched = source
                .replace(
                    "rootUri = params.workspaceFolders[0].uri;",
                    "rootUri = params.workspaceFolders?.[0]?.uri ?? params.rootUri ?? null;",
                )
                .replace(
                    "completionProvider: false,",
                    "completionProvider: { resolveProvider: true, triggerCharacters: [\"$\", \"#\", \" \", \"\\t\"] },",
                );
            if patched != source {
                let _ = fs::write(&lib_path, patched);
            }
        }

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![server_path, "--stdio".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        // The server reads settings under `foam.languageserver.*`
        // (diagnostics, completion). Users configure it via
        // `lsp.foam-ls.settings` in their settings.json.
        LspSettings::for_worktree(server_id.as_ref(), worktree)
            .map(|lsp_settings| lsp_settings.settings)
    }
}

zed::register_extension!(FoamExtension);
