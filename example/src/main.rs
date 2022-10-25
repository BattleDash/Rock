use rock::*;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::init();

    info!("Starting up");

    let mut host_str: String = String::new();
    host_str += &rock_core::cli::cmd_option_default("host", "127.0.0.1");
    host_str += ":";
    host_str += &rock_core::cli::cmd_option_default("port", "25565");
    info!("Host: {}", host_str);

    if rock_network::network_handler::start(host_str.as_str()).is_err() {
        error!("Failed to start");
    }
}
