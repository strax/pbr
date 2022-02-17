use std::path::PathBuf;
use clap::Parser;

use openexr;

#[derive(Debug, Eq, PartialEq, Clone, Parser)]
struct Options {
    #[clap(parse(from_os_str))]
    filename: PathBuf
}

fn main() {
    env_logger::init();
    let opts: Options = Options::parse();

    println!("OpenEXR version: {}", openexr::core::version());
    // let ctxt = openexr::core::context::Context::for_read(&opts.filename, &Default::default()).unwrap();
}