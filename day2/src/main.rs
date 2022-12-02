use std::fs;
use std::str::FromStr;


#[derive(Debug, PartialEq, Clone, Copy)]
enum RPC {
    Rock,
    Paper,
    Scissors,
}


impl FromStr for RPC {
    type Err = ();

    fn from_str(input: &str) -> Result<RPC, Self::Err> {
        match input {
            "A"  => Ok(RPC::Rock),
            "B"  => Ok(RPC::Paper),
            "C"  => Ok(RPC::Scissors),
            "X"  => Ok(RPC::Rock),
            "Y"  => Ok(RPC::Paper),
            "Z"  => Ok(RPC::Scissors),
            _      => Err(()),
        }
    }
}
#[derive(Debug, PartialEq)]

enum Outcome {
    Win,
    Lose,
    Draw,
}


impl FromStr for Outcome {
    type Err = ();

    fn from_str(input: &str) -> Result<Outcome, Self::Err> {
        match input {
            "X"  => Ok(Outcome::Lose),
            "Y"  => Ok(Outcome::Draw),
            "Z"  => Ok(Outcome::Win),
            _      => Err(()),
        }
    }
}


fn getScoreTwo(them: RPC, outcome: Outcome) -> u32 {
    let me = match (&them, outcome) {
        (RPC::Rock, Outcome::Win) => RPC::Paper,
        (RPC::Rock, Outcome::Lose) => RPC::Scissors,
        (RPC::Rock, Outcome::Draw) => RPC::Rock,

        (RPC::Paper, Outcome::Win) => RPC::Scissors,
        (RPC::Paper, Outcome::Lose) => RPC::Rock,
        (RPC::Paper, Outcome::Draw) => RPC::Paper,

        (RPC::Scissors, Outcome::Win) => RPC::Rock,
        (RPC::Scissors, Outcome::Lose) => RPC::Paper,
        (RPC::Scissors, Outcome::Draw) => RPC::Scissors,
    };
    return getScore(them, me);
}

fn getScore(them: RPC, me: RPC) -> u32 {
    let score = match (them, &me) {
        (RPC::Rock, RPC::Rock) => 3,
        (RPC::Rock, RPC::Paper) => 6,
        (RPC::Rock, RPC::Scissors) => 0,

        (RPC::Paper, RPC::Rock) => 0,
        (RPC::Paper, RPC::Paper) => 3,
        (RPC::Paper, RPC::Scissors) => 6,

        (RPC::Scissors, RPC::Rock) => 6,
        (RPC::Scissors, RPC::Paper) => 0,
        (RPC::Scissors, RPC::Scissors) => 3,
    };

    let bonus = match me {
        RPC::Rock => 1,
        RPC::Paper => 2,
        RPC::Scissors => 3,
    };

    return score + bonus;
}

fn main() {
    let input = fs::read_to_string("./input").expect("failed to read");
    let mut resultOne = 0;
    let mut resultTwo = 0;
    for line in input.lines() {
        let ugh: Vec<&str> = line.split(' ').collect();
        let me = RPC::from_str(ugh[1]).unwrap();
        let outcome = Outcome::from_str(ugh[1]).unwrap();
        let them = RPC::from_str(ugh[0]).unwrap();
        resultOne =  resultOne + getScore(them, me);
        resultTwo =  resultTwo + getScoreTwo(them, outcome);
    }
    println!("{}", resultOne);
    println!("{}", resultTwo);

}
