use std::{fs::File, io::Read};

const SCORE_WIN: i32 = 6;
const SCORE_DRAW: i32 = 3;
const SCORE_LOSE: i32 = 0;

enum Shape {
    Rock,
    Paper,
    Scissors
}

fn to_score(shape: Shape) -> i32{
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn get_enemy_shape(input: &str) -> Shape {
    match input {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!(),
    }
}

fn get_player_shape(input: &str) -> Shape {
    match input {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!(),
    }
}

fn get_score(game: (Shape, Shape)) -> i32 {
    let match_score: i32 = match game {
        (Shape::Rock, Shape::Rock) => SCORE_DRAW,
        (Shape::Rock, Shape::Paper) => SCORE_LOSE,
        (Shape::Rock, Shape::Scissors) => SCORE_WIN,
        (Shape::Paper, Shape::Rock) => SCORE_WIN,
        (Shape::Paper, Shape::Paper) => SCORE_DRAW,
        (Shape::Paper, Shape::Scissors) => SCORE_LOSE,
        (Shape::Scissors, Shape::Rock) => SCORE_LOSE,
        (Shape::Scissors, Shape::Paper) => SCORE_WIN,
        (Shape::Scissors, Shape::Scissors) => SCORE_DRAW,
    };

    match_score + to_score(game.0)
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("./inputs/day2.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut total_score = 0;

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        let enemy_shape = get_enemy_shape(parts.next().unwrap());
        let player_shape = get_player_shape(parts.last().unwrap());
        total_score += get_score((player_shape, enemy_shape));
    }

    println!("Total Score: {}", total_score);

    return Ok(());
}
