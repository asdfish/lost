pub mod file;
use crate::file::*;
pub mod config;
use crate::config::*;

use json::parse;

fn main() {

    struct File {
        at: String,
        contents: String,
    }

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
        panic!("config.arrays must be an array");
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

    let mut files: Vec<File> = Vec::with_capacity(config["listens"].len());
    for i in 0..config["listens"].len() {
        let at: String = config["listens"][i]["at"].to_string();
        let path: String = config["listens"][i]["path"].to_string();

        let contents: Option<String> = file_contents(&path);
        if contents == None {
            panic!("Failed to read file \"{}\"", path);
        }
        let contents: String = contents.unwrap();

        let file: File = File {
            at: at,
            contents: contents,
        };

        files.push(file);
    }
}
