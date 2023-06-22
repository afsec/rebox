use rebox_types::ReboxResult;

fn main() -> ReboxResult<()> {
    use clap::Parser;

    let cli = cli::Args::parse();

    if let Some(command) = cli.command {
        run(command)?;
    } else {
        run(cli::Command::default())?;
    }
    Ok(())
}

fn run(command: cli::Command) -> ReboxResult<()> {
    match command {
        cli::Command::Hello(cmd) => {
            dbg!(cmd);
        }
        cli::Command::Todo(cmd) => {
            dbg!(cmd);
        }
    }
    Ok(())
}

mod cli {
    use clap::Parser;

    /// A tool to help with publishing crates
    #[derive(Parser, Debug)]
    pub struct Args {
        #[command(subcommand)]
        pub command: Option<Command>,
    }
    #[derive(Parser, Debug)]
    pub enum Command {
        /// Simple Hello
        Hello(Hello),
        /// Simple Todo
        Todo(Todo),
    }
    impl Default for Command {
        fn default() -> Self {
            Self::Hello(Default::default())
        }
    }
    #[derive(Parser, Debug, Default)]
    pub struct Hello {
        /// Disable println-like messagess
        #[arg(long, short)]
        pub quiet: bool,
    }
    #[derive(Parser, Debug)]
    pub struct Todo {
        /// Disable println-like messagess
        #[arg(long, short)]
        pub quiet: bool,
    }
}
