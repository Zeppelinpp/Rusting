use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::search;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args); [0] contains target path

    // let query = &args[1];
    // let file_path = &args[2];
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    // println!("With text:\n{contents}");
    if let Err(e) = run(config) {
        eprintln!("Application Error at: {}", e);
        process::exit(1)
    }
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            // panic!("not enough arguments");
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // println!("With text:\n{contents}");
    for line in search(&config.query, &contents, config.ignore_case) {
        println!("{line}");
    }
    Ok(())
}
