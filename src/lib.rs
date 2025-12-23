use zed_extension_api::{self as zed, Result, process::Command};

struct PolarExtension {}

fn get_lsp_binary_path(worktree: &zed_extension_api::Worktree) -> Result<String> {
    worktree
        .which("oso-cloud")
        .ok_or_else(|| "no oso-cloud binary found".to_string())
}

impl zed::Extension for PolarExtension {
    fn new() -> Self {
        PolarExtension {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        Ok(Command {
            command: get_lsp_binary_path(worktree)?,
            args: vec![String::from("lsp")],
            env: Default::default(),
        })
    }
}

zed::register_extension!(PolarExtension);
