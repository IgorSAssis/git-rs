use clap::Parser;
use clap::Subcommand;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize an empty git repository
    Init,
    /// Prints the content of a object file
    CatFile {
        /// Pretty print the contents of object file
        #[arg(short = 'p')]
        pretty_print: bool,
        object_hash: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => match Path::new(".git/").try_exists() {
            Ok(false) => {
                fs::create_dir(".git").unwrap();
                fs::create_dir(".git/objects").unwrap();
                fs::create_dir(".git/refs").unwrap();
                fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
                println!("Initialized git directory");
            }
            _ => panic!("Cannot initialize .git directory"),
        },
        _ => println!("Todo"),
    }

    // let args: Vec<String> = env::args().collect();
    // if args[1] == "init" {
    //     match Path::new(".git/").try_exists() {
    //         Ok(false) => {
    //             fs::create_dir(".git").unwrap();
    //             fs::create_dir(".git/objects").unwrap();
    //             fs::create_dir(".git/refs").unwrap();
    //             fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    //             println!("Initialized git directory");
    //         }
    //         _ => panic!("Cannot initialize .git directory"),
    //     }
    // } else if args[1] == "cat-file" {
    //     let arg = args.get(2).expect("Required argument not informed");
    //     let hash = args.get(3).expect("Object hash not informed");
    //     let path = format!(".git/objects/{}/{}", &hash[..2], &hash[2..]);

    //     if arg == "-p" {
    //         let blob_file = File::open(&path).expect("Cannot open file");
    //         let mut decoder = ZlibDecoder::new(BufReader::new(blob_file));

    //         let mut buffer = String::new();
    //         let _ = decoder.read_to_string(&mut buffer);
    //
    //         let content: Vec<&str> = buffer.split("\0").collect();
    //         print!("{}", content.get(1).unwrap());
    //     }
    // } else {
    //     println!("unknown command: {}", args[1])
    // }
}
