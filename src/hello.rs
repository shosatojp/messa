fn main() -> Result<(), &'static str> {
    let s = hoge().to_owned();

    let pwd = s.value_of("hoge").ok_or("")?;

    println!("{}", pwd);

    return Ok(());
}

fn hoge() -> clap::ArgMatches<'static> {
    return clap::App::new("hoge").get_matches();
}
