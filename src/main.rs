use rrt::cli_parser::Args;
use rrt::prelude::*;
use rrt::resource_converter::converter;
use rrt::setup;
use rustic_resource_transposer as rrt;

fn main() {
    let _args: Args = setup::setup_program();

    debug!("CWD = {:?}", std::env::current_dir().unwrap());
    converter::test();
}
