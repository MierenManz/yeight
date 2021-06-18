use clap::App;
use clap::Arg;
use yeight_http::
fn main() {
    let config = get_config();
}

fn get_config() -> Config {
    let app = App::new("Yeight")
        .version("0.1.0")
        .author("Skyler van Boheemen")
        .arg(Arg::with_name("ws port").long("ws-port").takes_value(true))
        .arg(
            Arg::with_name("http port")
                .long("http-port")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("write to db")
                .long("write-to-db")
                .takes_value(true),
        );

    let args = app.get_matches();

    Config {
        websocket_port: match args.value_of("ws port") {
            Some(value) => value.parse::<u16>().unwrap(),
            None => 80,
        },
        http_port: match args.value_of("http port") {
            Some(value) => value.parse::<u16>().unwrap(),
            None => 80,
        },
        fs_directory: match args.value_of("write to disk") {
            Some(thing) => Some(thing.to_string()),
            None => None,
        },
    }
}

struct Config {
    pub websocket_port: u16,
    pub http_port: u16,
    pub fs_directory: Option<String>,
}
