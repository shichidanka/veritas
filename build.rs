use cargo_metadata::MetadataCommand;

fn main() {
    let metadata_cmd = MetadataCommand::new()
        .exec()
        .unwrap();

    let metadata = &metadata_cmd
        .root_package()
        .unwrap()
        .metadata;

    let region = metadata["region"].as_str().unwrap();
    let patch = metadata["patch"].as_str().unwrap();
    let hotfix = metadata["hotfix"].as_str().unwrap();

    let build_metadata = if metadata["is_beta"].as_bool().unwrap() {
        format!("{}-beta-{}-{}", region, patch, hotfix)
    }
    else {
        format!("{}-prod-{}-{}", region, patch, hotfix)
    };
    println!("cargo:rustc-env=TARGET_BUILD={}", build_metadata);

    winres::WindowsResource::new()
        .set("FileDescription", &build_metadata)
        .compile()
        .unwrap();
}