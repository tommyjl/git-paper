use std::io::Result;
use std::process::{Command, Stdio};

pub struct GitCommand;

impl Default for GitCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl GitCommand {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run_quiet(&self, args: &[&str]) -> Result<()> {
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
