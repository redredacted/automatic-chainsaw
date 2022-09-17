use clap::{arg, command, Command, ArgMatches};
// use surrealdb::Datastore;

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Adds files to myapp")
                .arg(arg!([TITLE]))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => add(sub_matches.get_one::<String>("TITLE").unwrap_or(&String::from(""))),
        _ => unreachable!("no matching commands try --help"),
    }
}

fn add(title: &str) {
    println!("'todoctl add' was called, with title: {}", title)
}
