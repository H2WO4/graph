use std::fs::File;
use std::io::BufReader;
use std::sync::LazyLock;

use crate::types::Record;

pub static DATA: LazyLock<Box<[Record]>> = LazyLock::new(|| {
    let file = File::open("results.json").expect("file `results.json` should exist");
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).expect("`results.json` contains invalid data")
});
