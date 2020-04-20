use std::path::PathBuf;

use clap::Clap;

use git::repo::GitRepository;

/// git implemented in rust.
#[derive(Clap)]
struct Git {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// Create an empty Git repository or reinitialize an existing one
    Init {
        #[clap(name = "PATH", default_value = ".", parse(from_os_str))]
        path: PathBuf,
    },
    /// Add file contents to the index
    #[clap(name = "add")]
    Add(Add),
}

#[derive(Clap)]
struct Add {}

fn main() {
    let opts: Git = Git::parse();

    match opts.subcmd {
        SubCommand::Init { path } => GitRepository::init(path),
        SubCommand::Add(_add) => println!("add!"),
    }
}