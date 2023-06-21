use rebox_types::ReboxResult;

fn main() -> ReboxResult<()> {
    use clap::Parser;
    println!("Hello from: {} at line {}.", file!(), line!());
    let cli = cli::Args::parse();

    match cli.comamnd {
        cli::Command::Hello(cmd) => {
            dbg!(cmd);
        }
        cli::Command::Todo(cmd) => {
            dbg!(cmd);
        }
    };
    Ok(())
}

mod cli {
    use clap::Parser;

    /// A tool to help with publishing crates
    #[derive(Parser, Debug)]
    pub struct Args {
        #[command(subcommand)]
        pub comamnd: Command,
    }
    #[derive(Parser, Debug)]
    pub enum Command {
        /// Simple Hello
        Hello(Hello),
        /// Simple Todo
        Todo(Todo),
    }
    #[derive(Parser, Debug)]
    pub struct Hello {
        /// No println-like messages
        #[arg(long, short)]
        pub quiet: bool
        
    }
    #[derive(Parser, Debug)]
    pub struct Todo {
        /// No println-like messages
        #[arg(long, short)]
        pub quiet: bool
        
    }
}
