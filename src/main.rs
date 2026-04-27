mod claude;
mod config;
mod git;
mod telegram;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "ai-code-reviewer", about = "AI-powered post-commit code reviewer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the full review pipeline on the latest commit
    Review {
        /// Print the review to stdout instead of sending to Telegram
        #[arg(long)]
        dry_run: bool,
    },
    /// Install the post-commit git hook into .git/hooks/
    Install,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Review { dry_run } => run_review(dry_run),
        Commands::Install => run_install(),
    }
}

fn run_review(dry_run: bool) -> Result<()> {
    let config = config::Config::load()?;

    let diff = git::get_diff()?;
    if diff.trim().is_empty() {
        println!("No diff found — nothing to review.");
        return Ok(());
    }

    let diff = if diff.len() > config.max_diff_chars {
        println!(
            "Diff truncated from {} to {} chars.",
            diff.len(),
            config.max_diff_chars
        );
        &diff[..config.max_diff_chars]
    } else {
        &diff
    };

    println!("Sending diff to Claude for review...");
    let review = claude::review_diff(&config.anthropic_api_key, diff)?;

    if dry_run {
        println!("{review}");
    } else {
        println!("Sending review to Telegram...");
        let message = format!("*AI Code Review*\n\n{review}");
        telegram::send_message(
            &config.telegram_bot_token,
            &config.telegram_chat_id,
            &message,
        )?;
        println!("Done.");
    }

    Ok(())
}

fn run_install() -> Result<()> {
    let hooks_dir = Path::new(".git/hooks");
    if !hooks_dir.exists() {
        anyhow::bail!(
            ".git/hooks not found — run this command from the root of a git repository"
        );
    }

    let hook_src = Path::new("hooks/post-commit");
    if !hook_src.exists() {
        anyhow::bail!("hooks/post-commit not found — make sure you are running from the project root");
    }

    let dest = hooks_dir.join("post-commit");
    fs::copy(hook_src, &dest)
        .with_context(|| format!("failed to copy hook to {}", dest.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&dest)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&dest, perms)?;
    }

    println!("Hook installed at {}", dest.display());
    Ok(())
}
