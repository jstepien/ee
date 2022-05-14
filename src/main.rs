use clap::{arg, Arg};
use std::process::exit;
use std::process::Command;

fn opt(arg: Arg) -> Arg {
    arg.required(false)
}

fn main() {
    let matches = clap::Command::new("ee")
        .about("dee-dee but the d is silent")
        .arg(opt(arg!( -b --bs <BYTES> "Copy BYTES bytes at a time")))
        .arg(opt(arg!( -c --count <COUNT> "Copy COUNT times")))
        .arg(opt(arg!( -i --if <INPUT> "Read from INPUT in lieu of stdin")))
        .arg(opt(arg!( -o --of <OUTPUT> "Write to OUTPUT in lieu of stdout")))
        .get_matches();
    let long_names = vec!(
        "bs",
        "count",
        "if",
        "of",
        );
    let status = Command::new("dd")
        .args(
            long_names
                .iter()
                .map(|a| matches.value_of(a).map(|s| format!("{a}={s}")))
                .filter(|s| s.is_some())
                .map(|s| s.unwrap()),
        )
        .status()
        .expect("dd declined");
    exit(status.code().expect("dd died"));
}
