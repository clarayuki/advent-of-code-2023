use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let result: i32 = reader
                        .lines()
                        .into_iter()
                        .map(|line| {
                            let line = line.expect("Failed to read line");

                            let first =  line.bytes().find(u8::is_ascii_digit).unwrap() - b'0';
                            let last = line.bytes().rfind(u8::is_ascii_digit).unwrap() - b'0';

                            10 * first as i32 + last as i32
                        })
                        .sum();

    println!("Total calibration value: {}", result);

    Ok(())
}