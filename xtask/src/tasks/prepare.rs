use std::{cell::RefCell, fmt::Display};

use clap::Parser;
use xshell::{cmd, Shell};

use crate::helpers::{Runner, XtaskResult};

#[derive(Parser, Debug)]
pub(crate) struct Prepare {
    #[arg(long, short)]
    /// Show cargo ouput
    verbose: bool,
}

impl Prepare {
    pub(crate) fn verbose(&self) -> bool {
        self.verbose
    }
}

impl Runner for Prepare {
    fn run(&self, sh: &Shell) -> XtaskResult<()> {
        println!("{self}:\n");

        const COMMANDS_TO_EXECUTE: [&str; 3] =
            ["cargo check", "cargo build", "cargo build --release"];

        let refcell_sh = RefCell::new(sh);
        dbg!(sh.current_dir());

        // TODO: xshell when inside closure it losts Nix's contexts
        COMMANDS_TO_EXECUTE
            .iter()
            .try_for_each(|to_execute_str| -> XtaskResult<()> {
                let inner_sh = *(refcell_sh.borrow());
                // let cmd = cmd!(sh, "{to_execute_str}");
                let cmd = cmd!(inner_sh, "{to_execute_str}");
                if self.verbose() {
                    cmd.run()?;
                } else {
                    cmd.ignore_stdout().ignore_stderr().run()?;
                }
                Ok(())
            })?;

        Ok(())
    }
}

impl Display for Prepare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "prepare")
    }
}
