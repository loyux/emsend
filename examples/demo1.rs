use std::path::PathBuf;

use clap::{arg, command, value_parser, Parser};
pub fn main() {
    let matches = command!()
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();
    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        println!("{:?}", config_path.to_str().unwrap());
    }
}
