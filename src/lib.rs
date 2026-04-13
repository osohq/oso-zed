use zed_extension_api::process::Command as ProcessCommand;
use zed_extension_api::{
    self as zed, Architecture, DownloadedFileType, LanguageServerId, LanguageServerInstallationStatus,
    Os, Result, Worktree, current_platform, download_file, make_file_executable, process::Command,
    set_language_server_installation_status,
};

struct PolarExtension {}

const OSO_CLOUD_BINARY: &str = "oso-cloud";
const LOCAL_OSO_CLOUD_BINARY: &str = "oso-cloud-local";
const OSO_CLOUD_DOWNLOAD_HOST: &str = "https://d3i4cc4dqewpo9.cloudfront.net";
const REQUIRED_OSO_CLOUD_VERSION: (u64, u64, u64) = (0, 18, 0);

fn parse_semver(version: &str) -> Option<(u64, u64, u64)> {
    let mut parts = version.split('.');
    let major = parts.next()?.parse::<u64>().ok()?;
    let minor = parts.next()?.parse::<u64>().ok()?;
    let patch = parts.next()?.parse::<u64>().ok()?;
    Some((major, minor, patch))
}

fn extract_version_string(output: &str) -> Option<String> {
    let line = output.lines().find(|line| line.starts_with("version: "))?;
    let value = line.strip_prefix("version: ")?;
    let version = value
        .split(';')
        .next()
        .map(str::trim)
        .unwrap_or(value)
        .to_string();
    if version.is_empty() {
        return None;
    }
    Some(version)
}

fn platform_target() -> Result<&'static str> {
    let (os, arch) = current_platform();
    match (os, arch) {
        (Os::Mac, Architecture::X8664) => Ok("mac_osx_x86_64"),
        (Os::Mac, Architecture::Aarch64) => Ok("mac_osx_arm64"),
        (Os::Linux, Architecture::Aarch64) => Ok("linux_arm64"),
        (Os::Linux, _) => Ok("linux_musl"),
        _ => Err("automatic oso-cloud install is not supported on this platform".to_string()),
    }
}

fn install_or_update_oso_cloud(language_server_id: &LanguageServerId) -> Result<()> {
    set_language_server_installation_status(
        language_server_id,
        &LanguageServerInstallationStatus::Downloading,
    );

    let target = platform_target()?;
    let download_url = format!("{OSO_CLOUD_DOWNLOAD_HOST}/latest/oso_cli_{target}");
    if let Err(error) = download_file(
        &download_url,
        LOCAL_OSO_CLOUD_BINARY,
        DownloadedFileType::Uncompressed,
    ) {
        let message = format!("failed to download oso-cloud: {error}");
        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::Failed(message.clone()),
        );
        return Err(message);
    }

    if let Err(error) = make_file_executable(LOCAL_OSO_CLOUD_BINARY) {
        let message = format!("downloaded oso-cloud but failed to make it executable: {error}");
        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::Failed(message.clone()),
        );
        return Err(message);
    }

    set_language_server_installation_status(
        language_server_id,
        &LanguageServerInstallationStatus::None,
    );
    Ok(())
}

fn extract_version_info_for_command(command_path: &str, worktree: &Worktree) -> Result<(String, bool)> {
    let mut command = ProcessCommand::new(command_path)
        .arg("version")
        .envs(worktree.shell_env());
    let output = command.output()?;
    if output.status != Some(0) {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "failed to run `{OSO_CLOUD_BINARY} version`: {}",
            stderr.trim()
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let version = extract_version_string(&stdout).ok_or_else(|| {
        format!("unexpected output from `{OSO_CLOUD_BINARY} version`; cannot determine version")
    })?;
    let update_available = stderr.to_lowercase().contains("update available");

    Ok((version, update_available))
}

fn ensure_oso_cloud_binary(
    language_server_id: &LanguageServerId,
    worktree: &Worktree,
) -> Result<String> {
    let mut command = match worktree.which(OSO_CLOUD_BINARY) {
        Some(_) => OSO_CLOUD_BINARY.to_string(),
        None => {
            install_or_update_oso_cloud(language_server_id)?;
            LOCAL_OSO_CLOUD_BINARY.to_string()
        }
    };

    set_language_server_installation_status(
        language_server_id,
        &LanguageServerInstallationStatus::CheckingForUpdate,
    );

    let (version, update_available) = extract_version_info_for_command(&command, worktree)?;
    let too_old = version != "development"
        && parse_semver(&version)
            .map(|current| current < REQUIRED_OSO_CLOUD_VERSION)
            .unwrap_or(true);

    if too_old || update_available {
        install_or_update_oso_cloud(language_server_id)?;
        command = LOCAL_OSO_CLOUD_BINARY.to_string();
    } else {
        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::None,
        );
    }

    Ok(command)
}

impl zed::Extension for PolarExtension {
    fn new() -> Self {
        PolarExtension {}
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        Ok(Command {
            command: ensure_oso_cloud_binary(language_server_id, worktree)?,
            args: vec![String::from("lsp")],
            env: Default::default(),
        })
    }
}

zed::register_extension!(PolarExtension);
