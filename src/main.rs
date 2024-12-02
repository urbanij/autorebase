// Tool to automatically rebase branches.

use anyhow::Result;
use clap::Parser;
use git_auto::autorebase;
use git_auto::{Args, OpType, Operation};
use std::env::current_dir;

#[derive(Parser)]
#[command(name = "git-auto")]
#[command(about = "A Git automation tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Operation,
}

fn handle_rebase(args: Args) -> Result<()> {
    autorebase(
        &current_dir()?,
        args.onto.as_deref(),
        args.slow,
        args.include_non_local,
        args.match_branches.as_deref(),
        OpType::Rebase,
    )
}

fn handle_merge(args: Args) -> Result<()> {
    autorebase(
        &current_dir()?,
        args.onto.as_deref(),
        args.slow,
        args.include_non_local,
        args.match_branches.as_deref(),
        OpType::Merge,
    )
}

fn main() -> Result<()> {
    let res = run();
    if res.is_err() {
        // Print a newline because there may be a half finished output
        // (e.g. using `eprint!()` instead of `eprintln!()`.
        eprintln!();
    }
    res
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Operation::Rebase(args) => handle_rebase(args)?,
        Operation::Merge(args) => handle_merge(args)?,
    }

    Ok(())
}
