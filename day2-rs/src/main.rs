use core::str::FromStr;
#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
struct Turn {
    opp: Move,
    hero: Move,
}

impl TryFrom<char> for Move {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err("Error parsing move from char"),
        }
    }
}

impl Turn {
    fn outcome(self) -> Outcome {
        match (self.opp, self.hero) {
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Rock, Move::Paper) => Outcome::Win,
            (Move::Rock, Move::Scissors) => Outcome::Loss,
            (Move::Paper, Move::Rock) => Outcome::Loss,
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Paper, Move::Scissors) => Outcome::Win,
            (Move::Scissors, Move::Rock) => Outcome::Win,
            (Move::Scissors, Move::Paper) => Outcome::Loss,
            (Move::Scissors, Move::Scissors) => Outcome::Draw,
        }
    }

    fn shape_points(self) -> usize {
        match self.hero {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn outcome_points(self) -> usize {
        match self.outcome() {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    fn points(self) -> usize {
        self.outcome_points() + self.shape_points()
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl FromStr for Turn {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(opp), Some(' '), Some(hero), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err("Error parsing turn from line");
        }
        ;
        Ok(Self {
            opp: opp.try_into()?,
            hero: hero.try_into()?,
        })
    }
}

fn main() {
    let input = std::fs::read_to_string("../input/day2.txt").expect("reading input file");
    let lines = input.lines();
    let part1 = lines
        .map(|line| line.parse::<Turn>().unwrap().points())
        .reduce(|a, b| a + b)
        .unwrap();
    println!("{part1:?}");
}
