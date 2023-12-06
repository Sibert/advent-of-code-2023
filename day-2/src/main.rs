use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let red = 12;
    let green = 13;
    let blue = 14;

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut final_value = 0;
    let mut final_power = 0;

    for line in reader.lines() {
        let line = &line?;
        if line.is_empty() {
            continue;
        }

        let game = line.split(':').collect::<Vec<&str>>();
        let game_id: i32 = game[0].split(' ').collect::<Vec<&str>>()[1].parse().unwrap();

        println!("Parsing game {game_id}");

        let reveals = game[1].split(';');

        let mut possible = true;
        let mut min_amounts : Vec<Option<i32>> = vec![None::<i32>, None::<i32>, None::<i32>];

        for reveal in reveals {
            let amounts = get_amount_of_colors(reveal);

            if amounts[0] > red || amounts[1] > green || amounts[2] > blue {
                println!("Game {game_id} is impossible");
                possible = false;
            }

            for i in 0..3 {
                if (min_amounts[i] == None || amounts[i] > min_amounts[i].unwrap()) && amounts[i] != 0 {
                    min_amounts[i] = Some(amounts[i]);
                }
            }
        }

        if possible {
            final_value += game_id;
        }

        let mut power = 1;
        for i in &min_amounts {
            power *= i.unwrap();
        }

        final_power += power;

        println!("Min. cube: red = {:?}, green = {:?}, blue = {:?}", min_amounts[0], min_amounts[1], min_amounts[2]);

    }

    println!("Final value: {final_value}");
    println!("Final power: {final_power}");

    Ok(())
}

fn get_amount_of_colors(s: &str) -> Vec<i32> {
    let parts = s.split(", ");
    let mut colors : Vec<i32> = vec![0, 0, 0];

    for p in parts {
        let mut color_index : usize = 0;

        if p.contains("red") {
            color_index = 0;
        } else if p.contains("green") {
            color_index = 1;
        } else if p.contains("blue") {
            color_index = 2;
        }

        let color_parts = p.trim().split(' ').collect::<Vec<&str>>();
        colors[color_index] = color_parts[0].parse::<i32>().unwrap();
    }

    println!("Red: {}, Green: {}, Blue: {}", colors[0], colors[1], colors[2]);

    colors
}
