extern crate path_absolutize;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use path_absolutize::*;

const DATA_FILE_PATH: &str = "./data.json";

fn open_file(path: &str, read: bool) -> File {
    let file_path = Path::new(path).absolutize()
        .expect("Error: file path");

    let file = match read {
        true => OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(file_path)
            .expect("Error: open file for read"),
        false => OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)
            .expect("Error: open file for write"),
    };
    file
}

pub async fn data_save(data: &str) -> bool {
    let mut file = open_file(DATA_FILE_PATH, false);
    file.write_all(data.as_bytes())
        .expect("Error: write to file");
    file.sync_data().unwrap();
    true
}

pub async fn data_load() -> String {
    let mut file = open_file(DATA_FILE_PATH, true);
    let mut res = String::from("");
    file.read_to_string(&mut res)
        .expect("Error: read from file");
    res
}
