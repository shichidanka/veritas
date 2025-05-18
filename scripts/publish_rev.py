import subprocess
import toml
import os

import os
import re
from tomlkit import parse, dumps, value

class Patch:
    def __init__(self, major, minor, revision):
        self.major = major
        self.minor = minor
        self.revision = revision

    def __str__(self):
        return f"{self.major}.{self.minor}.{self.revision}"

def update_cargo_toml(update: str):
    patch_re = re.compile(r"(\d+)\.(\d+)\.(\d+)")

    cargo_toml_path = "Cargo.toml"
    with open(cargo_toml_path, "r") as f:
        toml_str = f.read()

    doc = parse(toml_str)
    metadata = doc.get("package", {}).get("metadata")
    if metadata and "patch" in metadata:
        patch_str = metadata["patch"]
        match = patch_re.match(patch_str)
        if match:
            major_patch, minor_patch, revision_patch = map(int, match.groups())
            patch = Patch(major_patch, minor_patch, revision_patch)

            if update == "MAJOR_PATCH":
                patch.major += 1
                patch.minor = 0
                patch.revision = 0
                metadata["hotfix"] = "0.1"
            elif update == "MINOR_PATCH":
                patch.minor += 1
                patch.revision = 0
                metadata["hotfix"] = "0.1"
            elif update == "REVISION_PATCH":
                patch.revision += 1
                metadata["hotfix"] = "0.1"
            elif update == "HOTFIX_PATCH":
                hotfix = str(float(metadata.get("hotfix", "0.0")) + 0.1)
                metadata["hotfix"] = hotfix

            metadata["patch"] = str(patch)

        with open(cargo_toml_path, "w") as f:
            f.write(dumps(doc))

# Run cargo build --release
def build_with_cargo():
    print("Building the project with cargo...")
    subprocess.run(['cargo', 'build', '--release'], check=True)
    print("Build successful.")

# Read Cargo.toml
def read_cargo_toml():
    # Load Cargo.toml file using toml library
    try:
        with open('Cargo.toml', 'r') as file:
            config = toml.load(file)
            metadata = config.get('package', {}).get('metadata', {})
            return metadata
    except FileNotFoundError:
        print("Error: Cargo.toml not found.")
        exit(1)

# Commit and tag the changes in git
def commit_and_tag(patch, region, is_beta, hotfix):
    # Git commit with the message
    commit_message = f"Update to {patch}"
    subprocess.run(['git', 'commit', '-am', commit_message], check=True)

    # Construct the tag
    if is_beta:
        tag = f"{region}-beta-{patch}-{hotfix}"
    else:
        tag = f"{region}-prod-{patch}-{hotfix}"

    # Git tag the commit
    subprocess.run(['git', 'tag', tag], check=True)

    print(f"Committed and tagged as {tag}")

    # Push the commit and tags to the remote repository
    subprocess.run(['git', 'push', 'origin', 'HEAD'], check=True)  # Push the commit
    subprocess.run(['git', 'push', 'origin', '--tags'], check=True)  # Push the tags
    print("Pushed the commit and tags to the remote repository.")

def main():
    # Step 1: Set the UPDATE environment variable and build with cargo
    update_cargo_toml("REVISION_PATCH")
    build_with_cargo()

    # Step 2: Read Cargo.toml and extract metadata
    metadata = read_cargo_toml()

    region = metadata.get('region', 'unknown')
    is_beta = metadata.get('is_beta', False)
    patch = metadata.get('patch', '0.0.0')
    hotfix = metadata.get('hotfix', '0.0')

    # Step 3: Commit and tag the changes
    commit_and_tag(patch, region, is_beta, hotfix)

if __name__ == "__main__":
    main()
