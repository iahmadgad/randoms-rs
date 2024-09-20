use rand::Rng;

#[derive(Debug)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    pub fn random() -> Choice {
        let r = rand::thread_rng().gen_range(0..3);
        match r {
            0 => Choice::Rock,
            1 => Choice::Paper,
            2 => Choice::Scissors,
            _ => unreachable!(),
        }
    }

    pub fn defeats(&self, other: &Choice) -> bool {
        match (self, other) {
            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissors, Choice::Paper) => true,
            _ => false,
        }
    }

    pub fn parse(choice: &str) -> Choice {
        match choice {
            "r" | "rock" => Choice::Rock,
            "p" | "paper" => Choice::Paper,
            "s" | "scissors" => Choice::Scissors,
            x => panic!("{x} is not a valid choice"),
        }
    }
}
