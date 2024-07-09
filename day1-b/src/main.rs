use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const DIGITS: [&[u8]; 9] =
    [b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine"];

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let result: i32 = reader
                        .lines()
                        .into_iter()
                        .map(|line| {
                            let line = line.expect("Failed to read line");

                            let mut bytes = line.as_bytes();

                            let first = 'first: loop {
                                if bytes[0].is_ascii_digit() {
                                    break (bytes[0] - b'0') as usize
                                }

                                for (index, digit) in DIGITS.iter().enumerate() {
                                    if bytes.starts_with(digit) {
                                        break 'first index + 1
                                    }
                                }
                                bytes = &bytes[1..];
                            };

                            let last = 'last: loop {
                                if bytes[bytes.len() - 1].is_ascii_digit() {
                                    break (bytes[bytes.len() - 1] - b'0') as usize
                                }

                                for (index, digit) in DIGITS.iter().enumerate() {
                                    if bytes.ends_with(digit) {
                                        break 'last index + 1
                                    }
                                }
                                bytes = &bytes[..bytes.len() - 1];
                            };

                            10 * first as i32 + last as i32

                        })
                        .sum();

    println!("Total calibration value: {}", result);

    Ok(())
}