pub fn get_arg_matches<'a>() -> clap::ArgMatches<'a> {
    return clap::App::new("powerline-shell")
        .version("0.0.4")
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
                .short("d")
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
        .arg(
            clap::Arg::with_name("width")
                .help("tput cols")
                .short("w")
                .long("width")
                .required(false)
                .default_value("200")
                .takes_value(true),
        )
        .get_matches();
}
