use ansi_term::Colour;
use clap::{Arg, Command};

pub fn create_cli() {
    let matches = Command::new("test")
        .version("1.0.0")
        .author("ssss")
        .about("test")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Provides a config file to myprog"),
        )
        .get_matches();

    match matches.get_one::<String>("file") {
        Some(f) => {
            println!("{}", Colour::Red.bold().paint(f));
            println!("{}", Colour::Red.paint(f).to_string() + "324");
        }
        None => {}
    }
}
