use anyhow::{anyhow, Result};
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub struct GitCommand {
    working_directory: PathBuf,
}

impl Default for GitCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl GitCommand {
    pub fn new() -> Self {
        let working_directory = current_dir().unwrap();
        Self { working_directory }
    }

    pub fn set_working_directory(&mut self, working_directory: &Path) {
        self.working_directory = PathBuf::from(working_directory);
    }

    pub fn run_quiet(&self, args: &[&str]) -> Result<()> {
        let status = Command::new("git")
            .args(args)
            .current_dir(&self.working_directory)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;

        if !status.success() {
            Err(anyhow!("ExitStatus not 0"))
        } else {
            Ok(())
        }
    }

    pub fn output_with_args(&self, args: &[&str]) -> Result<String> {
        let output = Command::new("git")
            .args(args)
            .current_dir(&self.working_directory)
            .output()?;

        if !output.status.success() {
            Err(anyhow!("ExitStatus not 0"))
        } else {
            let result = String::from_utf8(output.stdout)?.trim_end().to_owned();
            Ok(result)
        }
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
