use std::file::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("readme.md").unwrap();
    let reader = Bufreader::new(f);

    for line_ in reader.line() {
        let line = line_.unwrap();
        println("{} ({} byte long)", line, line.len());
    }
}
