use std::fs::File;
use std::io;
use std::path::PathBuf;

use clap::Clap;

use git::object;
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
    HashObject(HashObject),
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
    let repo = GitRepository::default();
    let object = repo.read_object(&args.sha);

    match object {
        Some(o) => {
            println!("Type: {}", o.object_type());
            println!("Size: {}", o.size());
            println!("Content:\n{}", o.content());
        }
        None => {
            println!("Type not supported")
        }
    }
}

#[derive(Clap)]
struct HashObject {
    /// write the object into the object database
    #[clap(short)]
    write: bool,
    /// read the object from stdin
    #[clap(long)]
    stdin: bool,
    #[clap()]
    file: Option<PathBuf>,
}

fn hash_object(args: HashObject) {
    if args.stdin {
        do_hash_object(io::stdin(), args.write);
    }

    if let Some(path) = args.file {
        do_hash_object(File::open(path).unwrap(), args.write);
    }
}

fn do_hash_object(mut input: impl io::Read, write: bool) {
    let mut buf: Vec<u8> = Vec::new();
    input.read_to_end(&mut buf).unwrap();

    let obj = object::GitBlob {
        data: buf,
    };

    let hash = object::hash_object(&obj, write);

    println!("{}", hash);
}

#[derive(Clap)]
struct Add {}

fn main() {
    let opts: Git = Git::parse();

    match opts.subcmd {
        SubCommand::Init { path } => GitRepository::init(path),
        SubCommand::CatFile(args) => cat_file(args),
        SubCommand::HashObject(args) => hash_object(args),
        SubCommand::Add(_add) => println!("add!"),
    }
}
