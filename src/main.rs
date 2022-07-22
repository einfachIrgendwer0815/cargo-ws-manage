use cargo_ws_manage::Config;

use argparse::{ArgumentParser, StoreTrue};

fn main() {
    let mut cfg = Config::default();
    {
        let mut parser = ArgumentParser::new();

        parser.set_description("Cargo workspace management");

        parser.refer(&mut cfg.verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Verbose output");

        parser.parse_args_or_exit();
    }
}
