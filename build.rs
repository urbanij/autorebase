use std::process::Command;
use tempfile::tempdir;

fn main() {
    // Assume `--initial-branch` argument is unsupported by default
    let mut git_supports_initial_branch = false;

    // Create a temporary directory to test `git init --initial-branch=foo`
    if let Ok(temp_dir) = tempdir() {
        let result = Command::new("git")
            .args(["init", "--initial-branch=foo"])
            .current_dir(temp_dir.path())
            .output();

        // If the command succeeded, set the flag
        if let Ok(output) = result {
            if output.status.success() {
                git_supports_initial_branch = true;
            }
        }
    }

    // Enable the custom feature if the Git version supports --initial-branch
    if git_supports_initial_branch {
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rustc-cfg=git_supports_initial_branch");
    }
}
