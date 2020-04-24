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
    CatFile(CatFile),
    /// Add file contents to the index
    #[clap(name = "add")]
    Add(Add),
}

#[derive(Clap)]
struct CatFile {
    #[clap(name = "SHA")]
    sha: String,
}

fn cat_file(args: CatFile) {
    GitRepository::read_object(&args.sha);
}

#[derive(Clap)]
struct Add {}

fn main() {
    let opts: Git = Git::parse();

    match opts.subcmd {
        SubCommand::Init { path } => GitRepository::init(path),
        // SubCommand::CatFile { sha } => GitRepository::read_object(&sha),
        SubCommand::CatFile(args) => cat_file(args),
        SubCommand::Add(_add) => println!("add!"),
    }
}
