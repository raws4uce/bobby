use dark::server::{Engine, KvsServer};

use clap::{command, Arg, ArgMatches};



fn main() {
    //args stuff
    let args: ArgMatches = command!()
        .arg(Arg::new("server").default_value("kvs"))
        .arg(Arg::new("port").default_value("127.0.0.1:4000"))
        .get_matches();

    //parses as Strings
    let engine: &String = args.get_one("server").unwrap();
    let port: &String = args.get_one("port").unwrap();

    //checks engine is cool
    let engine_parsed = match engine.parse::<Engine>() {
        Ok(engine) => engine,
        Err(_e) => {
            panic!("ONLY OPTIONS, SLED AND KVS")
        }
    };


    // info!(_log, "SKEPTA AN AN PLASTICIAN");
    // warn!(_log, "ARE YOU shtupid");

    // info!(_log, "THERES NO INTRODUCTION NEEDED"; arguments);

    // info!(_log, "Connection established to port"; o!("addy" => port));

    let server = KvsServer::new(engine_parsed);

    let _init_addr = server.run(port);

    // let string = kv::ting();
    // info!(_log, "{}", string);

    // follow through with request(append log or return value)
    // close connection
    // append log
    // return a value
}
