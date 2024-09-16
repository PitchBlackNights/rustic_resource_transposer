use rtt::cli_parser::Args;
use rtt::prelude::*;
use rtt::setup;
use rtt::resource_converter::converter;
use rustic_texture_transposer as rtt;

fn main() {
    let _args: Args = setup::setup_program();

    debug!("CWD = {:?}", std::env::current_dir().unwrap());
    converter::test();
}
