use std::fs;

enum ResultScore {
    Lose = 0,
    Win = 6,
    Draw = 3,
}

enum ShapeScore {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn get_play_result(play: &(ShapeScore, ShapeScore)) -> ResultScore {
    match play {
        (ShapeScore::Rock, ShapeScore::Rock) => ResultScore::Draw,
        (ShapeScore::Rock, ShapeScore::Paper) => ResultScore::Win,
        (ShapeScore::Rock, ShapeScore::Scissors) => ResultScore::Lose,
        (ShapeScore::Paper, ShapeScore::Rock) => ResultScore::Lose,
        (ShapeScore::Paper, ShapeScore::Paper) => ResultScore::Draw,
        (ShapeScore::Paper, ShapeScore::Scissors) => ResultScore::Win,
        (ShapeScore::Scissors, ShapeScore::Rock) => ResultScore::Win,
        (ShapeScore::Scissors, ShapeScore::Paper) => ResultScore::Lose,
        (ShapeScore::Scissors, ShapeScore::Scissors) => ResultScore::Draw,
    }
}

fn get_play_1(input: &str) -> (ShapeScore, ShapeScore) {
    match input {
        "A X" => (ShapeScore::Rock, ShapeScore::Rock),
        "A Y" => (ShapeScore::Rock, ShapeScore::Paper),
        "A Z" => (ShapeScore::Rock, ShapeScore::Scissors),
        "B X" => (ShapeScore::Paper, ShapeScore::Rock),
        "B Y" => (ShapeScore::Paper, ShapeScore::Paper),
        "B Z" => (ShapeScore::Paper, ShapeScore::Scissors),
        "C X" => (ShapeScore::Scissors, ShapeScore::Rock),
        "C Y" => (ShapeScore::Scissors, ShapeScore::Paper),
        "C Z" => (ShapeScore::Scissors, ShapeScore::Scissors),
        &_ => todo!(),
    }
}

fn get_play_2(input: &str) -> (ShapeScore, ShapeScore) {
    match input {
        "A X" => (ShapeScore::Rock, ShapeScore::Scissors),
        "A Y" => (ShapeScore::Rock, ShapeScore::Rock),
        "A Z" => (ShapeScore::Rock, ShapeScore::Paper),
        "B X" => (ShapeScore::Paper, ShapeScore::Rock),
        "B Y" => (ShapeScore::Paper, ShapeScore::Paper),
        "B Z" => (ShapeScore::Paper, ShapeScore::Scissors),
        "C X" => (ShapeScore::Scissors, ShapeScore::Paper),
        "C Y" => (ShapeScore::Scissors, ShapeScore::Scissors),
        "C Z" => (ShapeScore::Scissors, ShapeScore::Rock),
        &_ => todo!(),
    }
}

fn main() {
    let data = fs::read_to_string("/Users/owchoongseong/personal/adventofcode2022/day2/data.txt")
        .expect("Cannot find data.txt");

    let rounds = data.split("\n");
    let mut score1 = 0;
    let mut score2 = 0;
    for round in rounds {
        let play1 = get_play_1(round);
        let result1 = get_play_result(&play1);
        score1 += (result1 as u32) + (play1.1 as u32);

        let play2 = get_play_2(round);
        let result2 = get_play_result(&play2);
        score2 += (result2 as u32) + (play2.1 as u32);
    }

    println!("Total score is 1: {score1}, 2: {score2}")
}
