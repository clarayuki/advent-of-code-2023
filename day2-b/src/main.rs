use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct Game {
    max_red: i32,
    max_green: i32,
    max_blue: i32
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let total_possible_game_ids: i32 = reader
        .lines()
        .into_iter()
        .map(|line| {
            let line = line.expect("Failed to read line");

            let game = parse(line);

            game.max_red * game.max_green * game.max_blue
        })
        .sum();

    println!("Sum of ids of valid games: {}", total_possible_game_ids);

    Ok(())
}

fn parse(line: String) -> Game {
    let parts: Vec<&str> = line.split(": ").collect();

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    let samples: Vec<&str> = parts[1].split("; ").collect();

    for sample in samples {
        let amount_colors = parse_sample(sample);
        max_red = max_red.max(amount_colors.0);
        max_green = max_green.max(amount_colors.1);
        max_blue = max_blue.max(amount_colors.2);
    }

    Game { max_red: max_red, max_green: max_green, max_blue: max_blue }

}

fn parse_sample(sample: &str) -> (i32, i32, i32) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    let parts: Vec<&str> = sample.split(", ").collect();
    
    for part in parts {
        let count_color: Vec<&str> = part.split_whitespace().collect();
        let count = count_color[0].parse::<i32>().unwrap_or(0);
        let color = count_color[1];

        match color {
            "red" => red = count,
            "green" => green = count,
            "blue" => blue = count,
            _ => {}
        }
    }

    (red, green, blue)
}
