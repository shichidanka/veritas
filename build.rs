fn main() {
    // Parse the Cargo.toml version into a single 64‑bit integer:
    let ver = env!("CARGO_PKG_VERSION")
    .split('.')
    .map(|x| x.parse::<u64>().unwrap())
    .collect::<Vec<_>>();
    let sem_ver = (ver[0] << 48) | (ver[1] << 32) | (ver[2] << 16);

    // Only try to embed Windows resources when building for Windows:
    if cfg!(all(target_os = "windows", target_env = "msvc")) {
        let mut res = winres::WindowsResource::new();
        res.set_version_info(winres::VersionInfo::PRODUCTVERSION, sem_ver);

        // If there’s no metadata or missing files, just warn instead of panic.
        if let Err(e) = res.compile() {
            println!("cargo:warning=skipping Windows resource embedding: {}", e);
        }
    }
}
