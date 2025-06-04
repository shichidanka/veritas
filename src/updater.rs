use anyhow::Result;
use directories::BaseDirs;
use reqwest::blocking::Client;
use semver::Version;
use serde::Deserialize;
use std::{process::Command, io::Write};

#[derive(Clone, Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    assets: Vec<GithubAsset>,
}

#[derive(Clone, Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Clone)]
pub struct Updater {
    client: Client,
    current_version: String,
}

impl Updater {
    pub fn new(current_version: &str) -> Self {
        Self {
            client: Client::builder()
                .user_agent("veritas")
                .build()
                .unwrap(),
            current_version: current_version.to_string(),
        }
    }

    pub fn check_update(&self) -> Result<Option<String>> {
        let response = self.client
            .get("https://api.github.com/repos/hessiser/veritas/releases/latest")
            .send()?;

        let release = response.json::<GithubRelease>()?;

        // unnecessary but good anyways
        let latest_tag = release.tag_name.trim_start_matches('v').trim();
        let current_tag = self.current_version.trim_start_matches('v').trim();

        /*
        log::debug!("GitHub tag_name: {:?}", release.tag_name);
        log::debug!("Current version: {:?}", self.current_version);
        log::debug!("Parsed latest_tag: {:?}", latest_tag);
        log::debug!("Parsed current_tag: {:?}", current_tag);
        */

        let latest_ver = Version::parse(latest_tag);
        let current_ver = Version::parse(current_tag);

        let update_needed = match (latest_ver, current_ver) {
            (Ok(latest), Ok(current)) => {
                log::debug!("semver compare: latest={:?}, current={:?}", latest, current);
                latest > current
            }
            (Err(e1), Err(e2)) => {
                log::debug!("semver parse failed: {:?}, {:?}", e1, e2);
                latest_tag != current_tag
            }
            (Err(e), _) | (_, Err(e)) => {
                log::debug!("semver parse failed: {:?}", e);
                latest_tag != current_tag
            }
        };

        if update_needed {
            Ok(Some(release.tag_name))
        } else {
            Ok(None)
        }
    }

    pub fn download_update(
        &self,
        version: &str,
        dll_dir: &str,
        dll_filename: &str,
        defender_exclusion: bool,
    ) -> Result<()> {
        let response = self.client
            .get("https://api.github.com/repos/hessiser/veritas/releases/latest")
            .send()?;

        let release = response.json::<GithubRelease>()?;
        
        let dll_asset = release.assets
            .iter()
            .find(|a| a.name == "veritas.dll")
            .ok_or_else(|| anyhow::anyhow!("DLL not found in release"))?;

        // %APPDATA%/veritas-update
        let update_dir = BaseDirs::new()
            .map(|dirs| dirs.data_dir().join("veritas-update"))
            .ok_or_else(|| anyhow::anyhow!("Could not get appdata directory"))?;
        std::fs::create_dir_all(&update_dir)?;

        let update_dll_path = update_dir.join(format!("{dll_filename}.tmp"));

        let exclusion_file = update_dll_path.to_string_lossy();
        if defender_exclusion {
            let _ = Command::new("powershell")
                .args([
                    "-Command",
                    &format!("Add-MpPreference -ExclusionPath \"{}\"", exclusion_file),
                ])
                .spawn();
        }

        let dll_bytes = self.client
            .get(&dll_asset.browser_download_url)
            .send()?
            .bytes()?;
        std::fs::write(&update_dll_path, dll_bytes)?;

        let target_path = std::path::Path::new(dll_dir).join(dll_filename);
        let pid = std::process::id();

        let src = update_dll_path.to_string_lossy().to_string();
        let dst = target_path.to_string_lossy().to_string();

        let bat_path = update_dir.join("veritas_update.bat");
        let mut bat = std::fs::File::create(&bat_path)?;

        // batch script
        writeln!(bat, "@echo on")?;
        writeln!(bat, "echo Waiting for process PID to exit...")?;
        writeln!(bat, ":waitloop")?;
        writeln!(bat, "tasklist /FI \"PID eq {pid}\" | find /I \"{pid}\" >nul")?;
        writeln!(bat, "if not errorlevel 1 (")?;
        writeln!(bat, "    timeout /t 1 >nul")?;
        writeln!(bat, "    goto waitloop")?;
        writeln!(bat, ")")?;
        writeln!(bat, "echo Copying DLL...")?;
        writeln!(bat, "move /Y \"{}\" \"{}\"", src, dst)?;
        writeln!(bat, "if errorlevel 1 (")?;
        writeln!(bat, "    echo Move failed!")?;
        writeln!(bat, "    pause")?;
        writeln!(bat, "    exit /b 1")?;
        writeln!(bat, ")")?;
        if defender_exclusion {
            writeln!(bat, "echo Removing Defender exclusion...")?;
            writeln!(bat, "powershell -Command \"Remove-MpPreference -ExclusionPath '{}'", exclusion_file)?;
        }
        writeln!(bat, "echo Done.")?;
        writeln!(bat, "del /F /Q \"{}\"", bat_path.to_string_lossy())?;
        // writeln!(bat, "pause")?;

        std::process::Command::new("cmd")
            .arg("/C")
            .arg(bat_path)
            .spawn()?;

        std::process::exit(0);
    }
}
