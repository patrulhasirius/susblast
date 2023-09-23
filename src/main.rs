mod scripts;

use crate::scripts::get_file;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    month: String,
    vars: std::path::PathBuf,
    temp_path: std::path::PathBuf,
    output: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let vars_file = std::fs::read_to_string(&args.vars).expect("erro no arquivo de vars");
    let mut vars: Vec<String> = vec![];
    for line in vars_file.lines() {
        if !line.is_empty() {
            vars.push(line.trim().to_string());
        }
    }
    println!("{:#?}", args);
    println!("{:#?}", vars.clone());
    println!("Now let's see how well you fare against my BIDEN BLAST!!!");
    get_file(args.month, vars, args.temp_path, args.output);
}
