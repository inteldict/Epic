extern crate regex;
extern crate url;

use regex::Regex;
use std::env;
use std::error::Error;
use std::fs::OpenOptions;
//use std::io;
use std::io::Write;
//use std::process;
use std::process::{Command, Stdio};

//use std::io::Read;

//use std::fs::File;
//use std::io;
//use std::io::prelude::*;
//use std::io::Write;
//use std::env;
//use std::collections::btree_map::BTreeMap;
//
//fn write_stderr( msg : String ) {
//    let mut stderr = std::io::stderr();
//    write!(&mut stderr, "{}", msg).unwrap();
//}
//
//fn write_stderr_s( msg : &str ) {
//    write_stderr( msg.to_string() );
//}
//
//fn write_stdout( msg : String ) {
//    let mut stdout = std::io::stdout();
//    write!(&mut stdout, "{}", msg).unwrap();
//}
//
//fn write_stdout_s( msg : &str ) {
//    write_stdout( msg.to_string() );
//}
//
//fn html_escape( msg : String ) -> String {
//    let mut copy : String = String::with_capacity( msg.len() );
//
//    for thechar in msg.chars() {
//        if thechar == '&' {
//            copy.push_str( "&amp;" );
//        } else if thechar == '<' {
//            copy.push_str( "&lt;" );
//        } else if thechar == '>' {
//            copy.push_str( "&gt;" );
//        } else if thechar == '\"' {
//            copy.push_str( "&quot;" );
//        } else {
//            copy.push( thechar );
//        }
//    }
//
//    return copy;
//}
//
//fn main() {
//    write_stdout_s( "Content-type: text/html\n" );
//    write_stdout_s( "\n" );
//    write_stdout_s( "<html>\n" );
//    write_stdout_s( "  <head>\n" );
//    write_stdout_s( "    <title>Rust CGI Test</title>\n" );
//    write_stdout_s( "    <style type=\"text/css\">\n" );
//    write_stdout_s( "      td { border:1px solid black; }\n" );
//    write_stdout_s( "      td { font-family:monospace; }\n" );
//    write_stdout_s( "      table { border-collapse:collapse; }\n" );
//    write_stdout_s( "    </style>\n" );
//    write_stdout_s( "  </head>\n" );
//    write_stdout_s( "  <body>\n" );
//    write_stdout_s( "    <h1>Environment</h1>\n" );
//    write_stdout_s( "    <table>\n" );
//    write_stdout_s( "      <tr><th>Key</th><th>Value</th></tr>\n" );
//
//    // copy environment into a BTreeMap which is sorted
//    let mut sortedmap : BTreeMap<String,String> = BTreeMap::new();
//    for (key, value) in env::vars() {
//        sortedmap.insert( key, value );
//    }
//
//    // output environment into HTML table
//    for (key, value) in sortedmap {
//        write_stdout(
//            format!(
//                "      <tr><td>{}</td><td>{}</td></tr>\n",
//                html_escape( key ),
//                html_escape( value )
//            )
//        );
//    }
//    write_stdout_s( "    </table>\n" );
//    write_stdout_s( "  </body>\n" );
//    write_stdout_s( "</html>\n" );
//
////fn read_username_from_file() -> Result<String, io::Error> {
////    let f = File::open("hello.txt");
////
////    let mut f = match f {
////        Ok(file) => file,
////        Err(e) => return Err(e),
////    };
////
////    let mut s = String::new();
////
////    match f.read_to_string(&mut s) {
////        Ok(_) => Ok(s),
////        Err(e) => Err(e),
////    }
////}
//


pub struct Config {
    input: String,
    num: u32,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let query = match env::var("QUERY_STRING") {
            Ok(val) => val,
            Err(_) => return Err("Can't find 'QUERY_STRING' variable".to_string()),
        };

        if query.is_empty() {
            return Err("Query string is empty".to_string());
        }

        let parse: Vec<(String, String)> = url::form_urlencoded::parse(query.as_bytes()).into_owned().collect();

        if parse.len() != 2 {
            return Err(format!("Expected two query parameters, got: {}", parse.len()));
        }

        let mut parameter_values: (String, String) = (String::new(), String::new());
        for (key, value) in parse {
            match &key[..] {
                "s" => parameter_values.0 = value,
                "num" => parameter_values.1 = value,
                _ => return Err(format!("Unknown parameter {}:{}", key, value)),
            }
        }

        let num = match parameter_values.1.parse::<u32>() {
            Ok(n) => n,
            Err(_) => return Err("Error during parsing 'num' parameter".to_string()),
        };

        Ok(Config {
            input: parameter_values.0,
            num,
        })
    }
}

fn log(err: String) -> Result<(), Box<Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("epic.log")?;
    writeln!(&mut file, "{}", err)?;
    Ok(())
}

fn parse_input(config: &Config) -> Result<String, Box<Error>> {
    let mut parser = Command::new("./parser/bitpar")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("-s")
        .arg("TOP")
        .arg("-v")
        .arg("parser/grammar")
        .arg("parser/lexicon")

//        .arg("-ts '()' -s TOP -v parser/grammar parser/lexicon -u parser/open-class-tags -w parser/wordclass.txt")
//        .args(&["-ts", "'()'", "-s", "TOP", "-v", "grammar", "lexicon", "-u", "open-class-tags", "-w", "wordclass.txt"] )
//        .args(&["-ts '()'", "-s TOP", "-v", "parser/grammar", "parser/lexicon", "-u parser/open-class-tags", "-w parser/wordclass.txt"] )
        .spawn()?;

    let re = Regex::new(r"(?P<s>[?!.,:;\(\)\[\]/\\])")?;
    // transform query string to one word/punctuation character per line
    let parser_input: String = re.replace_all(&config.input, " $s").split_whitespace().map(|line| format!("{}\n", line)).collect();

    {
       let stdin = parser.stdin.as_mut().unwrap();
       stdin.write_all(parser_input.as_bytes())?;
    }

    let output = parser.wait_with_output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn main() {
    print!("Content-type: text/plain\n\n");

    let config = match Config::new() {
        Ok(arg) => arg,
        Err(e) => {
            log(e).unwrap();
            return;
        }
    };

    println!("input: {}\nnum: {}", config.input, config.num);
    let out = match parse_input(&config) {
        Ok(val) => val,
        Err(e) => {
            log(format!("Error during parsing {}", e)).unwrap();
            return;
        }
    };
    print!("{}", out);
}
