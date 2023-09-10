mod settings;

extern crate eposlib;
extern crate regex;
extern crate url;

use std::env;
use std::error::Error;
use std::fmt;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::Arc;

use eposlib::cky::ParserOutput;
use eposlib::config::Config as EPOS_CONFIG;
use eposlib::lm;
use regex::Regex;

use settings::AppSettings;


static QUERY_STRING: &str = "QUERY_STRING";


#[derive(Debug)]
struct QueryArgs {
    words: Vec<Box<str>>,
    num: usize,
}

impl Error for QueryArgs {}

impl fmt::Display for QueryArgs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "QueryArgs error")
    }
}

impl QueryArgs {
    pub fn new() -> Result<QueryArgs, Box<dyn Error>> {
        let query = env::var(QUERY_STRING).map_err(|_| "Can't find 'QUERY_STRING' variable".to_string())?;
        if query.is_empty() {
            return Err("Query string is empty".to_string().into());
        }

        let parse: Vec<(String, String)> = url::form_urlencoded::parse(query.as_bytes()).into_owned().collect();
        if parse.len() != 2 {
            return Err(format!("Expected two query parameters, got: {}", parse.len()).into());
        }

        let mut parameter_values = (String::new(), String::new());
        for (key, value) in &parse {
            match key.as_str() {
                "s" => parameter_values.0 = value.clone(),
                "num" => parameter_values.1 = value.clone(),
                _ => return Err(format!("Unknown parameter {}:{}", key, value).into()),
            }
        }

        let num = parameter_values.1.parse::<usize>().map_err(|_| "Error during parsing 'num' parameter".to_string())?;

        let re = Regex::new(r"(?P<s>[?!.,:;\(\)\[\]/\\])").unwrap();
        // transform query string to one word/punctuation character per line
        let parser_input: Vec<Box<str>> = re.replace_all(&parameter_values.0, " $s")
            .split_whitespace()
            .map(|line| line.to_string().into_boxed_str())
            .collect();

        Ok(QueryArgs {
            words: parser_input,
            num,
        })
    }
}


fn log_error<T: fmt::Display>(err: T) {
    if let Err(io_err) = writeln!(io::stderr(), "Error: {}", err) {
        eprintln!("Failed to log error: {}", io_err);
    }
}

fn log_to_file<T: fmt::Display>(filename: &str, err: T) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filename)?;
    writeln!(&mut file, "Error: {}", err)?;
    Ok(())
}


fn main() {
    print!("Content-type: text/plain\n\n");

    if let Err(err) = parse() {
        if let Err(io_err) = log_to_file("epic.log", &err) {
            eprintln!("Failed to log error to file: {}", io_err);
            log_error(&err);
        }
    }
}

fn parse() -> Result<(), Box<dyn Error>> {
    let query_args = QueryArgs::new()?;
    // let parser_args: Vec<String> = vec!["-load", "/opt/parsing_data/evalb/step211212/full.lm", "-s", "VROOT"]
    //     .iter()
    //     .map(|s| s.to_string())
    //     .collect();

    let app_settings = AppSettings::new()?;
    let config = EPOS_CONFIG::new(app_settings.parser_args.parser_init_args.into_iter())?;
    let lm = Arc::new(lm::load_model(&config)?);

    let parses: Vec<ParserOutput> = eposlib::parse_standard(query_args.words, &None, lm, query_args.num, false)?;

    for parse in &parses {
        println!("{}", parse.parse);
    }
    Ok(())
}