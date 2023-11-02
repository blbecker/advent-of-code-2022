use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const POINTS_FOR_ROCK: i32 = 1;
const POINTS_FOR_PAPER: i32 = 2;
const POINTS_FOR_SCISSORS: i32 = 3;

// A --> Their rock
// B --> Their Paper
// C --> Their Scissors
// X --> My rock
// Y --> My Paper
// Z --> My Scissors

enum RPSChoice {
    ROCK,
    PAPER,
    SCISSORS,
}

impl RPSChoice {
    fn to_string(&self) -> &str {
        match self {
            RPSChoice::ROCK => "rock",
            RPSChoice::PAPER => "paper",
            RPSChoice::SCISSORS => "scissors"
        }
    }
}

enum Player {
    ME,
    THEM
}

enum Outcome {
    IWIN,
    THEYWIN,
    TIE,
}

impl Outcome {
    fn to_string(&self) -> &str {
        match self {
            Outcome::IWIN => "I win",
            Outcome::THEYWIN => "they win",
            Outcome::TIE => "it's a tie",
        }
    }

    fn score(&self) -> i32 {
        match self {
            Outcome::IWIN => 6,
            Outcome::TIE => 3,
            Outcome::THEYWIN => 0,
        }
    }
}

fn who_wins(my_move: &RPSChoice, their_move: &RPSChoice) -> Outcome {
    match (my_move, their_move) {
        (RPSChoice::ROCK, RPSChoice::ROCK) => Outcome::TIE,
        (RPSChoice::ROCK, RPSChoice::PAPER) => Outcome::THEYWIN,
        (RPSChoice::ROCK,RPSChoice::SCISSORS) => Outcome::IWIN,
        (RPSChoice::PAPER, RPSChoice::ROCK) => Outcome::IWIN,
        (RPSChoice::PAPER, RPSChoice::PAPER) => Outcome::TIE,
        (RPSChoice::PAPER, RPSChoice::SCISSORS) => Outcome::THEYWIN,
        (RPSChoice::SCISSORS, RPSChoice::ROCK) => Outcome::THEYWIN,
        (RPSChoice::SCISSORS, RPSChoice::PAPER) => Outcome::IWIN,
        (RPSChoice::SCISSORS, RPSChoice::SCISSORS) => Outcome::TIE,
    }
}

fn score_move(my_move: &RPSChoice) -> i32 {
    match my_move {
        RPSChoice::ROCK => POINTS_FOR_ROCK,
        RPSChoice::PAPER => POINTS_FOR_PAPER,
        RPSChoice::SCISSORS => POINTS_FOR_SCISSORS
    }
}

fn parse_move(the_move: &&str) -> RPSChoice {
    if *the_move == "X" || *the_move == "A"{
        return RPSChoice::ROCK;
    } else if *the_move == "Y" || *the_move == "B" {
        return RPSChoice::PAPER;
    } else {
        return RPSChoice::SCISSORS;
    }
}

fn score_turn(my_move: &&str, opponents_move: &&str) -> i32{
    let mut score = 0;
    let my_move = parse_move(my_move);
    let their_move = parse_move(opponents_move);

    let outcome = who_wins(&my_move, &their_move);

    println!("I played {} and they played {} meaning {}.", my_move.to_string(), their_move.to_string(), outcome.to_string());

    return score_move(&my_move) + outcome.score();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Parsing input file: {}", filename);
    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);
    let mut score = 0;

    for (.., line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let lineparts = line.split_whitespace().collect::<Vec<_>>();
        let opponents_move = lineparts.get(0).unwrap();
        let my_move: &&str = lineparts.get(1).unwrap();

        println!("Opponent's move: {}\nMy Move: {}\nScore this turn: {}\n", opponents_move, my_move, score_turn(my_move, opponents_move));
        score += score_turn(my_move, opponents_move);
    }
    println!("Final score: {}", score);
}
