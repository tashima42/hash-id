extern crate clap;

use clap::{Arg, ArgAction, Command};

use hash_id::{Config, run};

fn main() {
    let matches = Command::new("Hash Identifier")
        .version("0.1.0")
        .author("Pedro Tashima <pedrotashima@protonmail.com>")
        .about("Identify different types of hashes")
        .arg(
            Arg::new("hash")
                .long("hash")
                .value_name("STRING")
                .help("Hash value to be identified")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("file")
                .long("file")
                .value_name("FILE")
                .help("File containing hashes (each one in a line)")
                .action(ArgAction::Set),
        )
        .get_matches();

    let input_default = String::new();
    let hash = matches.get_one::<String>("hash").unwrap_or(&input_default);
    let file = matches.get_one::<String>("file").unwrap_or(&input_default);

    let config = Config::new(hash.to_string(), file.to_string());
    run(config);
}
