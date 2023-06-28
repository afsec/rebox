use clap::Parser;
use std::path::PathBuf;

/// cargo xtask
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub comamnd: Command,
}
#[derive(Parser, Debug)]
pub enum Command {
    /// Perform cargo check, build , build --release
    Prep(Prepare),
    /// Compiles release version
    Build(Build),
    /// Run fuzzing tests
    FuzzTests(FuzzTests),
    /// Compiles distribute version
    Dist(Dist),
    /// Prepare release notes to publish
    PublishReleaseNotes(PublishReleaseNotes),
    /// Generate metrics
    Metrics(Metrics),
}

#[derive(Parser, Debug)]
pub struct Prepare {
    #[arg(long, short)]
    /// Show cargo ouput
    verbose: bool,
}

impl Prepare {
    pub fn verbose(&self) -> bool {
        self.verbose
    }
}

#[derive(Parser, Debug)]
pub struct Build {
    #[arg(long, short)]
    /// Enable release
    release: bool,
}

#[derive(Parser, Debug)]
pub struct FuzzTests;

#[derive(Parser, Debug)]
pub struct Dist {
    /// Path to the dist folder
    pub path: PathBuf,
}

#[derive(Parser, Debug)]
pub struct PublishReleaseNotes;

#[derive(Parser, Debug)]
pub struct Metrics;
