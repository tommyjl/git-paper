use clap::Parser;
use std::io::Result;
use std::process::{Command, ExitStatus, Stdio};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    branch_name: String,
}

pub struct GitCommand;

impl GitCommand {
    pub fn new() -> Self {
        Self {}
    }

    fn run_quiet(&self, args: &[&str]) -> Result<()> {
        let status = Command::new("git")
            .args(args)
            .stdout(Stdio::null())
            .spawn()?
            .wait()?;

        if !status.success() {
            // TODO: Handle error
            panic!("error status = {}", status);
        }

        Ok(())
    }

    fn output_with_args(&self, args: &[&str]) -> Result<String> {
        let output = Command::new("git").args(args).output()?;

        if !output.status.success() {
            // TODO: Handle error
            println!("upstream status {}", output.status);
            panic!("{:?}", output.stderr);
        }

        let result = String::from_utf8(output.stdout)
            .unwrap()
            .trim_end()
            .to_owned();

        Ok(result)
    }

    pub fn current_branch(&self) -> Result<String> {
        self.output_with_args(&["branch", "--show-current"])
    }

    pub fn upstream(&self, branch: &str) -> Result<String> {
        self.output_with_args(&[
            "rev-parse",
            "--abbrev-ref",
            "--symbolic-full-name",
            &format!("{branch}@{{upstream}}"),
        ])
    }
}

fn git_new_branch() -> Result<()> {
    let args = Args::parse();

    let git = GitCommand::new();

    let current = git.current_branch()?;
    let upstream = git.upstream(&current)?;

    git.run_quiet(&["branch", &args.branch_name])?;
    git.run_quiet(&["branch", "--set-upstream-to", &upstream, &args.branch_name])?;

    // println!("{:?}", args);
    // println!("Current = {}", current);
    // println!("Upstream = {}", upstream);
    Ok(())
}

fn main() -> Result<()> {
    git_new_branch()
}
