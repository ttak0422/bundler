#[cfg_attr(test, macro_use)]
extern crate derive_builder;

mod bundler;
mod constants;
mod lua;
mod payload;
mod unpack;

use crate::bundler::bundle;
use crate::payload::Payload;
use crate::unpack::unpack;
use std::{env, fs};

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("bundle start");

    let args: Vec<String> = env::args().collect();
    let input_json_path = &args[1];
    let output_dir = &args[2];
    log::debug!(
        "input json: {}, output dir: {}",
        input_json_path,
        output_dir
    );

    let input_json_text = fs::read_to_string(input_json_path).unwrap();
    let payload = serde_json::from_str::<Payload>(&input_json_text).unwrap();
    let pack = unpack(&payload);

    bundle(output_dir, &input_json_path, pack).unwrap();

    log::info!("bundle completed");
}
