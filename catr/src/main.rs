use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn get_args() -> Args {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("sam")
        .about("Rust version of `cat`")
        .arg(
            Arg::new("files")
                .required(true)
                .value_name("FILE")
                .num_args(0..)
                .help("Input file(s)")
                .default_value("-"),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("Number lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("Number non-blank lines")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    Args {
        files: matches.get_many("files").unwrap().cloned().collect(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank_lines"),
    }
}

fn main() {
    let args = get_args();
    println!("{args:#?}");
}
