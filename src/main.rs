use clap::{Arg, Command};
use crate::commands::crtusr;

#[path ="./commands"]
mod commands {
    pub mod crtusr;
}
mod db;

#[tokio::main]
async fn main() {
    let database = db::connect().await.expect("Failed to connect to database.");

    let matches = Command::new("LemCom-CLI")
        .about("A simple CLI for administrative tasks involving lemcom-api.")
        .subcommand(
            Command::new("crtusr")
                .about("Creates a new user")
                .arg(Arg::new("username")
                     .help("Specifies the username for the new user")
                     .required(true)
                     .index(1))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("crtusr", sub_m)) => {
            let username = sub_m.get_one::<String>("username").unwrap();
            if let Err(e) = crtusr::execute(database, username).await {
                println!("Failed to generate user: {}", e);
            }
        },
        _ => println!("Invalid command or missing arguments"),
    }
}
