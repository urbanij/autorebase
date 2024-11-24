use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    /// the target branch to pull and rebase onto;
    /// defaults to `git config --get init.defaultBranch` or `master` if unset
    #[arg(long, short)]
    pub onto: Option<String>,

    /// if there are conflicts, try rebasing commit by commit backwards from the
    /// target, instead of trying to determined the conflicting commit on the
    /// target branch directly
    #[arg(long, short)]
    pub slow: bool,

    /// include branches which have an upstream, the default is to exclude these
    #[arg(long, short)]
    pub include_non_local: bool,

    /// branch matching glob, the default is all branches
    #[arg(long, short)]
    pub match_branches: Option<String>,

    /// RUST_LOG-style logging string, e.g. --log debug
    #[arg(long, short)]
    pub log: Option<String>,
}

// #[derive(Parser)]
// struct MergeArgs {
//     /// Branch to merge from
//     #[arg(long, short)]
//     branch: Option<String>,

//     /// Squash merge
//     #[arg(long, short)]
//     squash: bool,

//     /// Skip fast-forward
//     #[arg(long)]
//     no_ff: bool,
// }

#[derive(Subcommand)]
pub enum Operation {
    /// Rebase the current branch
    Rebase(Args),
    /// Merge another branch
    Merge(Args),
}

pub enum OpType {
    Rebase,
    Merge,
}
