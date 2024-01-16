use std::path::PathBuf;
use clap::Parser;

///convert eml to markdown
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path to eml files 
    #[arg(short, long)]
    path: Vec<PathBuf>,

    /// subject keywords to look for
    #[arg(short, long)]
    subject: Vec<String>,
}

fn main() {
    let args = Args::parse();

    for arg in args.path {
        println!("Hello {}!", arg.display())
    }
}