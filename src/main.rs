pub mod file;
use crate::file::*;
pub mod config;
use crate::config::*;

use json::parse;

#[async_std::main]
async fn main() -> tide::Result<()> {

    let config: Option<String> = file_contents(CONFIG_FILE_NAME);
    if config == None {
        panic!("Failed to read file \"{}\"", CONFIG_FILE_NAME);
    }

    let config = parse(&config.unwrap()).unwrap();

    // check syntax
    if ! config["host"].is_string() {
        panic!("config.host must be a string");
    }

    if ! config["listens"].is_array() {
        panic!("config.listens must be an array");
    }

    if config["listens"].len() == 0 {
        panic!("configs.listens cannot be empty");
    }

    for i in 0..config["listens"].len() {
        if ! config["listens"][i].is_object() {
            panic!("config.listens.{} must be an object", i);
        }

        if ! config["listens"][i]["at"].is_string() {
            panic!("config.listens.{}.at must be a string", {i});
        }
        if ! config["listens"][i]["path"].is_string() {
            panic!("config.listens.{}.path must be a string", {i});
        }
    }

    let host: String = config["host"].to_string();

    let mut app = tide::new();
    for i in 0..config["listens"].len() {
        let at: String = config["listens"][i]["at"].to_string();
        let path: String = config["listens"][i]["path"].to_string();

        app.at(&at).serve_file(&path);
        println!("Serving {} at {}", path, at);
    }

    println!("Running at {}", host);
    app.listen(host).await?;

    Ok(())
}
