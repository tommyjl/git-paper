use anyhow::{anyhow, Result};
use clap::Parser;
use git_tp::file_iter::FileFinder;
use git_tp::git::GitCommand;
use std::env::var;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    repo_name: String,
}

fn get_code_path() -> Result<PathBuf> {
    let home = var("HOME")?;
    Ok(PathBuf::from(format!("{}/code", home)))
}

fn match_repo_name(path: &Path, repo_name: &str) -> bool {
    path.file_name().map_or(false, |name| name == repo_name)
}

fn is_inside_git_repo(path: &Path) -> bool {
    let mut git = GitCommand::new();
    git.set_working_directory(path);
    git.run_quiet(&["rev-parse", "--is-inside-git-dir"]).is_ok()
}

pub fn main() -> Result<()> {
    let args = Args::parse();
    let code_path = get_code_path()?;

    if let Some(found) = FileFinder::new(code_path)
        .ignore_dir_fn(is_inside_git_repo)
        .into_iter()
        .find(|dir| match_repo_name(dir, &args.repo_name))
    {
        println!("{}", found.display());
        Ok(())
    } else {
        Err(anyhow!("No such git repo"))
    }
}
