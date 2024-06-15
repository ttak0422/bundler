#[cfg_attr(test, macro_use)]
extern crate derive_builder;

mod bundle;
mod constant;
mod content;
mod payload;
mod util;

use crate::payload::Payload;
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

    // convert JSON generated in Nix to Rust struct.
    let payload = serde_json::from_str::<Payload>(input_json_text.as_str()).unwrap();

    // resolve the recursive structure of payload.
    let content = content::unpack(payload);

    // generate files for bundler-nvim.
    let bundle = bundle::bundle(&content);
    let export_option = bundle::ExportOption {
        root_dir: output_dir,
    };
    bundle::export(bundle, export_option).unwrap();

    log::info!("bundle completed");
}
