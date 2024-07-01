#[cfg_attr(test, macro_use)]
extern crate derive_builder;

pub mod bundle;
pub mod content;
pub mod payload;

use crate::content::Content;
use crate::payload::Payload;
use std::{env, fs};

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("bundle start");

    let args: Vec<String> = env::args().collect();
    let input_json_path = &args[1];
    let output_dir = &args[2];
    log::info!(
        "input json: {}, output dir: {}",
        input_json_path,
        output_dir
    );

    let input_json_text = fs::read_to_string(input_json_path).unwrap();

    // convert JSON generated in Nix to Rust struct.
    let payload = serde_json::from_str::<Payload>(input_json_text.as_str()).unwrap();

    // convert payload into manageable struct.
    let content = Content::from(payload);

    // export config files.
    bundle::bundle(output_dir, content).unwrap();
}
