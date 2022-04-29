use clap::{Command, Arg};


fn main() {
    let matches = Command::new("My RPN program")
        .version("1.0.0")
        .author("As_sqr")
        .about("Super awesome sample RPN calculator")
        .arg(
            Arg::new("formula_file")
                .help("Formulas written in RPN")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .help("Sets the level of verbosity")
                .short('v')
                .long("verbose")
                .required(false),
        )
        .get_matches();

    match matches.value_of("formula_file") {
        Some(file) => println!("Filed specified: {}", file),
        None => println!("No file specified."),
    }

    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified: {}", verbose);
}
