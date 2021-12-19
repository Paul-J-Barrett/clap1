use clap::{App, Arg};

fn main() {
    let matches = App::new("clap 1 ")
        .author("Paul Barrett")
        .version("1.0")
        .about("Do awesome stuff")
        .arg(
            Arg::new("Config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("sets a custom config")
                .takes_value(true),
        )
        .arg(
            Arg::new("INPUT")
                .help("sets the input file")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = matches.value_of("INPUT").unwrap();
    println!("{}", file);
}
