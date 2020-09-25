//! SimpleCI Repository Observer

use clap::{App, Arg};

fn main() {
    // let matches = App::new("Observer")
    //     .about("simple-ci observer")
    //     .arg(
    //         Arg::with_name("dispatcher-server")
    //             .short("d")
    //             .long("dispatcher-server")
    //             .value_name("DISPATCHER_SERVER")
    //             .help("Sets the dispatcher server. Defaults to localhost:8888")
    //             .takes_value(true),
    //     )
    //     .arg(
    //         Arg::with_name("repo")
    //             .required(true)
    //             .short("r")
    //             .long("repo")
    //             .value_name("REPO")
    //             .help("Sets the target repo.")
    //             .takes_value(true),
    //     )
    //     .get_matches();

    // let server = matches
    //     .value_of("dispatcher_server")
    //     .unwrap_or("localhost:8888");

    // let repo = matches.value_of("repo").unwrap();

    // println!("{} {}", server, repo);

    if let Err(e) = cimple::observer::poll("../ci-clone/") {
        println!("Error: {}", e);
    }
    // observer::notify();
}