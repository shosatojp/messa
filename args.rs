pub fn get_arg_matches() -> clap::ArgMatches<'static> {
    return clap::App::new("powerline-shell")
        .version("0.1.0")
        .arg(
            clap::Arg::with_name("home")
                .help("specify home directory")
                .short("h")
                .long("home")
                .required(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("pwd")
                .help("specify current working directory")
                .short("w")
                .long("pwd")
                .required(false)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("error")
                .help("previous error code")
                .short("e")
                .long("error")
                .required(false)
                .default_value("0")
                .takes_value(true),
        )
        .get_matches();
}
