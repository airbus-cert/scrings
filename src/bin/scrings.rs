use clap::{App, Arg};
use scrings::bash::Bash;
use scrings::js::Javascript;
use scrings::parser::IterScrings;
use scrings::php::Php;
use scrings::ps::Powershell;
use scrings::python::Python;
use scrings::sql::Sql;
use scrings::strings::Utf16le;
use std::fs::File;
use std::io::BufReader;

const APPLICATION_NAME: &str = "scrings";

fn main() {
    let matches = App::new(APPLICATION_NAME)
        .version("0.1.0")
        .author("Airbus CERT <cert@airbus.com>")
        .arg(
            Arg::with_name("path")
                .long("path")
                .short("p")
                .takes_value(true)
                .help("Path to the script file"),
        )
        .arg(
            Arg::with_name("step")
                .long("step")
                .short("s")
                .takes_value(true)
                .help("Min length")
                .default_value("20"),
        )
        .arg(
            Arg::with_name("language")
                .long("language")
                .short("l")
                .takes_value(true)
                .possible_values(&["powershell", "bash", "python", "sql", "javascript", "php"])
                .help("Language to match"),
        )
        .arg(Arg::with_name("bash").help("bash"))
        .arg(
            Arg::with_name("escape")
                .long("escape")
                .help("Escape string before print"),
        )
        .arg(
            Arg::with_name("offset")
                .long("offset")
                .short("o")
                .help("Print offset in file"),
        )
        .get_matches();

    let file = File::open(
        matches
            .value_of("path")
            .expect("Path argument is mandatory"),
    )
    .unwrap();
    let buf_reader = BufReader::new(file);

    let language = matches
        .value_of("language")
        .expect("language argument is mandatory");

    let step = matches
        .value_of("step")
        .expect("Step argument is mandatory")
        .parse::<usize>()
        .unwrap();

    match language {
        "powershell" => {
            for (offset, str) in buf_reader.iter_scrings::<Utf16le, Powershell>(step) {
                if matches.is_present("offset") {
                    print!("{}\t", offset)
                }
                if matches.is_present("escape") {
                    println!("{}", str.trim().escape_default());
                } else {
                    println!("{}", str.trim());
                }
            }
        }
        "bash" => {
            for (offset, str) in buf_reader.iter_scrings::<u8, Bash>(step) {
                if matches.is_present("offset") {
                    print!("{}\t", offset)
                }
                if matches.is_present("escape") {
                    println!("{}", str.trim().escape_default());
                } else {
                    println!("{}", str.trim());
                }
            }
        }
        "python" => {
            for (offset, str) in buf_reader.iter_scrings::<u8, Python>(step) {
                if matches.is_present("offset") {
                    print!("{}\t", offset)
                }
                if matches.is_present("escape") {
                    println!("{}", str.trim().escape_default());
                } else {
                    println!("{}", str.trim());
                }
            }
        }
        "sql" => {
            for (offset, str) in buf_reader.iter_scrings::<u8, Sql>(step) {
                if matches.is_present("offset") {
                    print!("{}\t", offset)
                }
                if matches.is_present("escape") {
                    println!("{}", str.trim().escape_default());
                } else {
                    println!("{}", str.trim());
                }
            }
        }
        "javascript" => {
            for (offset, str) in buf_reader.iter_scrings::<u8, Javascript>(step) {
                if matches.is_present("offset") {
                    print!("{}\t", offset)
                }
                if matches.is_present("escape") {
                    println!("{}", str.trim().escape_default());
                } else {
                    println!("{}", str.trim());
                }
            }
        }
        "php" => {
            for (offset, str) in buf_reader.iter_scrings::<u8, Php>(step) {
                if matches.is_present("offset") {
                    print!("{}\t", offset)
                }
                if matches.is_present("escape") {
                    println!("{}", str.trim().escape_default());
                } else {
                    println!("{}", str.trim());
                }
            }
        }
        _ => (),
    }
}
