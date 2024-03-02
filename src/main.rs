use regex::Regex;
use clap::{Parser};
use anyhow::{Context, Result, Ok};

fn get_boundary(content: &str) -> &str {
    let re = Regex::new(r"(boundary=)").unwrap();

    let Some(caps) = re.captures(content) else {
        println!("no match!");
        panic!()
    };
    let res = &caps[0];
    res
}

///This converts eml to markdown
#[derive(Parser)]
struct Cli {
    /// the pattern to look for
    pattern: String,
    /// the file
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file '{}'" , args.path.display()))?;

    let boundary = get_boundary(&content);

    println!("the boundary is {}", boundary);
    
    Ok(())

}
