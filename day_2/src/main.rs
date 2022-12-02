use std::{fs::File, io::Read};

const SCORE_WIN: i32 = 6;
const SCORE_DRAW: i32 = 3;
const SCORE_LOSE: i32 = 0;

#[derive(Clone, Copy)]
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

fn get_intended_move(enemy_move: Shape, target_outcome: &str) -> Shape {
    match target_outcome {
        "X" => match enemy_move {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
        "Y" => enemy_move,
        "Z" => match enemy_move {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
        _ => panic!()
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("./inputs/day2.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut total_score_1 = 0;
    let mut total_score_2 = 0;

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        let enemy_shape = get_enemy_shape(parts.next().unwrap());
        
        let player_str = parts.last().unwrap();

        let player_shape_1 = get_player_shape(player_str);
        let player_shape_2 = get_intended_move(enemy_shape, player_str);
        total_score_1 += get_score((player_shape_1, enemy_shape));
        total_score_2 += get_score((player_shape_2, enemy_shape));
    }

    println!("(part 1) Total Score: {}", total_score_1);
    println!("(part 2) Total Score: {}", total_score_2);


    return Ok(());
}
