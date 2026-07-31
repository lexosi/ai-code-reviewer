use anyhow::{Context, Result, bail};
use std::process::Command;

pub fn get_diff() -> Result<String> {
    run_git(&["diff", "HEAD~1", "HEAD"])
}

pub(crate) fn run_git(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .context("failed to run git")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git {} failed: {stderr}", args.join(" "));
    }

    String::from_utf8(output.stdout).context("git output is not valid UTF-8")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn git_is_available() {
        // Verifies git is on PATH and we are inside a repository.
        let result = run_git(&["rev-parse", "--git-dir"]);
        assert!(result.is_ok(), "expected git to be available: {:?}", result);
    }

    #[test]
    fn invalid_git_subcommand_returns_error() {
        let result = run_git(&["not-a-real-subcommand"]);
        assert!(result.is_err());
    }

    #[test]
    fn get_diff_returns_result() {
        // Environment-tolerant: on a shallow clone HEAD~1 may not exist → Err
        // (error propagation path). When Ok, the output must be well-formed
        // git diff text: either empty (no changes) or containing a real diff
        // header line. A garbage/corrupted capture would fail this.
        let result = get_diff();
        match result {
            Ok(diff) => assert!(
                diff.is_empty() || diff.contains("diff --git"),
                "expected empty or valid git diff output, got: {diff:?}"
            ),
            Err(e) => assert!(e.to_string().contains("git")),
        }
    }

    #[test]
    fn run_git_captures_stdout() {
        let output = run_git(&["rev-parse", "HEAD"]).unwrap();
        // A commit hash is 40 hex characters followed by a newline.
        assert_eq!(output.trim().len(), 40);
    }
}
