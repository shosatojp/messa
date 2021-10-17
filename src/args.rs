use clap::{crate_name, crate_version};

pub fn get_arg_matches<'a>() -> clap::ArgMatches<'a> {
    return clap::App::new(crate_name!())
        .version(crate_version!())
        .arg(
            clap::Arg::with_name("home")
                .help("specify home directory")
                .short("h")
                .long("home")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("pwd")
                .help("specify current working directory")
                .short("d")
                .long("pwd")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("error")
                .help("previous error code")
                .short("e")
                .long("error")
                .required(true)
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
        .arg(
            clap::Arg::with_name("user")
                .help("overwrite username")
                .short("u")
                .long("user")
                .required(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("host")
                .help("hostname")
                .short("i")
                .long("host")
                .required(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("kubeconfig")
                .help("kubernetes config file")
                .short("k")
                .long("kube")
                .default_value("~/.kube/config")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("config")
                .help("config file")
                .short("c")
                .long("config")
                .default_value("~/.messa.yaml")
                .takes_value(true),
        )
        .get_matches();
}
