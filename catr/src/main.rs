use clap::{Arg, ArgAction, Command, Parser};

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

fn main() {
    let args = Args::parse();
    println!("{args:#?}");
}
