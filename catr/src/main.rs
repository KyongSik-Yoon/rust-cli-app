use anyhow::Result;
use clap::Parser;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,

    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

// fn get_args() -> Args {
//     let matches = Command::new("catr")
//         .version("0.1.0")
//         .author("sam")
//         .about("Rust version of `cat`")
//         .arg(
//             Arg::new("files")
//                 .value_name("FILE")
//                 .num_args(1..)
//                 .help("Input file(s)")
//                 .default_value("-")
//         )
//         .arg(
//             Arg::new("number")
//                 .short('n')
//                 .long("number")
//                 .help("Number lines")
//                 .action(ArgAction::SetTrue),
//         )
//         .arg(
//             Arg::new("number_nonblank")
//                 .short('b')
//                 .long("number-nonblank")
//                 .help("Number non-blank lines")
//                 .action(ArgAction::SetTrue),
//         )
//         .get_matches();
//
//     Args {
//         files: matches.get_many("files").unwrap().cloned().collect(),
//         number_lines: matches.get_flag("number"),
//         number_nonblank_lines: matches.get_flag("number_nonblank"),
//     }
// }

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(buf) => {
                let mut line_number = 1;
                for line in buf.lines() {
                    let copied =  line?;
                    if args.number_lines || (args.number_nonblank_lines && !copied.is_empty()) {
                        print!("     {}	", line_number);
                        line_number += 1;
                    }
                    println!("{}", copied);
                }
            },
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}