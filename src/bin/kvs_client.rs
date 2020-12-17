/// KVS Client
use clap::{App, AppSettings, Arg, SubCommand};

use kvs::{client::KvsClient, Result};

const DEFAULT_SERVER_ADDR: &str = "127.0.0.1:4000";

fn main() -> Result<()> {
    env_logger::init();

    let matches = App::new("kvs-client")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("KVS Client")
        .arg(Arg::with_name("V").short("V").help("Print version info"))
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .value_name("IP-PORT")
                .default_value(DEFAULT_SERVER_ADDR)
                .help("Server address"),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Set a key and value")
                .arg(Arg::with_name("key"))
                .arg(Arg::with_name("value")),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get value with specified key")
                .arg(Arg::with_name("key")),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove the the specified key")
                .arg(Arg::with_name("key")),
        )
        .get_matches();

    // If version was requested, print it and return
    if matches.is_present("V") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let addr: String = matches.value_of("addr").unwrap().to_owned();
    let mut client = KvsClient::new(&addr)?;

    match matches.subcommand() {
        ("set", sub_match) => {
            let key = sub_match.unwrap().value_of("key").unwrap().to_owned();
            let value = sub_match.unwrap().value_of("value").unwrap().to_owned();
            client.set(key, value)?;
        }
        ("get", sub_match) => {
            let key = sub_match.unwrap().value_of("key").unwrap().to_owned();
            let value = client.get(key)?;

            if let Some(value) = value {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        ("rm", sub_match) => {
            let key = sub_match.unwrap().value_of("key").unwrap().to_owned();
            client.remove(key)?;
        }
        (_, _) => {
            panic!("Unexpected subcommand");
        }
    }

    Ok(())
}
