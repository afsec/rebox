#![warn(
    rust_2018_idioms,
    unused_lifetimes,
    semicolon_in_expressions_from_macros
)]

mod cli;
mod helpers;

// use anyhow::bail;
use helpers::XtaskResult;
use std::{
    env,
    path::{Path, PathBuf},
};
use xshell::{cmd, Shell};

use crate::helpers::project_root;

fn main() -> XtaskResult<()> {
    use clap::Parser;

    let cli = cli::Args::parse();

    let sh = &Shell::new()?;

    sh.change_dir(project_root()?);

    match cli.comamnd {
        cli::Command::Build(cmd) => {
            dbg!(cmd);
            ()
        }
        cli::Command::FuzzTests(cmd) => {
            run_fuzzer(sh)?;
            dbg!(cmd);
        }
        cli::Command::Dist(cmd) => {
            dbg!(cmd);
            ()
        }
        cli::Command::PublishReleaseNotes(cmd) => {
            dbg!(cmd);
            ()
        }
        cli::Command::Metrics(cmd) => {
            dbg!(cmd);
            ()
        }
    };
    println!("Hello from: {} at line {}.", file!(), line!());
    Ok(())
}

fn run_fuzzer(sh: &Shell) -> XtaskResult<()> {
    use anyhow::bail;
    let _d = sh.push_dir("./fuzzer");
    let _e = sh.push_env("RUSTUP_TOOLCHAIN", "nightly");
    if cmd!(sh, "cargo fuzz --help").read().is_err() {
        cmd!(sh, "cargo install cargo-fuzz").run()?;
    };

    // Expecting nightly rustc
    let out = cmd!(sh, "rustc --version").read()?;
    if !out.contains("nightly") {
        bail!("fuzz tests require nightly rustc")
    }

    cmd!(sh, "cargo fuzz run parser").run()?;
    Ok(())
}
