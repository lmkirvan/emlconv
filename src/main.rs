use regex::Regex;
use clap::{Parser};
use anyhow::{Context, Result, Ok};

fn get_boundary(content: &str) -> String {
    let re = Regex::new("(boundary=)\"(?<b>[a-z,0-9]+)").unwrap();

    let Some(caps) = re.captures(content) else {
        println!("no match!");
        panic!()
    };

    [ "--", &caps["b"].to_string() ].concat()
}

fn get_line_by_lead(content: String, lead: &str) -> String {
    let mut matches = content.lines()
        .filter(|x| x.starts_with(lead));

    let some_date = matches.next();
    
    let result = match some_date {
        Some(_) => {some_date},
        None => {Some("Date: Unkown")}, 
    };

   result.expect("no date somehow").to_string()

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
    
    let date: String = get_line_by_lead(content, "Date:");

    println!("the boundary is {} and the line is {}", boundary, date);
    
    Ok(())

}
