use clap::{command, Arg, ArgMatches, Command};
use dark::client::KvsClient;

fn main() {
    let args: ArgMatches = command!()
        .subcommand(
            Command::new("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::new("KEY").help("A string key").required(true))
                .arg(
                    Arg::new("VALUE")
                        .help("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("get")
                .about("Get the string value of a given string key")
                .arg(Arg::new("KEY").help("A string key").required(true)),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a given key")
                .arg(Arg::new("KEY").help("A string key").required(true)),
        )
        .arg(Arg::new("addr").default_value("127.0.0.1:4000"))
        .get_matches();

    let addr: &String = args.get_one("addr").unwrap();
    let client: KvsClient = KvsClient::connect(addr).unwrap();

    match args.subcommand() {
        Some(("set", sub_m)) => {
            let key: &String = sub_m.get_one("KEY").unwrap();
            let value: &String = sub_m.get_one("VALUE").unwrap();

            println!("this is the args, {:?}, {:?}, {}", key, value, addr);
            client.set(key, value);
        }
        Some(("get", sub_m)) => {
            let key: &String = sub_m.get_one("KEY").unwrap();
            println!("this is the args, {:?}, {:?}", key, addr);

            println!("Your beloved value {:?}", client.get(key));
        }
        Some(("rm", sub_m)) => {
            let key: &String = sub_m.get_one("KEY").unwrap();
            println!("this is the args, {:?}, {:?}", key, addr);

            client.rm(key);
        }
        _ => unreachable!(),
    }
}
