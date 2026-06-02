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
        let ignore_case = env::var("IGNORE_CASE").map(|v| v == "1").unwrap_or(false);

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let dim = "\x1b[2;37m";
    let reset = "\x1b[0m";

    for (line_no, col, line) in search(&config.query, &contents, config.ignore_case) {
        print!("{dim}[{}:{}:{}]{reset}: ", config.file_path, line_no, col);
        print_highlighted(line, &config.query, config.ignore_case);
    }
    Ok(())
}

fn print_highlighted(line: &str, query: &str, ignore_case: bool) {
    let cyan = "\x1b[38;2;102;217;239m";
    let reset = "\x1b[0m";

    if query.is_empty() {
        println!("{line}");
        return;
    }

    let line_cmp = if ignore_case { line.to_lowercase() } else { line.to_string() };
    let query_cmp = if ignore_case { query.to_lowercase() } else { query.to_string() };

    let mut start = 0;
    while let Some(pos) = line_cmp.get(start..).and_then(|s| s.find(&query_cmp)) {
        let match_start = start + pos;
        let match_end = match_start + query.len();
        print!("{}", &line[start..match_start]);
        print!("{cyan}{}{reset}", &line[match_start..match_end]);
        start = match_end;
    }
    println!("{}", &line[start..]);
}
