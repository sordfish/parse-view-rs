use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let f = File::open("/tmp/view").unwrap();
    let mut b = String::new();
    let mut reader = BufReader::new(f);

    loop {
        let data = reader.read_line(&mut b).unwrap();
        if data != 0 {
            println!("data: {}", b);
                b.clear();
            }
        }

}