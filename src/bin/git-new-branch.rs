use anyhow::Result;
use clap::Parser;
use git_tp::git::GitCommand;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    branch_name: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let git = GitCommand::new();

    let current = git.current_branch()?;
    let upstream = git.upstream(&current)?;

    git.run_quiet(&["branch", &args.branch_name])?;
    git.run_quiet(&["branch", "--set-upstream-to", &upstream, &args.branch_name])?;

    Ok(())
}
