use cargo_ws_manage::{config::Config, run};
use clap::Parser;

fn main() {
    // Get command line arguments and if first element is "ws-manage", remove it
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.get(0).and_then(|s| Some(s.as_str())) == Some("ws-manage") {
        args.remove(0);
    }

    // use the potentially modified argument list
    let cfg = Config::parse_from(&args);

    if cfg.verbose {
        println!("Running in verbose mode");
    }

    run(cfg);
}
