use std::vec;
use regex::Regex;
use clap::{Parser, builder::Str};
use anyhow::{Context, Result, Ok};

struct Eml {
    mime_version: String,
    date: String,
    subject: String,
    from: String,
    to: String,
    boundary: String, 
    body: String
}

fn find_matches(content: &str, pattern: &str) {
    for line in content.lines() {
        if line.starts_with(pattern) {
            return line.into( ));
        }
    }
}

fn get_boundary(content: &str) {
    let re = Regex::new(r"boundary=\"(.+)\"").unwrap();
    




}



fn handle_contents(content){

    let email: Eml = Eml {

        mime_version = find_matches(content, "MIME-Version:"), 
        date = find_matches(content, "Date:"),
        subject = find_matches(content, "Subject:"),
        from = find_matches(content, "From:"),
        to = find_matches(content, "To:"),





    }
    
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
        .with_context(|| format!("Could not read file '{}'",
         args.path.display()))?;

    println!("{}", content);

    // println!("is this the content length ? {x}");   

    let email = Eml {
        mime_version: handle_line(string, pattern)
    }

    let mut counter = 0; 


    for line in content.lines(){
        //if line.contains(&args.pattern){
        //    println!("{}", line);
       // }
       println!("{}", line);

       println!("{}", counter);
       counter = counter + 1; 
    }   

    Ok(())

}
